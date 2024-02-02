
import * as sim from "./dist/wasm32-unknown-unknown/debug/simulation.wasm"
import bencode from 'bencode'

var/** @type {number} */ ptr,
   /** @type {number} */ len;

/**
 * @type {WebAssembly.Memory}
 */
const memeory = sim.memory;
ptr = sim.get_buffs()
console.log(memeory)
// len = new Uint8Array(memeory.buffer,ptr,1);
// console.log(len[0])
let buff = new Uint8Array(memeory.buffer,ptr,sim.get_len());
console.log(buff)
let render_data = bencode.decode(buff);
console.log(render_data)
console.log(String.fromCharCode(render_data.actors.textures[0]))

// sim.update();
// ptr = sim.get_buffs(ptr,len);
// len = sim.get_buff_len();
// buff = new Uint8Array(memeory.buffer,ptr,len);
// render_data = bencode.decode(buff);
//
// /** @type {number} */
// ptr = sim.test()
// console.log(ptr)
// let ptrs = new Uint8Array(memeory.buffer,ptr,3)
// console.log(ptrs)
//let t = new TextDecoder()
//console.log(ptrs)

/**
 * 
 */
function getRenderData(){
    //do whats on the top and just return that 
}
//TODO bind movment  

//TODO make render loop like in testing for glyph lib
//getRenderData
//in loop approche render data based on factor
//repeat

