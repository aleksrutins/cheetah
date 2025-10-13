use notify::EventKind;
use notify::event::AccessKind;
use std::{env, error::Error, fs, path::Path, time::SystemTime};

use indicatif::ProgressBar;
use notify::{Event, RecursiveMode, Watcher};

use crate::{
    compile_template, compile_templates_recursive,
    config::{SETTINGS, Settings},
    copy_assets_recursive, hooks,
    template::TemplateLoader,
};

async fn compile_all(
    loader: &TemplateLoader,
    progress: &ProgressBar,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("_build/pages/_scripts")?;

    hooks::run_all(progress, true, None, hooks::During::PreBuild).await?;

    compile_templates_recursive("pages".to_string(), loader, progress)?;

    fs::write(
        "_build/pages/_scripts/component.js",
        include_str!("component.js"),
    )?;

    copy_assets_recursive("assets".to_string(), progress)?;

    hooks::run_all(progress, true, None, hooks::During::PostBuild).await?;

    Ok(())
}

pub async fn run(progress: ProgressBar) -> Result<(), Box<dyn Error>> {
    let start_time = SystemTime::now();
    let loader = TemplateLoader::default();

    compile_all(&loader, &progress).await?;

    progress.finish_with_message(format!(
        "Initial build finished in \x1b[1m{}ms\x1b[0m",
        SystemTime::now().duration_since(start_time)?.as_millis()
    ));
    println!("Starting watcher.");

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| match res {
        Ok(event) => {
            if event.kind != EventKind::Access(AccessKind::Close(notify::event::AccessMode::Write))
            {
                return;
            }
            for path in event.paths {
                if path.exists() {
                    let recompile_start = SystemTime::now();

                    let relative_path =
                        pathdiff::diff_paths(&path, env::current_dir().unwrap()).unwrap();

                    if relative_path.starts_with("_build/") || relative_path.starts_with(".git/") {
                        continue;
                    }

                    let rto = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .map_err(|e| {
                            println!("Error in compilation: {e:?}");
                        })
                        .ok();

                    if rto.is_none() {
                        return;
                    }

                    let rt = rto.unwrap();
                    rt.block_on(hooks::run_all(
                        &progress,
                        true,
                        Some(&relative_path),
                        hooks::During::PreBuild,
                    ))
                    .map_err(|e| {
                        println!("Error in compilation: {e:?}");
                    })
                    .ok();

                    if relative_path == Path::new("cheetah.toml") {
                        let mut settings = SETTINGS.lock().unwrap();
                        *settings = Settings::new().unwrap();
                        drop(settings);
                        rt.block_on(compile_all(&loader, &progress))
                            .map_err(|e| {
                                println!("Error in compilation: {e:?}");
                            })
                            .ok();
                    }
                    if relative_path.starts_with("layouts/")
                        || relative_path.starts_with("components/")
                    {
                        compile_templates_recursive("pages".to_string(), &loader, &progress)
                            .map_err(|e| {
                                println!("Error in compilation: {e:?}");
                            })
                            .ok();
                    } else if relative_path.starts_with("pages/") {
                        compile_template(relative_path.to_path_buf(), &loader, &progress)
                            .map_err(|e| {
                                println!("Error in compilation: {e:?}");
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

                    rt.block_on(hooks::run_all(
                        &progress,
                        true,
                        Some(&relative_path),
                        hooks::During::PostBuild,
                    ))
                    .map_err(|e| {
                        println!("Error in compilation: {e:?}");
                    })
                    .ok();

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
        Err(e) => println!("watch error: {e:?}"),
    })?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    println!("Starting server on port 3000.");

    let fileserver = warp::filters::fs::dir("_build/pages");

    warp::serve(fileserver).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
