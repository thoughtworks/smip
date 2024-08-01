# vsomeip-rs

Rust bindings to [vsomeip](https://github.com/COVESA/vsomeip)


### vsomeip-sys
This is the low-level binding to the vsomeip library. It is a direct mapping of the C++ API to Rust. This crate is not intended to be used directly by the user. It is used by the `vsomeip-rs` crate to provide a safe and idiomatic Rust API to the user.

### vsomeip-rs
This is the high-level binding to the vsomeip library. It provides a safe and idiomatic Rust API to the user. This crate is intended to be used by the user to interact with the vsomeip library.

## Prerequisites
### Install vsomeip
To use the vsomeip library, you need to install it first. 
> Note: The vsomeip library can be installed on Linux only.

```bash
git clone git@github.com:COVESA/vsomeip.git
cd vsomeip
mkdir build
cd build
cmake ..
make
sudo make install
```

## Build and Run

The vsomeip library is loaded dynamically at runtime, so it must be present in the dynamic library path when running any vsomeip application. 
This can be done by setting the `LD_LIBRARY_PATH` environment variable.

vsomeip also uses a config file to configure the vsomeip services. The config file's path needs to be set in the `VSOMEIP_CONFIGURATION` environment variable.

To run the example, run the scripts/run_example_server bash script which sets these environment variables appropriately.

```bash
scripts/run_example_server
```
