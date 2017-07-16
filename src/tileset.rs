use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Tileset {
    pub name: String,
    pub firstgid: u32,
    pub tilecount: u32,
    pub tileheight: u32,
    pub tilewidth: u32,
    
    pub columns: u32,
    pub image: String,
    pub imageheight: u32,
    pub imagewidth: u32,
    pub margin: u32,
    pub spacing: u32,
    
    pub properties: Option<HashMap<String, String>>,
    pub terrains: Option<Vec<Terrain>>,
    pub tileproperties: HashMap<u32, HashMap<String, String>>,
    pub tiles: HashMap<u32, [u32; 4]>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Terrain {
    pub name: String,
    pub tile: u32,
}
