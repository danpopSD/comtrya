use super::FileAction;
use crate::manifests::Manifest;
use crate::{actions::Action, atoms::Atom};
use anyhow::{Context as ResultWithContext, Result};
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::{ops::Deref, path::PathBuf, u32};
use tera::Context;
use tracing::error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileCopy {
    pub from: String,
    pub to: String,

    #[serde(default = "default_chmod", deserialize_with = "from_octal")]
    pub chmod: u32,

    #[serde(default = "default_template")]
    pub template: bool,
}

fn from_octal<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let chmod: u32 = Deserialize::deserialize(deserializer)?;
    u32::from_str_radix(&chmod.to_string(), 8).map_err(D::Error::custom)
}

fn default_chmod() -> u32 {
    0o644
}

fn default_template() -> bool {
    false
}

impl FileCopy {}

impl FileAction for FileCopy {}

impl Action for FileCopy {
    fn plan(&self, manifest: &Manifest, context: &Context) -> Vec<Box<dyn Atom>> {
        // There should be an Atom for rendering too
        let tera = self.init(manifest);

        let contents = match if self.template {
            tera.render(self.from.clone().deref(), context)
                .context("Failed to render template")
        } else {
            self.load(manifest, &self.from)
        } {
            Ok(contents) => contents,
            Err(_) => {
                // We need some way to bubble an error up the chain
                error!("Failed to get contents for FileCopy action");
                return vec![];
            }
        };

        use crate::atoms::command::Exec;
        use crate::atoms::file::{Chmod, Create, SetContents};

        let path = PathBuf::from(&self.to);
        let parent = path.clone();

        vec![
            Box::new(Exec {
                command: String::from("mkdir"),
                arguments: vec![
                    String::from("-p"),
                    String::from(parent.parent().unwrap().to_str().unwrap()),
                ],
                ..Default::default()
            }),
            Box::new(Create { path: path.clone() }),
            Box::new(Chmod {
                path: path.clone(),
                mode: self.chmod,
            }),
            Box::new(SetContents { path, contents }),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::Actions;

    #[test]
    fn it_can_be_deserialized() {
        let yaml = r#"
- action: file.copy
  from: a
  to: b
"#;

        let mut actions: Vec<Actions> = serde_yaml::from_str(yaml).unwrap();

        match actions.pop() {
            Some(Actions::FileCopy(file_copy)) => {
                assert_eq!("a", file_copy.from);
                assert_eq!("b", file_copy.to);
            }
            _ => {
                panic!("FileCopy didn't deserialize to the correct type");
            }
        };
    }
}
