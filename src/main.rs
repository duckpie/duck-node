use regex::Regex;

use duck_node::app::plugin::{console::Console as PluginConsole, Plugin};
use duck_node::app::project::{dialog::*, *};

fn main() {
    let project = Project::new("DuckBot".to_string(), None, chrono_tz::US::Pacific);

    let mut way = stair::Way::new();
    let s1 = stair::Stage::new(
        stair::StageKind::Basic,
        vec![
            Regex::new(r"start").unwrap(),           
        ],
        "cmd_start".to_string(),
        None,
        Some(stair::Body::new(Some("Hello from DuckBot!".to_string()))),
    );

    let s2 = stair::Stage::new(
        stair::StageKind::Basic,
        vec![Regex::new(r"help").unwrap()],
        "cmd_help".to_string(),
        None,
        Some(stair::Body::new(Some("Help info here".to_string()))),
    );

    way.add_many(vec![s1.unwrap(), s2.unwrap()]);

    PluginConsole::serve(&project, &way).unwrap();
}
