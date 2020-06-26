# PIX Wrapper

This library wraps PIX event runtime API so that it can be used from Rust.  
Unfortunately, the original headers are C++-only, and they use templates in such a way that rust-bindgen cannot handle them, so we have to introduce a second layer of indirection - C++ file wrapper.cpp that exposes plain C API, consumable by rust-bindgen, in its header.
To build this library, you need to set PIX_RUNTIME_PATH environment variable that would point to your directory with PIX runtime headers/libraries.