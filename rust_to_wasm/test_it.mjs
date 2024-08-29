import fs from 'node:fs';

const wasmModule = fs.readFileSync("output.wasm");
var memory; // We need to access the memory to read msg and log it

function loggme(sptr, slen) {
  const buf = new Uint8Array(memory.buffer, sptr, slen);
  const str = new TextDecoder("utf8").decode(buf);
  console.log(str);
}

const imports = {
  env: {
    loggme
  },
};

WebAssembly
  .instantiate(wasmModule, imports)
  .then((obj) => {
    memory = obj.instance.exports.memory;
    const r = obj.instance.exports.a_sum(20, 22)
    console.log(r)
  });

