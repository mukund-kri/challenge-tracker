use std::{collections::HashSet, error::Error};
/// Utilities for generating org-mode TODO lists.
use tinytemplate::TinyTemplate;

use crate::config::Config;

#[derive(Debug, Serialize)]
struct TodoContext {
    language: String,
    project: String,
    chapter: String,
    body: String,
}

static TODO_TEMPLATE: &str = r#"
* TODO [{language}] {project} :: Learn {chapter}
{body}"#;

pub fn todo_do_chapter(
    missing_chapters: &HashSet<String>,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("todo", TODO_TEMPLATE)?;

    let mut missing_chapters = missing_chapters.iter().collect::<Vec<_>>();
    missing_chapters.sort();

    for chapter in missing_chapters {
        let context = TodoContext {
            language: config.language.clone(),
            project: config.project.clone(),
            chapter: chapter.clone(),
            body: String::from(""),
        };

        let result = tt.render("todo", &context)?;
        print!("{}", result);
    }
    Ok(())
}
