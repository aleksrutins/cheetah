use kuchiki::{Attribute, ExpandedName, NodeData};
use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::template::TemplateContext;

pub struct BindingContext<'a> {
    node: &'a NodeData,
    ctx: &'a TemplateContext,
}

lazy_static! {
    pub static ref BIND_REGEX: Regex = Regex::new(r"[^\!]?\{\{(?P<var>.*?)\}\}").unwrap();
    pub static ref LITERAL_BIND_REGEX: Regex = Regex::new(r"\!\{\{").unwrap();
}

impl<'a> BindingContext<'a> {
    pub fn new(node: &'a NodeData, ctx: &'a TemplateContext) -> Self {
        Self { node, ctx }
    }

    pub fn expand_attributes(&self) {
        if let NodeData::Element(element) = self.node {
            let mut attrs = element.attributes.borrow_mut();
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
                            value: self
                                .ctx
                                .attrs
                                .get(&ExpandedName::new("", value.value))
                                .map(|attr| attr.value.clone())
                                .unwrap_or_default(),
                        },
                    );
                }
            }
        }
    }

    pub fn expand_text(&self) {
        if let NodeData::Text(text_ref) = self.node {
            let mut text = text_ref.borrow_mut();
            *text = BIND_REGEX
                .replace_all(&text, |caps: &Captures| {
                    if let Some(name) = caps.get(1) {
                        self.ctx
                            .attrs
                            .get(&ExpandedName::new("", name.as_str()))
                            .map(|attr| attr.value.clone())
                            .unwrap_or_default()
                    } else {
                        "".to_string()
                    }
                })
                .to_string();
            *text = LITERAL_BIND_REGEX.replace_all(&text, "{{").to_string();
        }
    }
}
