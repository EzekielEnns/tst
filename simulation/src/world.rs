
use crate::{entities::{Item, Actor, Tile}, render::RenderData, maps::Generate};
//TODO add animated entity
#[derive(Clone, Copy)]
pub struct Pos {pub x:usize, pub y:usize}

//gets the postion based on a index
pub fn get_pos(b:Pos,i:usize)->Pos{
        let column = i % b.x;
        let row =  (i- (i % b.x)) / b.x; 
        Pos {x:column, y:if row > b.y { row % b.y} else {row}}
}

//gets the index based on a postion 
pub fn get_index(b:&Pos,p:&Pos)->usize { p.y * b.y + p.x }

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
    pub render_data: RenderData,
    pub player_index: usize;
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
            render_data: RenderData::new(len),
            player_index: 0
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
                //TODO add a notification buffer 
                //that will popup and tell the player what they picked up
                actor.items.push(*item);
            }
            self.items[new] = None;
        }
        self.actors.swap(old,new);
        return Some(true);
    }

    //TODO move to trait for sanity
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
                    if let Some(actor) = &self.actors[i] {
                        positions.push(i as u8);
                        color.push(actor.render_value.text);
                        value.push(actor.render_value.color);
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
        self.render(RenderType::ITEMS);
    }
    pub unsafe fn render_tiles(&mut self){
        self.render(RenderType::TILES);
    }
}

impl Generate for World {
    fn generate(&mut self,func: fn(dim: &Pos, actors: &mut [Option<Actor>],items:&mut [Option<Item>],tiles:&mut [Tile])) {
        func(&self.dim,&mut self.actors,&mut self.items,&mut self.tiles);
    }
}
