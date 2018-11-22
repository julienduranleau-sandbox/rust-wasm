const rustPromise = import("../../hello-wasm/pkg/hello_wasm.js")

rustPromise.then(rust => {
  rust.greet("WebAssembly")
})

