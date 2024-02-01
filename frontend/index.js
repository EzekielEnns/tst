
import * as sim from "./dist/wasm32-unknown-unknown/debug/simulation.wasm"
import bencode from 'bencode'

var/** @type {number} */ ptr,
   /** @type {number} */ len;

/**
 * @type {WebAssembly.Memory}
 */
const memeory = sim.memory;
ptr =sim.get_buffs()
len = sim.get_buff_len();
console.log(memeory)
let buff = new Uint8Array(memeory.buffer,ptr,len);
console.log(buff)
let render_data = bencode.decode(buff);
console.log(render_data)

sim.update();
ptr = sim.get_buffs(ptr,len);
len = sim.get_buff_len();
buff = new Uint8Array(memeory.buffer,ptr,len);
render_data = bencode.decode(buff);
console.log(JSON.stringify(render_data,null,2))

/**
 * 
 */
function genRenderData(){
    //update ptr and len
    //fetch pointers/render data
    //{actors:{textures:{ptr:}.....,len:number}....}
    //return render data to be used in glyph lib
    //note a update needs to happen to get changes
}

//TODO make render loop like in testing for glyph lib
