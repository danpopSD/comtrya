use super::PackageProvider;
use crate::atoms::command::Exec;
use crate::{actions::package::PackageVariant, atoms::Atom};
use serde::{Deserialize, Serialize};
use tracing::warn;
use which::which;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Yay {}

impl PackageProvider for Yay {
    fn name(&self) -> &str {
        "Yay"
    }

    fn available(&self) -> bool {
        match which("yay") {
            Ok(_) => true,
            Err(_) => {
                warn!(message = "yay not available");
                false
            }
        }
    }

    fn bootstrap(&self) -> Vec<Box<dyn Atom>> {
        vec![
            Box::new(Exec {
                command: String::from("pacman"),
                arguments: vec![
                    String::from("-S"),
                    String::from("--noconfirm"),
                    String::from("base-devel"),
                    String::from("git"),
                ],
                privileged: true,
                ..Default::default()
            }),
            Box::new(Exec {
                command: String::from("git"),
                arguments: vec![
                    String::from("clone"),
                    String::from("https://aur.archlinux.org/yay.git"),
                    String::from("/tmp/yay"),
                ],
                ..Default::default()
            }),
            Box::new(Exec {
                command: String::from("makepkg"),
                arguments: vec![String::from("-si"), String::from("--noconfirm")],
                working_dir: Some(String::from("/tmp/yay")),
                ..Default::default()
            }),
        ]
    }

    fn has_repository(&self, _package: &PackageVariant) -> bool {
        false
    }

    fn add_repository(&self, _package: &PackageVariant) -> Vec<Box<dyn Atom>> {
        vec![]
    }

    fn query(&self, package: &PackageVariant) -> Vec<String> {
        package.packages()
    }

    fn install(&self, package: &PackageVariant) -> Vec<Box<dyn Atom>> {
        vec![Box::new(Exec {
            command: String::from("yay"),
            arguments: [
                vec![
                    String::from("-S"),
                    String::from("--noconfirm"),
                    String::from("--nocleanmenu"),
                    String::from("--nodiffmenu"),
                ],
                package.extra_args.clone(),
                package.packages(),
            ]
            .concat(),
            ..Default::default()
        })]
    }
}
