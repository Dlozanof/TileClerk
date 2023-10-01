use base64_serde::base64_serde_type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMetadata {
    pub name: String,
    pub x: i32,
    pub y: i32,
}

base64_serde_type!(Base64Standard, base64::engine::general_purpose::STANDARD);

#[derive(Default, Serialize, Deserialize)]
pub struct TilesetInformation
{
    pub rows: String,
    pub columns: String,
    pub width_px: String,
    pub height_px: String,
    pub tiles: Vec<TileMetadata>,
    pub image_path: String,
    #[serde(with = "Base64Standard")]
    pub image_raw: Vec<u8>,
}

