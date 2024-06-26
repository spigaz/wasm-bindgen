#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaImage)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaImage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaImage;
    #[wasm_bindgen(method, setter = "sizes")]
    fn sizes_shim(this: &MediaImage, val: &str);
    #[wasm_bindgen(method, setter = "src")]
    fn src_shim(this: &MediaImage, val: &str);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &MediaImage, val: &str);
}
#[cfg(web_sys_unstable_apis)]
impl MediaImage {
    #[doc = "Construct a new `MediaImage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(src: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.src(src);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sizes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sizes(&mut self, val: &str) -> &mut Self {
        self.sizes_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `src` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn src(&mut self, val: &str) -> &mut Self {
        self.src_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.type__shim(val);
        self
    }
}
