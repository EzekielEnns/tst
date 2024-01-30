
use crate::{entities::{Item, Actor, Tile}, render::RenderData};
//TODO add animated entity
pub struct Pos {x:usize,y:usize}

//gets the postion based on a index
fn get_pos(b:Pos,i:usize)->Pos{
        let column = i % b.x;
        let row =  (i- (i % b.x)) / b.x; 
        Pos {x:column, y:if row > b.y { row % b.y} else {row}}
}

//gets the index based on a postion 
fn get_index(b:&Pos,p:&Pos)->usize { p.y * b.y + p.x }

/*
 * mvp needs to have movment and combat 
 * there dosnet need to be any items, things can be hard coded and worked in
 * also theres world generation....
 */
pub struct World{ 
    pub actors: Vec<Option<Actor>>, 
    pub items: Vec<Option<Item>>,
    pub tiles: Vec<Tile>,   
    pub dim: Pos,
    pub render_data: RenderData
    //TODO add combat states i.e. enemy team and player team
}

enum RenderType{ ACTORS=0,ITEMS,TILES}
impl World{
    fn new(width:usize,height:usize)->World{
        let len = width*height;
        World {
            dim: Pos{y:height,x:width},
            actors: Vec::with_capacity(len),
            items: Vec::with_capacity(len),
            tiles: Vec::with_capacity(len),
            render_data: RenderData::new(len)
        }
    }

    fn len(&self)->usize{self.dim.x*self.dim.y}
    fn move_actor(&mut self, new_pos:Pos, old_pos:Pos) -> Option<bool> {
        let new = get_index(&self.dim,&new_pos);
        let old =  get_index(&self.dim,&old_pos);
        //check for collison
        if self.tiles[new].collision {
            return Some(false)
        }
        if let Some(actor) = self.actors.get(new)? {
            //TODO store dialog into dialog static pointer if not hostile
            if actor.is_hostile {
                //TODO start swap to combat mode
                return Some(false);
           }
        }
        else if let Some(item) = self.items.get(new)? {
            if let Some(actor) = self.actors[old].as_mut() {
                actor.items.push(*item);
            }
            self.items[new] = None;
        }
        self.actors.swap(old,new);
        return Some(true);
    }

    //lets js read the render_data struct from the world
    unsafe fn render_alloc(&self)-> *mut u8 {
        let mut buf = bendy::serde::to_bytes(&self.render_data).unwrap();
        let ptr = buf.as_mut_ptr();
        std::mem::forget(buf);
        return ptr;
    }


    //updates the render values inside the render_data
    unsafe fn render(&mut self, v:RenderType){
        let mut value = match v {
            RenderType::ACTORS => {self.render_data.actors.get_textures()}
            RenderType::TILES => {self.render_data.map.get_textures()}
            RenderType::ITEMS => {self.render_data.items.get_textures()}
        };
        value.clear();
        let mut color = match v {
            RenderType::ACTORS => {self.render_data.actors.get_colors()}
            RenderType::TILES => {self.render_data.map.get_colors()}
            RenderType::ITEMS => {self.render_data.items.get_colors()}
        };
        color.clear();
        let mut positions = match v {
            RenderType::ACTORS => {self.render_data.actors.get_locations()}
            RenderType::TILES => {self.render_data.map.get_locations()}
            RenderType::ITEMS => {self.render_data.items.get_locations()}
        };
        positions.clear();
        
        //update the values later based on actor glyphs
        for i in 0..self.len() {
            match v {
                RenderType::ACTORS => {
                    if let Some(_actor) = &self.actors[i] {
                        positions.push(i as u8);
                        color.push(_actor.render_value.text);
                        value.push(_actor.render_value.color);
                    }
                }
                RenderType::ITEMS => {
                    if let Some(item) = &self.items[i] {
                        positions.push(i as u8);
                        color.push(item.render_value.text);
                        value.push(item.render_value.color);
                    }
                }
                RenderType::TILES => {
                    //TODO needs culling
                    let tile = &self.tiles[i];
                    positions.push(i as u8);
                    color.push(tile.render_value.text);
                    value.push(tile.render_value.color);
                }
            };

        }
        match v {
            RenderType::ITEMS => {
                self.render_data.items.len = value.len(); 
            }
            RenderType::ACTORS => {
                self.render_data.actors.len = value.len();
            }
            RenderType::TILES => {}
        } 
    }
    pub unsafe fn render_actors(&mut self){
        self.render(RenderType::ACTORS);
    }
    pub unsafe fn render_items(&mut self){
        //updates buffers
        self.render(RenderType::ITEMS);
    }
    pub unsafe fn render_tiles(&mut self){
        //updates buffers
        self.render(RenderType::TILES);
    }
}


//determins if in range
//moves entities and takes sp
//stores combat state
// struct Combat{ 
//     pub actors: Vec<MobileEntity<Actor>>,
//     pub items: Vec<StationayEntity<Item>>,
//     pub tiles: Vec<Tile>,   //this is not a StationayEntity becuase it will fill the vector to w
//                             //and h
//     pub w: u8,
//     pub h: u8,
//
//     //TODO add combat info
//     //TODO implment into https://doc.rust-lang.org/std/convert/trait.Into.html
// }
//
// impl Combat{ }

// trait World {
//     // progressing the world sim by one time
//     // move mobile entiets in explore struct 
//     fn step(&mut self);
//
//     //this will switch the global world or comat to the 
//     //other state
//     // fn switch(&mut self, target:Box<dyn World>); //essentally just take the mem to another world
//     // // fn create_expe(&mut self)-> Explore{
//     // //    Explore {
//     // //        me:std::mem::take(&mut self.me),
//     // //        w: self.w,
//     // //        se:std::mem::take(&mut self.se),
//     // //        h: self.h
//     // //    } 
//     // // }
//
//     //a collison happens and 
//     fn collison(&mut self);
//
//     //save state
//     // unsafe fn init_save() -> *mut u8;
//     // unsafe fn get_save(ptr:*mut u8) -> *mut u8;
//     // unsafe fn len_save() -> *mut u8;
//
//     //getting render values and init pointers
//     unsafe fn init_text(&self) -> *mut u8;
//     // unsafe fn init_color(&self) -> *mut u8;
//     // unsafe fn init_alpha(&self) -> *mut u8;
//     unsafe fn get_text(&self, ptr:*mut u8) -> *mut u8;
//     // unsafe fn get_color(&self,ptr:*mut u8) -> *mut u8;
//     // unsafe fn get_alpha(&self,ptr:*mut u8) -> *mut u8;
//     unsafe fn len_text(&self) -> *mut u8;
//     // unsafe fn len_color(&self) -> *mut u8;
//     // unsafe fn len_alpha(&self) -> *mut u8;
// }
//
