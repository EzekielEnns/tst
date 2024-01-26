
use crate::entities::{Item, Actor, Tile};

// #[derive(PartialEq, Eq)]
// pub struct Pos {
//     //TODO enforce being topleft coords 0,0 -> width,height
//     //Stop roll over 
//     pub x:i32, //column
//     pub y:i32  //row
// }
//
// impl Pos {
//     //TODO add a step
//     pub fn approch<'a>(&mut self, dest: &'a Pos) {
//         let diff: Pos = Pos {x: dest.x - self.x, y:dest.y - self.y};
//         self.y += if diff.y > 0 {1} else if diff.y < 0 {-1} else {diff.y};
//         self.x += if diff.x > 0 {1} else if diff.x < 0 {-1} else {diff.x};
//     }
// }
//
// trait GetPos {
//     fn get_local(&self) -> Pos;
// }
//
// pub struct MobileEntity<T:Entity> {
//     pub entity: T,
//     pub location: Pos,
//     pub destination: Option<Pos>,
// }
// impl GetPos for MobileEntity{
//    fn get_local(&self) -> Pos {self.location} 
// }
//
// pub struct StationayEntity<T: Entity>{
//     pub location: Pos,
//     pub entity: T,
// }
// impl GetPos for StationayEntity {
//    fn get_local(&self) -> Pos {self.location} 
// }
//TODO add animated entity

pub struct Pos {x:usize,y:usize}

fn get_pos(b:Pos,i:usize)->Pos{
        let column = i % b.x;
        let row =  (i- (i % b.x)) / b.x; 
        Pos {x:column, y:if row > b.y { row % b.y} else {row}}
}
fn get_index(b:&Pos,p:&Pos)->usize { p.y * b.y + p.x }

/*
 * mvp needs to have movment and combat 
 * there dosnet need to be any items, things can be hard coded and worked in
 * also theres world generation....
 */
pub struct World{ 
    pub actors: Vec<Option<Actor>>, 
    pub items: Vec<Option<Item>>,
    pub tiles: Vec<Option<Tile>>,   
    pub dim: Pos,

    //TODO add combat states i.e. enemy team and player team
}

impl World{
    fn new(width:usize,height:usize)->World{
        let len = width*height;
        World {
            dim: Pos{y:height,x:width},
            actors: Vec::with_capacity(len),
            items: Vec::with_capacity(len),
            tiles: Vec::with_capacity(len),
        }
    }

    fn len(&self)->usize{self.dim.x*self.dim.y}
    fn move_actor(&mut self, new_pos:Pos, old_pos:Pos) -> Option<bool> {
        let new = get_index(&self.dim,&new_pos);
        let old =  get_index(&self.dim,&old_pos);
        //check for collison
        if let Some(tile) = self.tiles.get(new)? {
            if tile.collision { return Some(false) }
        }
        if let Some(actor) = self.actors.get(new)? {
            //TODO store dialog into dialog static pointer if not hostile
            if actor.is_hostile {
                //start swap to combat mode
                return Some(false);
           }
        }
        //TODO fix this
        else if let Some(item) = self.items.get(new)? {
            self.actors[old]?.items.push(self.items.remove(new)?);
        }
        self.actors.swap(old,new);
        return Some(true);
    }
    unsafe fn render_init(&self)-> *mut u8 {
        todo!() 
        /* setup a bunch of pointers for rendering 
         * then pointer to that array in js
         *
         *
         * it would look something like this VVVVVVVVV
         * bot obviliouly different
         */
        // let ptrs = [
        //     self.actors.as_ptr(),
        //     self.items.as_ptr(),
        //     self.tiles.as_ptr(),
        // ]
    }

    unsafe fn render_actors(&self,color:*mut u8,text:* mut u8, alpha:*mut u8){
        //updates buffers
    }
    unsafe fn render_items(&self,color:*mut u8,text:* mut u8, alpha:*mut u8){
        //updates buffers
    }
    unsafe fn render_tiles(&self,color:*mut u8,text:* mut u8, alpha:*mut u8){
        //updates buffers
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
