use std::{collections::BTreeMap, error::Error, fmt::Display, fs, path::PathBuf};

use template::TemplateLoader;

#[macro_use]
extern crate html5ever;
mod markdown;
mod template;

const BUILD_DIR: &'static str = "_build";

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

fn compile_templates_recursive(dir: String, loader: &TemplateLoader) -> Result<(), Box<dyn Error>> {
    let pages = fs::read_dir(loader.resolve(&dir))?;

    for entry in pages {
        if let Ok(entry) = entry {
            let mut full_path = PathBuf::from(format!(
                "{}/{}",
                dir,
                entry
                    .file_name()
                    .to_str()
                    .ok_or(CompileError::NotAFileNameError)?
            ));
            println!("{}", full_path.to_string_lossy());
            if entry.file_type()?.is_dir() {
                let out_path = format!("{}/{}", BUILD_DIR, full_path.to_string_lossy().to_string());
                fs::create_dir_all(out_path)?;
                compile_templates_recursive(full_path.to_string_lossy().to_string(), loader)?;
            } else {
                let template = loader.load(&full_path.to_string_lossy().to_string())?;
                full_path.set_extension("html");
                let out_path = format!("{}/{}", BUILD_DIR, full_path.to_string_lossy().to_string());
                fs::write(
                    out_path,
                    template.render_to_string(&template::TemplateContext {
                        loader: loader.clone(),
                        contents: None,
                        attrs: BTreeMap::new(),
                    })?,
                )?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let loader = TemplateLoader::default();

    compile_templates_recursive("pages".to_string(), &loader)?;

    Ok(())
}
