#![allow(dead_code)]
/*
turns out for some reason you cant send a array of pointers in wasm to 
js......


this means my current render system dose not work,
and i will have to overcome this some how 
*/


use maps::{Generate, first_test_world};
use once_cell::sync::Lazy;
use world::{World, move_player, get_index, Pos};
mod utils;
mod skills; 
mod world;
mod entities;
mod stats;
mod render;
mod maps;

static mut WORLD: Lazy<World> = Lazy::new(|| {
    let mut wl = World::new(10,10);
    wl.generate(first_test_world);
    wl
});

#[no_mangle]
pub unsafe extern "C" fn move_pc(x:usize, y:usize ) {
    let new = get_index(&WORLD.dim,&Pos {x,y});
    move_player(&mut WORLD,new);
}

#[no_mangle]
pub unsafe extern "C" fn get_buffs(ptr: *mut u8, size: usize)-> *mut u8 {
    if ptr == std::ptr::null_mut() || size == 0 {
        WORLD.render_alloc()
    } else {
        //dose alloc and dealloc
        WORLD.render_update(ptr,size)
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_buff_len()-> usize{
    WORLD.render_len()
}

#[no_mangle]
pub unsafe extern "C" fn update() {
    WORLD.render_actors();
    WORLD.render_items();
    WORLD.render_tiles();
}

#[no_mangle]
pub unsafe extern "C" fn test() -> *mut *mut u8  {
    let mut buf = vec![WORLD.render_data.actors.textures.ptr, 
                    WORLD.render_data.actors.colors.ptr,
                    WORLD.render_data.actors.locations.ptr
    ];
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}


#[cfg(test)]
mod tests {
    use crate::{update, WORLD};

    #[test]
    fn it_works() {
        unsafe {
            println!("hi");
            update();
            println!("ih");
            println!("{}",WORLD.render_data.actors.len);
            panic!()
        }
    }
}
