pub mod dialog;

use chrono_tz::Tz;

use dialog::stair::Way;

#[allow(dead_code)]
pub struct Project {
    name: String,
    description: Option<String>,
    timezone: Tz,
    scene: Option<Vec<Way>>,
}

impl Project {
    pub fn new(name: String, description: Option<String>, timezone: Tz) -> Project {
        Self {
            name,
            description,
            timezone,
            scene: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
