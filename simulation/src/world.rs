use crate::Entity;

#[derive(PartialEq, Eq)]
pub struct Pos {
    //TODO enforce being topleft coords 0,0 -> width,height
    //Stop roll over 
    pub x:i32, //column
    pub y:i32  //row
}

impl Pos {
    //TODO add a step
    pub fn approch<'a>(&mut self, dest: &'a Pos) {
        let diff: Pos = Pos {x: dest.x - self.x, y:dest.y - self.y};
        self.y += if diff.y > 0 {1} else if diff.y < 0 {-1} else {diff.y};
        self.x += if diff.x > 0 {1} else if diff.x < 0 {-1} else {diff.x};
    }
}


pub struct MobileEntity {
    pub entity: Box<dyn Entity>,
    pub location: Pos,
    pub destination: Option<Pos>,
}
pub struct StationayEntity{
    pub entity: Box<dyn Entity>,
    pub location: Pos,
}
//TODO add animated entity


/*
 * mvp needs to have movment and combat 
 * there dosnet need to be any items, things can be hard coded and worked in
 * also theres world generation....
 */
pub struct Explore{ 
    pub me: Vec<MobileEntity>,
    pub se: Vec<StationayEntity>,
    pub w: u8,
    pub h: u8,
}

//determins if in range
//moves entities and takes sp
//stores combat state
struct Combat{ 
    me: Vec<MobileEntity>,
    se: Vec<StationayEntity>,
    w: u8,
    h: u8,
}

impl Combat{ }

trait World {
    // progressing the world sim by one time
    // move mobile entiets in explore struct 
    fn step(&mut self);

    //this will switch the global world or comat to the 
    //other state
    fn switch(&mut self, target:Box<dyn World>); //essentally just take the mem to another world
    // fn create_expe(&mut self)-> Explore{
    //    Explore {
    //        me:std::mem::take(&mut self.me),
    //        w: self.w,
    //        se:std::mem::take(&mut self.se),
    //        h: self.h
    //    } 
    // }

    //a collison happens and 
    fn collison(&mut self);

    //save state
    // unsafe fn init_save() -> *mut u8;
    // unsafe fn get_save(ptr:*mut u8) -> *mut u8;
    // unsafe fn len_save() -> *mut u8;

    //getting render values and init pointers
    unsafe fn init_text(&self) -> *mut u8;
    // unsafe fn init_color(&self) -> *mut u8;
    // unsafe fn init_alpha(&self) -> *mut u8;
    unsafe fn get_text(&self, ptr:*mut u8) -> *mut u8;
    // unsafe fn get_color(&self,ptr:*mut u8) -> *mut u8;
    // unsafe fn get_alpha(&self,ptr:*mut u8) -> *mut u8;
    unsafe fn len_text(&self) -> *mut u8;
    // unsafe fn len_color(&self) -> *mut u8;
    // unsafe fn len_alpha(&self) -> *mut u8;
}

