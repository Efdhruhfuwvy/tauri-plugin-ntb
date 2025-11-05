use serde::{Deserialize, Serialize};
#[cfg(target_os = "linux")]
use zvariant::OwnedValue;
#[cfg(target_os = "linux")]
use zvariant::Type;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all_fields = "camelCase", tag = "type")]
pub enum DragBehavior {
    Immediate,
    Threshold { threshold: u16 },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all_fields = "camelCase")]
pub enum WindowControl {
    Minimize,
    Maximize,
    Close,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowControls {
    pub left: Vec<WindowControl>,
    pub right: Vec<WindowControl>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all_fields = "camelCase", tag = "type")]
pub(crate) enum WindowControlImage {
    SVG { svg: String },
    Text { font: String, size: Option<u32>, text: String },
}

#[cfg(target_os = "linux")]
#[derive(Debug, Deserialize, Type, OwnedValue)]
pub(crate) struct GResourceData {
    pub size: u32,
    pub flags: u32,
    pub content: Vec<u8>,
}
