#pragma once
#include <vsomeip/application.hpp>
#include <vsomeip/handler.hpp>

// virtual void register_message_handler(service_t _service,
//             instance_t _instance, method_t _method,
//             const message_handler_t &_handler) = 0;


// typedef std::function< void (const std::shared_ptr< message > &) > message_handler_t;
namespace vsomeip_v3 {
    using c_void = void;
    typedef void (*shim_message_handler_t)(std::shared_ptr<vsomeip_v3::message>, c_void*);

    void application_register_message_handler(vsomeip_v3::application& application, vsomeip_v3::service_t _service, vsomeip_v3::instance_t _instance, vsomeip_v3::method_t _method, shim_message_handler_t _handler, c_void* user_data) {
            application.register_message_handler(_service, _instance, _method, [_handler, user_data] (const std::shared_ptr<vsomeip_v3::message>& message) {
                _handler(message, user_data);
            });
    }
}