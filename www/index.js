


async function init() {

  const importObject = {
    console: {
      log: () => {
        console.log("Just logging something!");
      },
      error: () => {
        console.log("I am just error");
      }
    }
  }

  const response = await fetch("sum.wasm");
  const buffer = await response.arrayBuffer();
  const wasm = await WebAssembly.instantiate(buffer, importObject);

  const sumFunction = wasm.instance.exports.sum;
  const wasmMemory = wasm.instance.exports.mem;
  const uint8Array = new Uint8Array(wasmMemory.buffer, 0, 2);

  const hiText = new TextDecoder().decode(uint8Array);
  console.log(hiText);
}

init();
