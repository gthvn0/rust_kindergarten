const fs = require("fs");

const wasmModule = fs.readFileSync("output.wasm");

const importObject = {
  env: {
    loggme(arg) {
      console.log(arg);
    },
  },
};

WebAssembly
  .instantiate(wasmModule, importObject)
  .then((obj) => {
    const r = obj.instance.exports.a_sum(20, 22)
    console.log(r)
  });

