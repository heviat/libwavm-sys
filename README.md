### libwasvm-sys

##### a FFI library for WAVM (Web Assembly Virtual Machine) C-API

WAVM can be find [here](https://github.com/WAVM/WAVM) 

The bindings for Rust are generated via [bindgen](https://crates.io/crates/bindgen) and not wrapped any further at this moment. Therefore, you will need to write some wrapper functions to handle the unsafe API calls by yourself.
Currently, the bindings are only generated for the static WAVM C-API on Windows systems. As soon as possible, there will be bindings for Unix systems.

Please keep in mind that this crate is in a very early stage and not well tested. Additionally, the provided interfaces could change rapidly.

#### Getting Started

All you need is a working WAVM installation on your system. By using static libraries, this crate should be working then.

Example code:

```
extern crate libwavm_sys;

fn main() {
    //Take a look at https://github.com/WAVM/WAVM/blob/master/Examples/embedder/c/embedder-example.c to see how the C API of WAVM works
    
    unsafe {
        let engine = libwavm_sys::wasm_engine_new();
        //Do whatever you want with the engine
    }
}
```