pub use app::TemplateApp;

mod app;
mod appui;
mod views;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EScale {
    Small,
    Medium,
    Large,
}
impl From<EScale> for f32 {
    fn from(val: EScale) -> Self {
        match val {
            EScale::Small => 2.2,
            EScale::Medium => 3.0,
            EScale::Large => 4.5,
        }
    }
}
