use std::collections::HashMap;

use layer;
use tileset;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Level {
    pub height: u32,
    pub width: u32,
    pub version: u32,
    
    pub properties: HashMap<String, String>,
    
    pub orientation: String,
    pub renderorder: String,
    
    pub tileheight: u32,
    pub tilewidth: u32,
    
    pub layers: Vec<layer::Layer>,
    pub tilesets: Vec<tileset::Tileset>,
}
