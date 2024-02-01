
import * as sim from "./dist/wasm32-unknown-unknown/debug/simulation.wasm"
import bencode from 'bencode'
/**
 * @type {WebAssembly.Memory}
 */
const memeory = sim.memory;
let ptr =sim.alloc_buffers()
let len = sim.get_buff_len();
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
console.log(render_data)
