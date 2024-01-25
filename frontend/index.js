import * as sim from "./dist/wasm32-unknown-unknown/debug/simulation.wasm"
import bencode from 'bencode'
console.log(sim)
/**
 * @type {WebAssembly.Memory}
 */
const memeory = sim.memory;
const ptr = sim.hello_world();

const buff = new Uint8Array(memeory.buffer,ptr,sim.len())
console.log(buff)
let de = new TextDecoder()
let test = bencode.decode(buff,"")
console.log(test)
console.log(de.decode(test.name));
//let t2 = {hi:1}
//console.log(bencode.decode(encode(t2)))


//TODO add type def for sim
//https://stackoverflow.com/quetions/36737921/how-to-extend-a-typedef-parameter-in-jsdoc
