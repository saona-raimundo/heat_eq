use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize, strum::EnumString)]
pub enum BorderConditions {
    Fixed,
    Periodic,
}

impl Default for BorderConditions {
    fn default() -> Self {
        BorderConditions::Fixed
    }
}
