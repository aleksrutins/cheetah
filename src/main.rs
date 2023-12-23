use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    env,
    error::Error,
    fmt::Display,
    fs,
    path::PathBuf,
    sync::Arc,
    time::SystemTime,
};

use template::TemplateLoader;

#[macro_use]
extern crate html5ever;
mod markdown;
mod server;
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
    if let Ok(assets) = fs::read_dir(dir.clone()) {
        fs::create_dir_all("_build/pages/assets")?;
        for asset in assets.flatten() {
            if asset.file_type()?.is_dir() {
                let full_path = format!("{}/{}", dir, asset.file_name().to_string_lossy());
                fs::create_dir_all(format!("_build/pages/{}", full_path))?;
                copy_assets_recursive(full_path)?;
            } else {
                println!(
                    "Copying asset \x1b[1m{}\x1b[0m",
                    asset.path().to_string_lossy()
                );
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

fn compile_template(path: PathBuf, loader: &TemplateLoader) -> Result<(), Box<dyn Error>> {
    let template = loader.load(&path.to_string_lossy().to_string())?;
    println!("Building page \x1b[1m{}\x1b[0m", path.to_string_lossy());
    let mut html_path = path;
    html_path.set_extension("html");
    let out_path = format!("{}/{}", BUILD_DIR, html_path.to_string_lossy());
    let result = template.render_to_string(&template::TemplateContext {
        loader: loader.clone(),
        contents: None,
        component_name: None,
        attrs: BTreeMap::new(),
        scripts: Arc::new(RefCell::new(HashMap::new())),
    })?;
    fs::write(out_path, format!("<!doctype html>{}", result.0))?;
    for (script_name, registrar) in result.1 {
        let mut scripts = registrar.connected_scripts;
        scripts.sort();
        scripts.dedup();
        let contents = format!(
            "
import {{ registerComponent }} from './component.js';

registerComponent(`{}`, `{}`, [{}]);
        ",
            registrar.name,
            registrar
                .template
                .html_str
                .replace('`', "\\`")
                .replace("${", "\\${"),
            scripts
                .iter()
                .map(|script| format!("async function() {{{}}}", script))
                .collect::<Vec<_>>()
                .join(", ")
        );
        fs::write(format!("_build/pages/_scripts/{}", script_name), contents)?;
    }
    Ok(())
}

fn compile_templates_recursive(dir: String, loader: &TemplateLoader) -> Result<(), Box<dyn Error>> {
    if let Ok(pages) = fs::read_dir(loader.resolve(&dir)) {
        for entry in pages.flatten() {
            let full_path = PathBuf::from(format!(
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
                compile_template(full_path, loader)?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == *"dev" {
        server::run().await
    } else {
        let start = SystemTime::now();
        let loader = TemplateLoader::default();

        fs::create_dir_all("_build/pages/_scripts")?;
        compile_templates_recursive("pages".to_string(), &loader)?;

        fs::write(
            "_build/pages/_scripts/component.js",
            include_str!("component.js"),
        )?;

        copy_assets_recursive("assets".to_string())?;

        println!(
            "\x1b[1mFinished in {}ms\x1b[0m",
            SystemTime::now().duration_since(start).unwrap().as_millis()
        );

        Ok(())
    }
}
