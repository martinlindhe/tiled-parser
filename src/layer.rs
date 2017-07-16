use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Layer {
    pub name: String,
    pub opacity: f32,
    pub properties: Option<HashMap<String, String>>,
    pub visible: bool,
    pub width: u32,
    pub height: u32,
    pub x: f32,
    pub y: f32,
    
    // for tile layers
    pub data: Vec<u32>,

    // for object layers
    pub draworder: String,
    pub objects: Vec<Object>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Object {
    pub id: u32,
    pub name: String,
    
    #[serde(rename = "type")]
    pub _type: String,
    pub gid: Option<u32>,
    pub ellipse: Option<bool>,
    pub polygon: Option<Vec<PolyPoint>>,
    
    pub properties: HashMap<String, String>,
    pub rotation: f32,
    pub visible: bool,
    
    pub height: f32,
    pub width: f32,
    
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct PolyPoint {
    pub x: f32,
    pub y: f32,
}
