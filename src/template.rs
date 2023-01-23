use html5ever::{local_name, ns, QualName};
use kuchiki::{traits::*, Attribute, ExpandedName, NodeData, NodeRef};
use regex::{Captures, Regex};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    error::Error,
    fs,
    ops::DerefMut,
    sync::Arc,
};

use crate::markdown;

#[derive(Clone, Debug)]
pub struct Template {
    pub dom: NodeRef,
    pub is_document: bool,
    pub extends: Option<NodeRef>,
    pub html_str: String,
}

#[derive(Clone, Debug)]
pub struct ElementRegistrar {
    pub name: String,
    pub template: Template,
    pub connected_scripts: Vec<String>,
}

pub type Scripts = Arc<RefCell<HashMap<String, Arc<RefCell<ElementRegistrar>>>>>;

#[derive(Clone)]
pub struct TemplateContext {
    pub loader: TemplateLoader,
    pub contents: Option<Vec<NodeRef>>,
    pub component_name: Option<String>,
    pub attrs: BTreeMap<ExpandedName, Attribute>,
    pub scripts: Scripts,
}

impl Template {
    fn root(&self) -> NodeRef {
        self.dom
            .select("template")
            .unwrap()
            .next()
            .unwrap_or_else(|| self.dom.select(":root").unwrap().next().unwrap())
            .as_node()
            .to_owned()
    }
    pub fn from_html(html_str: String) -> Result<Self, Box<dyn Error>> {
        let is_document = html_str.contains("<!DOCTYPE HTML>")
        || html_str.contains("<!DOCTYPE html>")
        || html_str.contains("<!doctype HTML>") // I don't know why anybody would _ever_ do this, but you never know...
        || html_str.contains("<!doctype html>");
        let dom = if is_document {
            kuchiki::parse_html()
        } else {
            kuchiki::parse_fragment(
                QualName::new(None, ns!(html), local_name!("template")),
                vec![],
            )
        }
        .one(html_str.clone());

        let extends = dom
            .select("extends[template]:first-child")
            .unwrap()
            .next()
            .map(|n| n.as_node().to_owned());

        if let Some(node) = extends.as_ref() {
            node.detach();
        }

        Ok(Self {
            extends,
            dom,
            is_document,
            html_str,
        })
    }
    pub fn from_markdown(markdown_in: String) -> Result<Self, Box<dyn Error>> {
        Self::from_html(markdown::transform(markdown_in))
    }

    fn expand_tree_recursive(
        &self,
        mut root: &mut NodeRef,
        scripts_ref: &Scripts,
        registrar: Option<Arc<RefCell<ElementRegistrar>>>,
        ctx: &TemplateContext,
    ) -> Result<(), Box<dyn Error>> {
        let scripts_ref_cloned = scripts_ref.clone();
        let bind_regex = Regex::new(r"\{\{(?P<var>.*?)\}\}").unwrap();
        let node = root.deref_mut();

        for mut child in node.children() {
            let registrar = if let NodeData::Element(el) = child.data() {
                let mut scripts = scripts_ref.borrow_mut();
                if el.name.ns == ns!(html)
                    && el.name.local == local_name!("script")
                    && ctx.component_name.is_some()
                {
                    if let Some(name) = &ctx.component_name {
                        let script_name = format!("{}.registrar.js", name);
                        if scripts.get(&script_name).is_none() {
                            scripts.insert(
                                script_name.clone(),
                                Arc::new(RefCell::new(ElementRegistrar {
                                    name: name.to_string(),
                                    connected_scripts: vec![],
                                    template: self.clone(),
                                })),
                            );
                        }
                        let script = scripts.get(&script_name).unwrap().clone();
                        drop(scripts);
                        Some(script)
                    } else {
                        drop(scripts);
                        registrar.clone()
                    }
                } else {
                    drop(scripts);
                    registrar.clone()
                }
            } else {
                registrar.clone()
            };
            self.expand_tree_recursive(&mut child, &scripts_ref, registrar, ctx)?;
        }

        match node.data() {
            NodeData::Element(el) => {
                let mut attrs = el.attributes.borrow_mut();
                for (name, value) in attrs.map.clone() {
                    if name.local.starts_with('[') && name.local.ends_with(']') {
                        attrs.map.insert(
                            ExpandedName::new(
                                "",
                                name.local
                                    .clone()
                                    .strip_prefix('[')
                                    .unwrap()
                                    .strip_suffix(']')
                                    .unwrap(),
                            ),
                            Attribute {
                                prefix: None,
                                value: ctx
                                    .attrs
                                    .get(&ExpandedName::new("", value.value))
                                    .map(|attr| attr.value.clone())
                                    .unwrap_or_else(|| "".to_string()),
                            },
                        );
                    }
                }
                drop(attrs);
                if el.name.local.to_string().contains('-') {
                    let (rendered_contents, new_scripts) = ctx
                        .loader
                        .load(&("components/".to_string() + &el.name.local.to_string() + ".html"))?
                        .render(&TemplateContext {
                            loader: ctx.loader.clone(),
                            contents: Some(node.children().collect()),
                            attrs: el.attributes.borrow().map.clone(),
                            component_name: Some(el.name.local.to_string()),
                            scripts: scripts_ref.clone(),
                        })?;
                    let mut scripts = scripts_ref_cloned.borrow_mut();
                    for (name, contents) in new_scripts {
                        scripts.insert(name.to_string(), Arc::new(RefCell::new(contents)));
                    }
                    let mut attrs = HashMap::new();
                    attrs.insert(
                        ExpandedName::new("", "shadowroot"),
                        Attribute {
                            prefix: None,
                            value: "open".to_string(),
                        },
                    );
                    let shadow_root = NodeRef::new_element(
                        QualName::new(None, ns!(html), local_name!("template")),
                        attrs,
                    );
                    shadow_root.append(rendered_contents);
                    for child in node.children() {
                        child.detach();
                    }
                    node.append(shadow_root);
                } else if el.name.ns == ns!(html)
                    && el.name.local == *"slot"
                    && ctx.component_name == None
                {
                    if let Some(contents) = &ctx.contents {
                        for elem in contents {
                            node.append(elem.clone());
                        }
                    }
                }
            }
            NodeData::Text(text_ref) => {
                let mut text = text_ref.borrow_mut();
                *text = bind_regex
                    .replace_all(&*text, |caps: &Captures| {
                        if let Some(name) = caps.get(1) {
                            ctx.attrs
                                .get(&ExpandedName::new("", name.as_str()))
                                .map(|attr| attr.value.clone())
                                .unwrap_or_else(|| "".to_string())
                        } else {
                            "".to_string()
                        }
                    })
                    .to_string();

                if let Some(registrar) = registrar {
                    registrar
                        .borrow_mut()
                        .connected_scripts
                        .push(text.to_string());
                    node.detach();
                };
            }
            _ => (),
        };
        Ok(())
    }

    fn render_basic(
        &self,
        ctx: &TemplateContext,
    ) -> Result<(NodeRef, HashMap<String, ElementRegistrar>), Box<dyn Error>> {
        let mut root = self.root();
        let scripts_ref = ctx.scripts.clone();
        self.expand_tree_recursive(&mut root, &scripts_ref, None, ctx)?;
        let scripts = scripts_ref.borrow();
        if let Ok(body) = root.select_first("body") {
            for name in scripts.keys() {
                let mut attrs = HashMap::new();
                attrs.insert(
                    ExpandedName::new("", "src"),
                    Attribute {
                        prefix: None,
                        value: format!("/_scripts/{}", name),
                    },
                );
                attrs.insert(
                    ExpandedName::new("", "type"),
                    Attribute { prefix: None, value: "module".into() }
                );
                let node = NodeRef::new_element(
                    QualName::new(None, ns!(html), local_name!("script")),
                    attrs,
                );
                body.as_node().append(node);
            }
        }
        let mut result_scripts = HashMap::new();

        for (name, contents) in scripts.iter() {
            result_scripts.insert(name.to_string(), contents.as_ref().borrow().clone());
        }
        Ok((root, result_scripts))
    }

    pub fn render(
        &self,
        ctx: &TemplateContext,
    ) -> Result<(NodeRef, HashMap<String, ElementRegistrar>), Box<dyn Error>> {
        match &self.extends {
            Some(tmpl) => {
                let attrs = tmpl.as_element().unwrap().attributes.borrow();
                let (contents, scripts) = self.render_basic(ctx)?;
                let new_scripts = Arc::new(RefCell::new(HashMap::new()));
                for (name, contents) in ctx.scripts.take() {
                    new_scripts.borrow_mut().insert(name, contents);
                }
                for (name, contents) in scripts {
                    new_scripts.borrow_mut().insert(name, Arc::new(RefCell::new(contents)));
                }
                (&ctx.loader)
                    .load(&attrs.get("template").unwrap().to_string())?
                    .render(&TemplateContext {
                        loader: ctx.loader.clone(),
                        contents: Some(vec![contents]),
                        attrs: attrs.map.clone(),
                        scripts: new_scripts,
                        component_name: None,
                    })
            }
            None => self.render_basic(ctx),
        }
    }
    pub fn render_to_string(
        &self,
        ctx: &TemplateContext,
    ) -> Result<(String, HashMap<String, ElementRegistrar>), Box<dyn Error>> {
        let render_result = self.render(ctx)?;
        Ok((render_result.0.to_string(), render_result.1))
    }
}

#[derive(Clone)]
pub struct TemplateLoader {
    pub root: String,
}

impl TemplateLoader {
    pub fn resolve(&self, path: &String) -> String {
        format!("{}/{}", self.root, path)
    }
    pub fn load(&self, name: &String) -> Result<Template, Box<dyn Error>> {
        let contents = fs::read_to_string(self.resolve(name))?;
        Ok(
            match name
                .split('.')
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap_or(&"html")
            {
                &"md" => Template::from_markdown(contents)?,
                _ => Template::from_html(contents)?,
            },
        )
    }
}

impl Default for TemplateLoader {
    fn default() -> Self {
        Self {
            root: ".".to_string(),
        }
    }
}
