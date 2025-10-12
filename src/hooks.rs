use std::{collections::HashMap, error::Error, path::PathBuf};

use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use wax::{Glob, Pattern};

use crate::config::SETTINGS;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum During {
    PreBuild,
    PostBuild,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Dev {
    Watch(Vec<String>),
    Disabled,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hook {
    name: String,
    during: During,
    dev: Dev,
    command: String,
}

pub async fn run_hook(hook: &Hook, progress: &ProgressBar) -> Result<i32, Box<dyn Error>> {
    progress.set_message(format!("Running hook \x1b[1m{}\x1b[0m", hook.name));
    let list = deno_task_shell::parser::parse(&hook.command)?;

    let env_vars = std::env::vars_os().collect::<HashMap<_, _>>();
    let cwd = std::env::current_dir()?;

    let exit_code =
        deno_task_shell::execute(list, env_vars, cwd, Default::default(), Default::default()).await;

    Ok(exit_code)
}

pub async fn run_all(
    progress: &ProgressBar,
    dev: bool,
    file_changed_path: Option<&PathBuf>,
    at: During,
) -> Result<(), Box<dyn Error>> {
    for hook in &SETTINGS.lock()?.hooks {
        let run_in_dev = !dev
            || (hook.dev != Dev::Disabled
                && if let Some(path) = &file_changed_path {
                    if let Dev::Watch(globs) = &hook.dev {
                        globs
                            .iter()
                            .map(|glob| Glob::new(glob).map(|it| it.is_match(path.as_path())))
                            .any(|r| matches!(r, Ok(true)))
                    } else {
                        true
                    }
                } else {
                    true
                });
        if run_in_dev && hook.during == at {
            run_hook(hook, progress).await?;
        }
    }

    Ok(())
}
