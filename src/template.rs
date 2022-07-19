use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fs,
    ops::DerefMut,
};

use html5ever::{local_name, ns, QualName};
use kuchiki::{traits::*, Attribute, ExpandedName, NodeData, NodeRef};
use regex::{Captures, Regex};

use crate::markdown;

pub struct Template {
    pub dom: NodeRef,
    pub extends: Option<String>,
}

#[derive(Clone)]
pub struct TemplateContext {
    pub loader: TemplateLoader,
    pub contents: Option<Vec<NodeRef>>,
    pub attrs: BTreeMap<ExpandedName, Attribute>,
}

impl Template {
    fn root(&self) -> NodeRef {
        self.dom
            .select(":root > template")
            .unwrap()
            .next()
            .unwrap_or_else(|| self.dom.select(":root").unwrap().next().unwrap())
            .as_node()
            .to_owned()
    }
    pub fn from_html(html_str: String) -> Result<Self, Box<dyn Error>> {
        let dom = kuchiki::parse_html().one(html_str);
        let extends = dom
            .select("extends[template]:first-child")
            .unwrap()
            .next()
            .map(|n| n.as_node().to_owned())
            .map(|n| n.as_element().unwrap().to_owned())
            .map(|el| el.attributes.borrow().get("template").unwrap().to_owned())
            .map(|s| s.to_string());

        Ok(Self { extends, dom })
    }
    pub fn from_markdown(markdown_in: String) -> Result<Self, Box<dyn Error>> {
        Self::from_html(markdown::transform(markdown_in))
    }

    fn expand_tree_recursive(
        &self,
        mut root: &mut NodeRef,
        ctx: &TemplateContext,
    ) -> Result<(), Box<dyn Error>> {
        let bind_regex = Regex::new(r"\{\{(?P<var>.*?)\}\}").unwrap();
        let node = root.deref_mut();

        for mut child in node.children() {
            self.expand_tree_recursive(&mut child, ctx)?;
        }

        match node.data() {
            NodeData::Element(el) => {
                for (name, value) in el.attributes.borrow().map.clone() {
                    if name.ns.to_string() == "bind".to_string() {
                        el.attributes.borrow_mut().map.insert(
                            ExpandedName::new("", name.local.clone()),
                            Attribute {
                                prefix: None,
                                value: ctx
                                    .attrs
                                    .get(&ExpandedName::new("", value.value))
                                    .map(|attr| attr.value.clone())
                                    .unwrap_or("".to_string()),
                            },
                        );
                    }
                }
                if el.name.ns == "x".to_string() {
                    let rendered_contents =
                        ctx.loader
                            .load(&el.name.local.to_string())?
                            .render(&TemplateContext {
                                loader: ctx.loader.clone(),
                                contents: Some(node.children().clone().collect()),
                                attrs: el.attributes.borrow().map.clone(),
                            })?;
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
                    node.append(
                        shadow_root
                            .as_element()
                            .unwrap()
                            .template_contents
                            .as_ref()
                            .unwrap()
                            .clone(),
                    );
                } else if el.name.ns == ns!(html) && el.name.local == "slot".to_string() {
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
                                .unwrap_or("".to_string())
                        } else {
                            "".to_string()
                        }
                    })
                    .to_string();
            }
            _ => (),
        };
        Ok(())
    }

    fn render_basic(&self, ctx: &TemplateContext) -> Result<NodeRef, Box<dyn Error>> {
        let mut root = self.root().clone();
        self.expand_tree_recursive(&mut root, ctx)?;
        Ok(root)
    }

    pub fn render(&self, ctx: &TemplateContext) -> Result<NodeRef, Box<dyn Error>> {
        match &self.extends {
            Some(tmpl) => (&ctx.loader).load(tmpl)?.render(&TemplateContext {
                loader: ctx.loader.clone(),
                contents: Some(vec![self.render_basic(ctx)?]),
                attrs: BTreeMap::new(),
            }),
            None => self.render_basic(ctx),
        }
    }
    pub fn render_to_string(&self, ctx: &TemplateContext) -> Result<String, Box<dyn Error>> {
        Ok(self.render(ctx)?.to_string())
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
                .split(".")
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
