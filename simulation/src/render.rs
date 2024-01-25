use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone, Copy)]
pub struct RenderData {
    pub value:u8,
    pub color:u32,
    pub alpha:u8,
}
