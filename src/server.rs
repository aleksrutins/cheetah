use std::{fs, error::Error, time::SystemTime, path::Path, env};

use futures::{channel::mpsc::{channel, Receiver}, SinkExt, StreamExt};
use notify::{RecommendedWatcher, Event, Watcher, Config, RecursiveMode, event::AccessKind, EventKind};

use crate::{template::TemplateLoader, compile_templates_recursive, copy_assets_recursive, compile_template};

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    }, Config::default())?;
    
    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(base: P, loader: &TemplateLoader) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(&base.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                if event.kind != EventKind::Access(AccessKind::Close(notify::event::AccessMode::Write)) { continue }
                for path in event.paths {
                    if path.exists() {
                        let relative_path = pathdiff::diff_paths(&path, &base).unwrap();
                        if relative_path.starts_with("_build/") { continue }
                        println!("File changed: {:?}", relative_path);
                        if relative_path.starts_with("layouts/") || relative_path.starts_with("components/") {
                            println!("Layout or component changed; recompiling all");
                            compile_templates_recursive("pages".to_string(), &loader).map_err(|e| {
                                println!("Error in compilation: {:?}", e);
                            }).ok();
                        } else if relative_path.starts_with("pages/") {
                            compile_template(relative_path.to_path_buf(), loader).map_err(|e| {
                                println!("Error in compilation: {:?}", e);
                            }).ok();
                        } else if relative_path.starts_with("assets/") {
                            println!("Copying asset \x1b[1m{}\x1b[0m", relative_path.to_string_lossy().to_string());
                            fs::create_dir_all(format!("_build/pages/{}", relative_path.parent().unwrap().to_string_lossy())).unwrap();
                            fs::copy(
                                &relative_path,
                                format!(
                                    "_build/pages/{}",
                                    relative_path.to_string_lossy().to_string()
                                ),
                            )?;
                        }
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let start_time = SystemTime::now();
    let loader = TemplateLoader::default();

    fs::create_dir_all("_build/pages/_scripts")?;
    compile_templates_recursive("pages".to_string(), &loader)?;

    fs::write(
        "_build/pages/_scripts/component.js",
        include_str!("component.js"),
    )?;

    copy_assets_recursive("assets".to_string())?;
    println!("Initial build finished in \x1b[1m{}ms\x1b[0m", SystemTime::now().duration_since(start_time)?.as_millis());
    println!("Starting watcher.");

    futures::executor::block_on(async {
        async_watch(env::current_dir()?, &loader).await
    })?;


    Ok(())
}