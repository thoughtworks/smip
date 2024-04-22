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
fn cleanup(app: Application) {
    app.unregister_state_handler();
    app.unregister_message_handler(SERVICE_ID, INSTANCE_ID, METHOD_ID);
    app.clear_all_handlers();
    app.release_service(SERVICE_ID, INSTANCE_ID);
    app.stop();
}
fn client() {
    let runtime = Runtime::get();
    let app = runtime.create_application_with_name("hello_world_client").expect("Failed to create client");

    let app_clone = app.clone();
    let app_clone1 = app.clone();
    app.register_state_handler(move |state| {
        if state == State::Registered {
            app_clone.request_service(SERVICE_ID, INSTANCE_ID, 0, 0);
        }
    });
    
    app.register_message_handler(ANY_SERVICE, INSTANCE_ID, ANY_METHOD, move |request| {
        if request.get_service() == SERVICE_ID && 
        request.get_instance() == INSTANCE_ID && 
        request.get_message_type() == MessageType::Response && 
        request.get_return_code() == ReturnCode::Ok {
            let payload = request.get_payload();
            let bytes = payload.get_data();
            let response_text = std::str::from_utf8(bytes).unwrap();
            println!("{}", response_text);
        }
    });

    app.register_availability_handler(SERVICE_ID, INSTANCE_ID, move |service_id, instance_id, is_available| {
        if service_id == SERVICE_ID && instance_id == INSTANCE_ID && is_available {
            let mut message = Message::request(true);
            message.set_service(service_id);
            message.set_instance(instance_id);
            message.set_method(METHOD_ID);

            let payload = Payload::with_data("World".as_bytes());
            message.set_payload(&payload);

            app_clone1.send(&message);
        }
    }, 0, 0);
    app.start();
    
    cleanup(app);
}
pub fn main() {
    let args: Vec<_> = std::env::args().collect();
    
    if args[1] == "server" {
        server();
    } else {
        client();
    }

}