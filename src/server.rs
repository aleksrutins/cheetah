use std::{env, error::Error, fs, path::Path, time::SystemTime};

use indicatif::ProgressBar;
use notify::{Event, RecursiveMode, Watcher};

use crate::{
    compile_template, compile_templates_recursive,
    config::{Settings, SETTINGS},
    copy_assets_recursive,
    template::TemplateLoader,
};

fn compile_all(loader: &TemplateLoader, progress: &ProgressBar) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("_build/pages/_scripts")?;
    compile_templates_recursive("pages".to_string(), loader, progress)?;

    fs::write(
        "_build/pages/_scripts/component.js",
        include_str!("component.js"),
    )?;

    copy_assets_recursive("assets".to_string(), progress)?;

    Ok(())
}

pub async fn run(progress: ProgressBar) -> Result<(), Box<dyn Error>> {
    let start_time = SystemTime::now();
    let loader = TemplateLoader::default();

    compile_all(&loader, &progress)?;

    progress.finish_with_message(format!(
        "Initial build finished in \x1b[1m{}ms\x1b[0m",
        SystemTime::now().duration_since(start_time)?.as_millis()
    ));
    println!("Starting watcher.");

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| match res {
        Ok(event) => {
            // if event.kind != EventKind::Access(AccessKind::Close(notify::event::AccessMode::Write)) { return }
            for path in event.paths {
                if path.exists() {
                    let recompile_start = SystemTime::now();

                    let relative_path =
                        pathdiff::diff_paths(&path, env::current_dir().unwrap()).unwrap();
                    if relative_path.starts_with("_build/") || relative_path.starts_with(".git/") {
                        continue;
                    }
                    if relative_path == Path::new("hyena.toml") {
                        let mut settings = SETTINGS.lock().unwrap();
                        *settings = Settings::new().unwrap();
                        drop(settings);
                        compile_all(&loader, &progress)
                            .map_err(|e| {
                                println!("Error in compilation: {:?}", e);
                            })
                            .ok();
                    }
                    if relative_path.starts_with("layouts/")
                        || relative_path.starts_with("components/")
                    {
                        compile_templates_recursive("pages".to_string(), &loader, &progress)
                            .map_err(|e| {
                                println!("Error in compilation: {:?}", e);
                            })
                            .ok();
                    } else if relative_path.starts_with("pages/") {
                        compile_template(relative_path.to_path_buf(), &loader, &progress)
                            .map_err(|e| {
                                println!("Error in compilation: {:?}", e);
                            })
                            .ok();
                    } else if relative_path.starts_with("assets/") {
                        progress.set_message(format!(
                            "Copying asset \x1b[1m{}\x1b[0m",
                            relative_path.to_string_lossy()
                        ));
                        fs::create_dir_all(format!(
                            "_build/pages/{}",
                            relative_path.parent().unwrap().to_string_lossy()
                        ))
                        .unwrap();
                        fs::copy(
                            &relative_path,
                            format!("_build/pages/{}", relative_path.to_string_lossy()),
                        )
                        .unwrap();
                    }
                    progress.finish_with_message(format!(
                        "Rebuild finished in \x1b[1m{}ms\x1b[0m",
                        SystemTime::now()
                            .duration_since(recompile_start)
                            .unwrap()
                            .as_millis()
                    ));
                }
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    println!("Starting server on port 3000.");

    let fileserver = warp::filters::fs::dir("_build/pages");

    warp::serve(fileserver).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
