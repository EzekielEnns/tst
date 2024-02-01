#![allow(dead_code)]
mod utils;
mod skills; 
mod world;
mod entities;
mod stats;
mod render;
mod maps;

//TODO init world either via lazy static or via init function
#[no_mangle]
pub unsafe extern "C" fn init() { 
    //TOOD either lazy static or init
}


#[no_mangle]
pub unsafe extern "C" fn move_player() {
    //moves player to specific spot
    //sets player_index after doing move
}

#[no_mangle]
pub unsafe extern "C" fn get_buffs() {
    //init and send the buffer serilized
}
#[no_mangle]
pub unsafe extern "C" fn update() {
    //updates world, and render buffers
}

