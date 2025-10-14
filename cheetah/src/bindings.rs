use kuchiki::{Attribute, ExpandedName, NodeData};
use kuchikikiki as kuchiki;
use lazy_static::lazy_static;
use locrian::{eval::EvalResult, parser::ExprValue};
use regex::{Captures, Regex};

use crate::template::TemplateContext;

pub struct BindingContext<'a> {
    component_name: Option<String>,
    node: &'a NodeData,
    ctx: &'a TemplateContext,
}

lazy_static! {
    pub static ref BIND_REGEX: Regex = Regex::new(r"[^\!]?\{\{(?P<var>.*?)\}\}").unwrap();
    pub static ref LITERAL_BIND_REGEX: Regex = Regex::new(r"\!\{\{").unwrap();
}

impl<'a> BindingContext<'a> {
    pub fn new(
        component_name: Option<String>,
        node: &'a NodeData,
        ctx: &'a TemplateContext,
    ) -> Self {
        Self {
            component_name,
            node,
            ctx,
        }
    }

    pub fn locrian_ctx(&self) -> locrian::eval::EvalContext<'a> {
        let mut ctx = locrian::stdlib::STDLIB.with_var(
            "$me",
            self.component_name
                .clone()
                .map(ExprValue::String)
                .unwrap_or(ExprValue::Null),
        );
        for (key, value) in &self.ctx.attrs {
            ctx.vars.insert(
                key.local.to_string(),
                ExprValue::String(value.value.clone()),
            );
        }
        ctx
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
                            value: if let Ok(EvalResult::String(s)) =
                                locrian::eval_str(&value.value, self.locrian_ctx())
                            {
                                s
                            } else {
                                "".to_string()
                            },
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
                    if let Some(expr) = caps.get(1)
                        && let Ok(EvalResult::String(s)) =
                            locrian::eval_str(expr.as_str(), self.locrian_ctx())
                    {
                        s
                    } else {
                        "".to_string()
                    }
                })
                .to_string();
            *text = LITERAL_BIND_REGEX.replace_all(&text, "{{").to_string();
        }
    }
}
