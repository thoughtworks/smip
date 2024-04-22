#pragma once
#include <vsomeip/application.hpp>
#include <vsomeip/handler.hpp>

using c_void = void;
typedef void (*message_handler_callback_t)(const std::shared_ptr<vsomeip_v3::message>*, c_void*);

void application_register_message_handler(vsomeip_v3::application& application, vsomeip_v3::service_t _service, vsomeip_v3::instance_t _instance, vsomeip_v3::method_t _method, message_handler_callback_t _handler, c_void* user_data) {
    application.register_message_handler(_service, _instance, _method, [=] (const std::shared_ptr<vsomeip_v3::message>& message) {
         _handler(&message, user_data);
    });
}
typedef void (*state_handler_callback_t)(vsomeip_v3::state_type_e, c_void*);

void application_register_state_handler(vsomeip_v3::application& application, state_handler_callback_t _handler, c_void* user_data) {
    application.register_state_handler([=](vsomeip_v3::state_type_e state) {
        _handler(state, user_data);
    });
}

typedef void (*availability_handler_callback_t)(vsomeip_v3::service_t, vsomeip_v3::instance_t, bool, c_void*);

void application_register_availability_handler(vsomeip_v3::application& application, vsomeip_v3::service_t _service, vsomeip_v3::instance_t _instance, availability_handler_callback_t _handler, vsomeip_v3::major_version_t _major, vsomeip_v3::minor_version_t _minor,  c_void* user_data) {
    application.register_availability_handler(_service, _instance, [=](vsomeip_v3::service_t service, vsomeip_v3::instance_t instance, bool is_available) {
        _handler(service, instance, is_available, user_data);
    },
    _major,
    _minor);
}

