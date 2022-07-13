use std::{error::Error, fs};

use html_parser::Dom;

use crate::markdown;

pub struct Template {
    pub top: String,
    pub slot_default: Option<String>,
    pub bottom: Option<String>,
    pub extends: Option<String>
}

impl Template {
    pub fn from_html(html_str: String) -> Result<Self, Box<dyn Error>> {
        let dom = Dom::parse(&html_str)?;
        let contents = html_str.split("<slot>").collect::<Vec<&str>>();
        Ok(Self {
            extends: if dom.children.get(0).map(|node| node.element().map(|el| el.name.clone())) == Some(Some("extend".into())) {
                let extends = dom.children.get(0).map(|node| node.element()).unwrap().unwrap();
                match extends.attributes.get("template") {
                    Some(Some(path)) => Some(path.clone()),
                    _ => None
                }
            } else { None },
            top: contents.get(0).unwrap().to_owned().to_string(),
            slot_default: if contents.len() == 2 {
                let contents_after_open = contents.get(1).unwrap().split("</slot>").collect::<Vec<&str>>();
                Some(contents_after_open.get(0).unwrap().to_owned().to_string())
            } else { None },
            bottom: if contents.len() == 2 {
                let contents_after_open = contents.get(1).unwrap().split("</slot>").collect::<Vec<&str>>();
                if contents_after_open.len() == 2  {
                    Some(contents_after_open.get(1).unwrap().to_owned().to_string())
                } else { None }
            } else { None }
        })
    }
    pub fn from_markdown(markdown_in: String) -> Result<Self, Box<dyn Error>> {
        Self::from_html(markdown::transform(markdown_in))
    }

    fn render_basic(&self, template_loader: TemplateLoader, contents: Option<String>) -> Result<String, Box<dyn Error>> {
        
    }

    pub fn render(&self, template_loader: TemplateLoader, contents: Option<String>) -> Result<String, Box<dyn Error>> {
        match self.extends {
            Some(tmpl) => template_loader.load(tmpl)?.render(template_loader, Some(self.render_basic(template_loader, contents)?)),
            None => self.render_basic(template_loader, contents)
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