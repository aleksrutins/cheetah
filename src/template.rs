use std::{error::Error, fs, collections::HashMap};

use regex::Regex;

use crate::markdown;

pub struct Template<'a> {
    pub root: &'a Node,
    pub extends: Option<String>
}

pub struct TemplateContext {
    pub loader: TemplateLoader,
    pub contents: Option<String>,
    pub attrs: HashMap<String, Option<String>>
}

impl<'a> Template<'a> {
    pub fn from_html(html_str: String) -> Result<Self, Box<dyn Error>> {
        let dom = Dom::parse(&html_str)?;
        let contents = html_str.split("<slot>").collect::<Vec<&str>>();
        let extends = if dom.children.get(0).map(|node| node.element().map(|el| el.name.clone())) == Some(Some("extend".into())) {
            let extends = dom.children.get(0).map(|node| node.element()).unwrap().unwrap();
            match extends.attributes.get("template") {
                Some(Some(path)) => Some(path.clone()),
                _ => None
            }
        } else { None };
        Ok(Self {
            extends,
            root: dom.children.iter().find(|node| node.element().map(|el| el.name) == Some("template".to_string())).expect("Template has no root; please add a `<template>` element.")
        })
    }
    pub fn from_markdown(markdown_in: String) -> Result<Self, Box<dyn Error>> {
        Self::from_html(markdown::transform(markdown_in))
    }

    fn expand_tree_recursive(&self, tree: &mut Node, ctx: TemplateContext) {
        let bind_regex = Regex::new(r"\{\{(?P<var>.*?)\}\}").unwrap();

    }

    fn render_basic(&self, ctx: TemplateContext) -> Result<String, Box<dyn Error>> {
        let mut root = self.root.clone();
        self.expand_tree_recursive(&mut root, ctx);
        Ok(root.)
    }

    pub fn render(&self, ctx: TemplateContext) -> Result<String, Box<dyn Error>> {
        match self.extends {
            Some(tmpl) => ctx.loader.load(tmpl)?.render(TemplateContext { loader: ctx.loader, contents: Some(self.render_basic(ctx)?), attrs: HashMap::new() }),
            None => self.render_basic(ctx)
        }
    }
}

pub struct TemplateLoader {
    pub root: String,
}

impl TemplateLoader {
    pub fn load(&self, name: String) -> Result<Template, Box<dyn Error>> {
        let contents = fs::read_to_string(format!("{}/{}", self.root, name))?;
        Ok(match name.split(".").collect::<Vec<&str>>().get(1).unwrap_or(&"html") {
            &"md" => Template::from_markdown(contents)?,
            _ => Template::from_html(contents)?
        })
    }
}
