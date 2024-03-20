use serde::{Deserialize, Serialize, ser::SerializeStruct};

#[derive(Serialize, Deserialize,Clone, Copy,Debug)]
pub struct RenderValue {
    pub text:u8,
    pub color:u8,
}



#[derive(Serialize)]
pub struct RenderBuffers {
    pub textures: Vec<u8>,
    pub colors: Vec<u8>,
    pub locations: Vec<u8>,
    pub len: usize
}
impl RenderBuffers {
    pub fn new()-> RenderBuffers {
       RenderBuffers {
           textures: Vec::<u8>::new(),
           locations: Vec::<u8>::new(),
           colors:Vec::<u8>::new(),
           len:0
       } 
    }
}

#[derive(Serialize)]
pub struct RenderData {
    pub actors: RenderBuffers,
    pub items: RenderBuffers,
    pub tiles: RenderBuffers
}
