# smip
A framework for working with someip in rust.

A service can be defined using attributes on function and structs.
For example:

```rust

#[smip::service(id = 0x1234, major_version = 1, minor_version = 0)]
pub struct MyService {
    foo: u32
}

#[smip::methods_impl]
impl MyService {
    #[smip::method(id = 0x5678)]
    pub fn method(&self, arg1: u32, arg2: u32) -> u32 {
        // let service_id = self.serveice_id();
        // let major_version = self.major_version();
        // let minor_version = self.minor_version();

        arg1 + arg2
    }
}
pub fn main() {
    let runtime = Runtime::new()
    .unicast(true)
    .service(MyService::new())
    .build();

    runtime.start();
}
```

The macros desugar to this kind of code:

```rust
pub trait Service: Send {
    fn service_id(&self) -> u16;
    fn major_version(&self) -> u8;
    fn minor_version(&self) -> u8;
    fn register(&mut self, context: &mut Context);
}

impl Service for MyService {
    fn service_id(&self) -> u16 {
        0x1234
    }

    fn major_version(&self) -> u8 {
        1
    }

    fn minor_version(&self) -> u8 {
        0
    }

    fn register(&mut self, context: &mut Context) {
        context.register_method(0x5678, Box::new(|request: smip::Message| {
            let arg1 = request.get_argument::<u32>(0);
            let arg2 = request.get_argument::<u32>(1);
            let result = self.method(arg1, arg2);
            request.respond(&request.create_response(result));
        }));
    }
}
```
