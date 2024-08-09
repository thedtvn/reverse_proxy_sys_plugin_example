use http::{HeaderValue, StatusCode};
use reverse_proxy_sys::{RequestPlugin, ResponsePlugin};

// DO NOT UNWARP OR PANIC IN PLUGIN OR IT WILL CRASH THE PROGRAM
// SO HANDLE ERRORS PROPERLY
// EXAMPLE: 
// BAD WAY: 
// ```rs
// let _ = test_err().unwrap();
// ```
// GOOD WAY: 
// ```rs
// let test = test_err();
// if test.is_err() {
//    return
// }
// let _ = test.unwrap();
// ```

#[no_mangle]
pub fn on_init() {
    // add your init code here
}


#[no_mangle]
pub fn on_request(request: &mut RequestPlugin) {
    // edit request headers
    request.get_parts().headers.insert("test", HeaderValue::from_static("test_plugin_on_request"));
    // edit request forword to
    request.foword_to = Some("127.0.0.1:3000".to_string());
}

#[no_mangle]
pub fn on_response(response: &mut ResponsePlugin) {
    println!("test_plugin_on_response");
    // edit response headers
    response.get_parts().headers.insert("test", HeaderValue::from_static("test_plugin"));
    // edit response status
    if response.get_parts().status == StatusCode::INTERNAL_SERVER_ERROR {
        response.get_parts().status = StatusCode::from_u16(218).unwrap();
    }
}