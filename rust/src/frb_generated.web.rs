// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.5.

// Section: imports

use super::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::for_generated::wasm_bindgen;
use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: dart2rust

impl<T> CstDecode<Option<T>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
where
    JsValue: CstDecode<T>,
{
    fn cst_decode(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.cst_decode())
    }
}
impl CstDecode<String> for String {
    fn cst_decode(self) -> String {
        self
    }
}
impl CstDecode<crate::api::simple::DeviceConfig>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> crate::api::simple::DeviceConfig {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::simple::DeviceConfig {
            alias: self_.get(0).cst_decode(),
            fingerprint: self_.get(1).cst_decode(),
            device_model: self_.get(2).cst_decode(),
            device_type: self_.get(3).cst_decode(),
        }
    }
}
impl CstDecode<crate::core::model::DeviceInfo>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> crate::core::model::DeviceInfo {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            11,
            "Expected 11 elements, got {}",
            self_.length()
        );
        crate::core::model::DeviceInfo {
            alias: self_.get(0).cst_decode(),
            version: self_.get(1).cst_decode(),
            device_model: self_.get(2).cst_decode(),
            device_type: self_.get(3).cst_decode(),
            fingerprint: self_.get(4).cst_decode(),
            address: self_.get(5).cst_decode(),
            port: self_.get(6).cst_decode(),
            protocol: self_.get(7).cst_decode(),
            download: self_.get(8).cst_decode(),
            announcement: self_.get(9).cst_decode(),
            announce: self_.get(10).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::simple::DiscoverState>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> crate::api::simple::DiscoverState {
        let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::simple::DiscoverState::Discovering(self_.get(1).cst_decode()),
            1 => crate::api::simple::DiscoverState::Done,
            _ => unreachable!(),
        }
    }
}
impl CstDecode<Vec<crate::core::model::DeviceInfo>>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> Vec<crate::core::model::DeviceInfo> {
        self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap()
            .iter()
            .map(CstDecode::cst_decode)
            .collect()
    }
}
impl CstDecode<Vec<u8>> for Box<[u8]> {
    fn cst_decode(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl CstDecode<Option<String>> for Option<String> {
    fn cst_decode(self) -> Option<String> {
        self.map(CstDecode::cst_decode)
    }
}
impl CstDecode<crate::core::model::Progress>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> crate::core::model::Progress {
        let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::core::model::Progress::Idle,
            1 => crate::core::model::Progress::Progress(
                self_.get(1).cst_decode(),
                self_.get(2).cst_decode(),
            ),
            2 => crate::core::model::Progress::Done,
            _ => unreachable!(),
        }
    }
}
impl CstDecode<crate::api::simple::ServerConfig>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> crate::api::simple::ServerConfig {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            6,
            "Expected 6 elements, got {}",
            self_.length()
        );
        crate::api::simple::ServerConfig {
            multicast_addr: self_.get(0).cst_decode(),
            port: self_.get(1).cst_decode(),
            protocol: self_.get(2).cst_decode(),
            download: self_.get(3).cst_decode(),
            announcement: self_.get(4).cst_decode(),
            announce: self_.get(5).cst_decode(),
        }
    }
}
impl CstDecode<String> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl CstDecode<bool> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> bool {
        self.is_truthy()
    }
}
impl CstDecode<i32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<Vec<u8>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> Vec<u8> {
        self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Uint8Array>()
            .to_vec()
            .into()
    }
}
impl CstDecode<crate::api::simple::ServerStatus>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> crate::api::simple::ServerStatus {
        (self.unchecked_into_f64() as i32).cst_decode()
    }
}
impl CstDecode<u16> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> u16 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<u8> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<usize> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> usize {
        self.unchecked_into_f64() as _
    }
}

#[wasm_bindgen]
pub fn dart_fn_deliver_output(
    call_id: i32,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    let message = unsafe {
        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
            ptr_,
            rust_vec_len_,
            data_len_,
        )
    };
    FLUTTER_RUST_BRIDGE_HANDLER.dart_fn_handle_output(call_id, message)
}

#[wasm_bindgen]
pub fn wire_discover(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_discover_impl(port_)
}

#[wasm_bindgen]
pub fn wire_init_server(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    device: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_init_server_impl(port_, device)
}

#[wasm_bindgen]
pub fn wire_listen_discover(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_listen_discover_impl(port_)
}

#[wasm_bindgen]
pub fn wire_listen_progress(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_listen_progress_impl(port_)
}

#[wasm_bindgen]
pub fn wire_server_status(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_server_status_impl(port_)
}

#[wasm_bindgen]
pub fn wire_start_server(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    config: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_start_server_impl(port_, config)
}

#[wasm_bindgen]
pub fn wire_stop_server(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_stop_server_impl(port_)
}
