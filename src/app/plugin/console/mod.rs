use crate::app::project::{dialog::stair::Way, Project};
use std::io::Write;

pub struct Console;

impl<'a> super::Plugin<'a> for Console {
    fn serve(project: &Project, way: &Way) -> Result<(), super::PluginError> {
        way.start()
            .map(|s| println!("{}: {}", project.name(), s.response().get_text()));

        loop {
            let mut line = String::new();
            print!("You: ");
            std::io::stdout().flush().unwrap();

            std::io::stdin().read_line(&mut line).unwrap();

            match way.define_by_pattern(line.as_str()) {
                Some(s) => println!("{}: {}", project.name(), s.response().get_text()),
                None => (),
            }

            println!();
        }
    }

    fn kind() -> super::Kind {
        super::Kind::Console
    }
}
