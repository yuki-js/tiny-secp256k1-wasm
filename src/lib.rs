extern crate wasm_bindgen;

extern crate secp256k1;
use wasm_bindgen::prelude::*;
use js_sys::TypeError;

// Dummy Node.js Buffer type
// TODO: Replace appropriate type. It might be good to subdivide into `Point`, `Scaler`, etc.
//type JsBuffer = Box<[u8]>;
#[wasm_bindgen]
pub struct JsBuffer(Box<[u8]>);

fn jsbuffer_from(buf: Box<[u8]>) -> JsBuffer {
    JsBuffer(buf)
}
fn is_buffer(buf: JsBuffer) -> bool {
    true
}
impl JsBuffer {
    fn to_boxed_slice(self) -> Box<[u8]> {
        self.0
    }
}
#[wasm_bindgen]
pub fn isPoint(p: JsBuffer) -> bool {
    true
}

#[wasm_bindgen]
pub fn isPointCompressed(p: JsBuffer) -> bool {
    true
}

#[wasm_bindgen]
pub fn isPrivate(x: JsBuffer) -> bool {
    is_buffer(x)
}
#[wasm_bindgen]
pub fn pointAdd(pA: JsBuffer, pB: JsBuffer, compressed: Option<bool>) -> Option<JsBuffer> {
    Some(jsbuffer_from(Box::new([0u8])))
}
#[wasm_bindgen]
pub fn pointAddScaler(p: JsBuffer, tweak: JsBuffer, compressed: Option<bool>) -> Option<JsBuffer> {
    Some(jsbuffer_from(Box::new([0u8])))
}
#[wasm_bindgen]
pub fn pointCompress(p: JsBuffer, compressed: Option<bool>) -> JsBuffer {
    jsbuffer_from(Box::new([0u8]))
}
#[wasm_bindgen]
pub fn pointFromScalar(d: JsBuffer, compressed: Option<bool>) -> Option<JsBuffer> {
    Some(jsbuffer_from(Box::new([0u8])))
}
#[wasm_bindgen]
pub fn pointMultiply(p: JsBuffer, tweak: JsBuffer, compressed: Option<bool>) -> Option<JsBuffer> {
    Some(jsbuffer_from(Box::new([0u8])))
}
#[wasm_bindgen]
pub fn privateAdd(d: JsBuffer,  tweak: JsBuffer) -> Option<JsBuffer> {
    Some(jsbuffer_from(Box::new([0u8])))
}
#[wasm_bindgen]
pub fn privateSub(d: JsBuffer,  tweak: JsBuffer) -> Option<JsBuffer> {
    Some(jsbuffer_from(Box::new([0u8])))
}
#[wasm_bindgen]
pub fn sign(_hash: JsBuffer,  _x: JsBuffer) -> Result<JsBuffer, JsValue> {
    let hash = _hash.to_boxed_slice();
    let x = _x.to_boxed_slice();
    if hash.len() != 32 {
        return Err(JsValue::from(TypeError::new("Expected Hash")));
    }
    if x.len() != 32 {
        return Err(JsValue::from(TypeError::new("Expected Private")));
    }
    let msg_hash = secp256k1::Message::parse_slice(&hash).unwrap_throw();
    let pk = secp256k1::SecretKey::parse_slice(&x).unwrap_throw();
    let (sig, _) = secp256k1::sign(&msg_hash, &pk);
    let bytes = sig.serialize();
    Ok(jsbuffer_from(Box::new(bytes)))
}
#[wasm_bindgen]
pub fn signWithEntropy(hash: JsBuffer,  x: JsBuffer, addData: JsValue) -> JsBuffer {
    jsbuffer_from(Box::new([0u8]))
}
#[wasm_bindgen]
pub fn verify(hash: JsBuffer,  q: JsBuffer, signature: JsValue, compressed: Option<bool>) -> bool { // strict flag is not found in js impl (See #5)
    true
}
