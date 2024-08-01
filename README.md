# smip
smip aims to make SOME/IP development in Rust feel as natural as building regular web services.

Why is it better than something like vsomeip-rs?

vsomeip-rs is a Rust binding for the vSomeIP C++ library. It exposes the C++ API directly to Rust developers, potentially leading to less Rust-idiomatic code.
Also vSomeIP is a very low level SOME/IP implementation and is heavily callback oriented.
For example a service definition in vSomeIP may look like this:

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
Methods on services are defined using callbacks. While this is fine for simple services, this can get very complicated for services with complex logic, or with many methods. This is where smip can be of use. 

It offers a higher-level abstraction over the underlying protocol, making it easier to use and understand by providing a cleaner more structured way to define services by leveraging Rust's powerful macro system, something similar to [rocket.rs](https://rocket.rs/) but for SOME/IP.

# Example
Let's look at a simple "Hello World" example using smip. 

```rust
use smip::{Runtime, RuntimeConfig, Service};

#[smip::service(id = 0x1234, major_version = 1, minor_version = 0)]
struct MyService {
    x: u32,
}

#[smip::methods_impl]
impl MyService {

    #[smip_method(id = 1)]
    fn add(&mut self, value: u32) -> u32 {
        self.x += value;
        self.x
    }

    #[smip_method(id = 2)]
    fn hello(&self) -> String {
        "Hello World!".to_string()
    }
}

fn main() {
    let config = smip::RuntimeConfig::new("Simple", 0xABCD, 0x1);

    let application = Runtime::new(config).service(
        MyService {
        x: 0
    }, 30509);

    let _ = application.run();
}
```

A service is represented by a struct, `MyService` in this case, with a `service` attribute for providing its `id` and other metadata like the `major_version` (optional) and `minor_version` (optional). This struct will also hold all of the service's state. 

SOME/IP methods are just rust methods with a special `smip_method` attribute to indicate its id. Whatever you pass as an argument to your method is parsed automatically from the payload, and whatever you return from it serialized into a response and sent back.
All of these need to be in a special impl block marked with a `methods_impl` attribute for the framework to recognize them. 

There are two methods

A `Runtime` needs to be created using a `RuntimeConfig` which which take care of creating and running all of your services.

After adding all your services to the `Runtime`, call `runtime.run()` to start all the services.

# Aim

Smip aims to be a SOME/IP framework and not an implementation of SOME/IP, so its not competing with [vSomeIP](https://github.com/COVESA/vsomeip) or [SommR](https://projects.eclipse.org/projects/automotive.sommr). Currently vSomeIP is used as the underlying implementation but this can be swapped with any compliant implementation in the future. 

Key Benefits:
* **Macro-Based Definition**: The smip macro simplifies the definition of services and methods, reducing the amount of code needed.
* **Automatic Serialization/Deserialization**: smip handles the serialization and deserialization of your service's data types, so you don't have to write it yourself.
* **Rust-Idiomatic API**: The framework's API is designed to be Rust-friendly, with a focus on clarity and simplicity.
* **Improved Developer Experience**: smip streamlines the development process, making it easier to create and manage SOME/IP services in Rust.


⚠️ **This is a highly experimental framework and doesn't support all of the features in SOME/IP**

# Run
For a working demo see `examples/simple.rs` and `examples/simple_client.rs`:

To run locally,
```bash
cargo run --example simple
```
In another terminal,
```bash
cargo run --example simple_client
```

- You may need to set the `LD_LIBRARY_PATH` environment to a path that contains the vSomeIP library as this is dynamically loaded `LD_LIBRARY_PATH=/usr/local/lib`


# License

smip is licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0), check LICENSE.md for more information.