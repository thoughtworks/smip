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
# Check out to a compatible version of vsomeip 
git checkout cf497232adf84f55947f7a24e1b64e04b49f1f38
mkdir build
cd build
# See these two issues for why this flag needs to be set
# https://github.com/COVESA/vsomeip/issues/688
# https://github.com/COVESA/vsomeip/issues/527
cmake -E env CXXFLAGS="-Wno-error=stringop-overflow" cmake .. -DCMAKE_INSTALL_PREFIX:PATH=/usr -DCMAKE_BUILD_TYPE=Debug
make
sudo make install
```

## Example usage
```rust
use vsomeip_rs::*;

const SERVICE_ID: ServiceId = 0x1111;
const INSTANCE_ID: InstanceId = 0x2222;
const METHOD_ID: MethodId = 0x3333;

fn server() {
    let runtime = Runtime::get();
    let app = runtime.create_application_with_name("hello_world_service").expect("Failed to create server");

    let app_clone = app.clone();
    let app_clone1 = app.clone();
    app.register_state_handler(move |state| {
        if state == State::Registered {
            app_clone.offer_service(SERVICE_ID, INSTANCE_ID, 0, 0);
        }
    });
    
    let mut state = 0;
    app.register_message_handler(SERVICE_ID, INSTANCE_ID, METHOD_ID, move |request| {
        let payload = request.get_payload();
        let bytes = payload.get_data();
        let response_text = std::str::from_utf8(bytes).unwrap();
        println!("{}", response_text);
        
        let mut response = Message::response(request);
        response.set_payload(&Payload::with_data(format!("Hello {}", state).as_bytes()));

        app_clone1.send(&response);

        state += 1;
    });

    app.start();
    
    cleanup(app);
}
```

## Build and Run

The vsomeip library is loaded dynamically at runtime, so it must be present in the dynamic library path when running any vsomeip application. 
This can be done by setting the `LD_LIBRARY_PATH` environment variable.

vsomeip also uses a config file to configure the vsomeip services. The config file's path needs to be set in the `VSOMEIP_CONFIGURATION` environment variable.

To run the example, run the scripts/run_example_server bash script which sets these environment variables appropriately.

```bash
scripts/run_example_server
```
