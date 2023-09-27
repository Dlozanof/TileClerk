
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMetadata {
    pub name: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct TilesetInformation
{
    pub rows: String,
    pub columns: String,
    pub width_px: String,
    pub height_px: String,
    pub tiles: Vec<TileMetadata>,
}

