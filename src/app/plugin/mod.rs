pub mod console;

use thiserror::Error;

use crate::app::project::{dialog::stair::Way, Project};

pub enum Kind {
    Console,
}

impl Kind {
    pub fn value(&self) -> &str {
        match *self {
            Kind::Console => "CONSOLE_PLUGIN",
        }
    }
}

#[derive(Error, Debug)]
pub enum PluginError {}

pub trait Plugin<'a> {
    fn kind() -> Kind;
    fn serve(project: &Project, way: &Way) -> Result<(), PluginError>;
}
