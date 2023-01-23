use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fmt::Display,
    fs,
    path::PathBuf, cell::RefCell, sync::Arc, time::SystemTime,
};

use template::TemplateLoader;

#[macro_use]
extern crate html5ever;
mod markdown;
mod template;

const BUILD_DIR: &str = "_build";

#[derive(Debug)]
enum CompileError {
    NotAFileNameError,
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::NotAFileNameError => write!(f, "Not a file name"),
        }
    }
}

fn copy_assets_recursive(dir: String) -> Result<(), Box<dyn Error>> {
    if let Ok(assets) = fs::read_dir("assets") {
        for asset in assets.flatten() {
            if asset.file_type()?.is_dir() {
                let full_path = format!("{}/{}", dir, asset.file_name().to_string_lossy());
                fs::create_dir_all(&full_path)?;
                copy_assets_recursive(full_path)?;
            } else {
                fs::copy(
                    asset.path(),
                    format!(
                        "_build/pages/{}/{}",
                        dir,
                        asset.file_name().to_string_lossy()
                    ),
                )?;
            }
        }
    }

    Ok(())
}

fn compile_templates_recursive(dir: String, loader: &TemplateLoader) -> Result<(), Box<dyn Error>> {
    if let Ok(pages) = fs::read_dir(loader.resolve(&dir)) {
        for entry in pages.flatten() {
            let mut full_path = PathBuf::from(format!(
                "{}/{}",
                dir,
                entry
                    .file_name()
                    .to_str()
                    .ok_or(CompileError::NotAFileNameError)?
            ));
            if entry.file_type()?.is_dir() {
                let out_path = format!("{}/{}", BUILD_DIR, full_path.to_string_lossy());
                fs::create_dir_all(out_path)?;
                compile_templates_recursive(full_path.to_string_lossy().to_string(), loader)?;
            } else {
                let template = loader.load(&full_path.to_string_lossy().to_string())?;
                println!("Building page \x1b[1m{}\x1b[0m", full_path.to_string_lossy());
                full_path.set_extension("html");
                let out_path = format!("{}/{}", BUILD_DIR, full_path.to_string_lossy());
                let result = template.render_to_string(&template::TemplateContext {
                    loader: loader.clone(),
                    contents: None,
                    component_name: None,
                    attrs: BTreeMap::new(),
                    scripts: Arc::new(RefCell::new(HashMap::new())),
                })?;
                fs::write(out_path, result.0)?;
                for (script_name, registrar) in result.1 {
                    let contents = format!(
                        "
import {{ registerComponent }} from './component.js';

registerComponent(`{}`, `{}`, [{}]);
                    ",
                        registrar.name,
                        registrar.template.html_str,
                        registrar
                            .connected_scripts
                            .iter()
                            .map(|script| format!("async function() {{{}}}", script))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    fs::write(format!("_build/pages/_scripts/{}", script_name), contents)?;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = SystemTime::now();
    let loader = TemplateLoader::default();

    fs::create_dir_all("_build/pages/_scripts")?;
    compile_templates_recursive("pages".to_string(), &loader)?;

    fs::write(
        "_build/pages/_scripts/component.js",
        include_str!("component.js"),
    )?;

    copy_assets_recursive("assets".to_string())?;

    println!("\x1b[1mFinished in {}ms\x1b[0m", SystemTime::now().duration_since(start).unwrap().as_millis());
    Ok(())
}
