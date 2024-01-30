use serde::{Deserialize, Serialize, ser::SerializeStruct};

#[derive(Serialize, Deserialize,Clone, Copy)]
pub struct RenderValue {
    pub text:u8,
    pub color:u8,
}

//this is a struct just cause its handly 
pub struct Buffer { pub ptr: *mut u8 }
impl Buffer { fn new(ptr:*mut u8) -> Buffer { Buffer { ptr } } }
//no need for deserilize since this is only for the js to access
impl Serialize for Buffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("Buffer",2)?;
            state.serialize_field("ptr", &format!("{:p}",self.ptr))?;
            state.end()
        }
}

#[derive(Serialize)]
pub struct RenderBuffers {
    pub textures: Buffer,
    pub colors: Buffer,
    pub locations: Buffer,
    pub len: usize
}

pub enum BINDEX { TEXTURES=0,LOCATIONS,COLORS}

impl RenderBuffers {
    fn new(len:usize, b:&[*mut u8;3])->RenderBuffers {
        RenderBuffers {
            len,
            //TODO make this easier to deal with
            colors:Buffer::new(b[BINDEX::COLORS as usize]), 
            textures: Buffer::new(b[BINDEX::TEXTURES as usize]),
            locations:Buffer::new(b[BINDEX::LOCATIONS as usize]),
        }
    }
    pub unsafe fn get_textures(&mut self)-> Vec<u8>{
        Vec::from_raw_parts(self.textures.ptr,self.len,self.len)
    }
    pub unsafe fn get_colors(&mut self)-> Vec<u8>{
        Vec::from_raw_parts(self.colors.ptr,self.len,self.len)
    }
    pub unsafe fn get_locations(&mut self)-> Vec<u8>{
        Vec::from_raw_parts(self.locations.ptr,self.len,self.len)
    }
}

#[derive(Serialize)]
pub struct RenderData {
    pub actors: RenderBuffers,
    pub items: RenderBuffers,
    pub map: RenderBuffers
}
macro_rules! create_buffers {
    ($size:expr,$total:expr) => {{
        let mut ptr_arr = [std::ptr::null_mut(); $total];
        for i in 0..$total {
            let mut arr = Vec::<u8>::with_capacity($size);
            ptr_arr[i] = arr.as_mut_ptr();
            std::mem::forget(arr);
        }
        RenderBuffers::new($size,&ptr_arr)
    }};
}
impl RenderData {
    pub fn new(len:usize)-> RenderData {
        RenderData {
            map: create_buffers!(len,3),
            actors: create_buffers!(0,3),
            items: create_buffers!(0,3),
        }
    }
    pub fn grow_actors(&mut self, len: usize) {
        self.actors.len = len;
    }
    pub fn grow_map(&mut self, len: usize) {
        self.map.len = len;
    }
    pub fn grow_items(&mut self, len: usize) {
        self.items.len = len;
    }
}
