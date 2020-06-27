#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use js_sys::TypeError;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
use secp256k1_sys as ffi;
use secp256k1_sys::CPtr;
use wasm_bindgen::prelude::*;

//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate lazy_static;





// Dummy Node.js Buffer type
// TODO: Replace appropriate type. It might be good to subdivide into `Point`, `Scaler`, etc.
































// We can assume the return value because it's not possible to construct
// an invalid signature from a valid `Message` and `SecretKey`

#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct SECP {
    __private_field: (),
}
#[doc(hidden)]
static SECP: SECP = SECP{__private_field: (),};
impl ::lazy_static::__Deref for SECP {
    type Target = Secp256k1<secp256k1::All>;
    fn deref(&self) -> &Secp256k1<secp256k1::All> {
        #[inline(always)]
        fn __static_ref_initialize() -> Secp256k1<secp256k1::All> {
            Secp256k1::new()
        }
        #[inline(always)]
        fn __stability() -> &'static Secp256k1<secp256k1::All> {
            static LAZY: ::lazy_static::lazy::Lazy<Secp256k1<secp256k1::All>>
             =
                ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for SECP {
    fn initialize(lazy: &Self) { let _ = &**lazy; }
}
const ZERO: [u8; 32] = [0u8; 32];
enum Error {
    BadPrivate,
    BadPoint,
    BadTweak,
    BadHash,
    BadSignature,
    BadExtraData,
}
impl Error {
    fn as_str(&self) -> &str {
        match *self {
            Error::BadPrivate => "Expected Private",
            Error::BadPoint => "Expected Point",
            Error::BadTweak => "Expected Tweak",
            Error::BadHash => "Expected Hash",
            Error::BadSignature => "Expected Signature",
            Error::BadExtraData => "Expected Extra Data (32 bytes)",
        }
    }
}
impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        JsValue::from(TypeError::new(error.as_str()))
    }
}
type JsBuffer = Box<[u8]>;
#[allow(bad_style)]
#[doc = ""]
#[repr(transparent)]
#[allow(clippy :: all)]
pub struct Buffer {
    obj: wasm_bindgen::JsValue,
}
#[allow(bad_style)]
#[allow(clippy :: all)]
const __wbg_generated_const_Buffer: () =
    {
        use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi};
        use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
        use wasm_bindgen::convert::RefFromWasmAbi;
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsValue, JsCast};
        use wasm_bindgen::__rt::core;
        impl WasmDescribe for Buffer {
            fn describe() { JsValue::describe() }
        }
        impl core::ops::Deref for Buffer {
            type Target = wasm_bindgen::JsValue;
            #[inline]
            fn deref(&self) -> &wasm_bindgen::JsValue { &self.obj }
        }
        impl IntoWasmAbi for Buffer {
            type Abi = <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi { self.obj.into_abi() }
        }
        impl OptionIntoWasmAbi for Buffer {
            #[inline]
            fn none() -> Self::Abi { 0 }
        }
        impl <'a> OptionIntoWasmAbi for &'a Buffer {
            #[inline]
            fn none() -> Self::Abi { 0 }
        }
        impl FromWasmAbi for Buffer {
            type Abi = <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi) -> Self {
                Buffer{obj: JsValue::from_abi(js).into(),}
            }
        }
        impl OptionFromWasmAbi for Buffer {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool { *abi == 0 }
        }
        impl <'a> IntoWasmAbi for &'a Buffer {
            type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self) -> Self::Abi { (&self.obj).into_abi() }
        }
        impl RefFromWasmAbi for Buffer {
            type Abi = <JsValue as RefFromWasmAbi>::Abi;
            type Anchor = core::mem::ManuallyDrop<Buffer>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
                core::mem::ManuallyDrop::new(Buffer{obj:
                                                        core::mem::ManuallyDrop::into_inner(tmp).into(),})
            }
        }
        impl From<JsValue> for Buffer {
            #[inline]
            fn from(obj: JsValue) -> Buffer { Buffer{obj: obj.into(),} }
        }
        impl AsRef<JsValue> for Buffer {
            #[inline]
            fn as_ref(&self) -> &JsValue { self.obj.as_ref() }
        }
        impl AsRef<Buffer> for Buffer {
            #[inline]
            fn as_ref(&self) -> &Buffer { self }
        }
        impl From<Buffer> for JsValue {
            #[inline]
            fn from(obj: Buffer) -> JsValue { obj.obj.into() }
        }
        impl JsCast for Buffer {
            fn instanceof(val: &JsValue) -> bool {
                #[cfg(not(all(target_arch = "wasm32",
                              not(target_os = "emscripten"))))]
                unsafe fn __wbg_instanceof_Buffer_9f4dc2eab00bba4b(_: u32)
                 -> u32 {
                    {
                        ::std::rt::begin_panic("cannot check instanceof on non-wasm targets")
                    };
                }
                unsafe {
                    let idx = val.into_abi();
                    __wbg_instanceof_Buffer_9f4dc2eab00bba4b(idx) != 0
                }
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                Buffer{obj: val.into(),}
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const Buffer) }
            }
        }
        ()
    };
impl Buffer {
    #[allow(bad_style)]
    #[doc = ""]
    #[allow(clippy :: all)]
    pub fn from(buffer: JsBuffer) -> Buffer {
        #[cfg(not(all(target_arch = "wasm32",
                      not(target_os = "emscripten"))))]
        unsafe fn __wbg_from_f27af52402e2a065(buffer:
                                                  <JsBuffer as
                                                  wasm_bindgen::convert::IntoWasmAbi>::Abi)
         -> <Buffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(buffer);
            {
                ::std::rt::begin_panic("cannot call wasm-bindgen imported functions on \
                            non-wasm targets")
            };
        }
        unsafe {
            let _ret =
                {
                    let buffer =
                        <JsBuffer as
                            wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer);
                    __wbg_from_f27af52402e2a065(buffer)
                };
            <Buffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
macro_rules! unwrap_or_jsnullres {
    ($ e : expr) =>
    { match $ e { Ok(x) => x, Err(_) => return Ok(JsValue :: NULL), } } ;
}
fn pubkey_from_slice(p: &JsBuffer) -> Result<PublicKey, Error> {
    let plen = p.len();
    if plen != 33 && plen != 65 { return Err(Error::BadPoint); }
    if plen == 33 && p[0] != 2u8 && p[0] != 3u8 {
        return Err(Error::BadPoint);
    }
    if plen == 65 && p[0] != 4u8 { return Err(Error::BadPoint); }
    PublicKey::from_slice(&p).map_err(|_| Error::BadPoint)
}
fn seckey_from_slice(p: &JsBuffer) -> Result<SecretKey, Error> {
    SecretKey::from_slice(&p).map_err(|_| Error::BadPrivate)
}
fn message_from_slice(p: &JsBuffer) -> Result<Message, Error> {
    Message::from_slice(&p).map_err(|_| Error::BadHash)
}
fn signature_from_slice(p: &JsBuffer) -> Result<Signature, Error> {
    Signature::from_compact(&p).map_err(|_| Error::BadSignature)
}
fn is_point_internal(p: &JsBuffer) -> bool {
    match pubkey_from_slice(&p) { Ok(_) => true, Err(_) => false, }
}
fn compare_32_bytes(p: &JsBuffer, q: &[u8; 32]) -> i8 {
    for i in 0..32 {
        if p[i] < q[i] { return -1; } else if p[i] > q[i] { return 1; }
    }
    return 0;
}
fn is_private_internal(x: &JsBuffer) -> bool {
    is_tweak(x) && compare_32_bytes(x, &ZERO) != 0
}
fn is_tweak(tweak: &JsBuffer) -> bool {
    tweak.len() == 32 &&
        compare_32_bytes(tweak, &secp256k1::constants::CURVE_ORDER) == -1
}
fn check_tweak(tweak: &JsBuffer) -> Result<(), Error> {
    if !is_tweak(tweak) { Err(Error::BadTweak) } else { Ok(()) }
}
pub fn is_point(p: JsBuffer) -> bool { is_point_internal(&p) }
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_isPoint(arg0:
                                                       <JsBuffer as
                                                       wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <bool as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            is_point(arg0)
        };
    <bool as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn is_point_compressed(p: JsBuffer) -> Result<bool, JsValue> {
    let has_proper_len = p.len() == 33;
    if !has_proper_len { return Ok(false); }
    if !is_point_internal(&p) { Err(Error::BadPoint)? }
    Ok(has_proper_len)
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_isPointCompressed(arg0:
                                                                 <JsBuffer as
                                                                 wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<bool, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            is_point_compressed(arg0)
        };
    <Result<bool, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn is_private(x: JsBuffer) -> bool { is_private_internal(&x) }
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_isPrivate(arg0:
                                                         <JsBuffer as
                                                         wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <bool as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            is_private(arg0)
        };
    <bool as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn point_add(p_a: JsBuffer, p_b: JsBuffer, compressed: Option<bool>)
 -> Result<JsValue, JsValue> {
    let is_compressed = compressed.unwrap_or(p_a.len() == 33);
    let puba = pubkey_from_slice(&p_a)?;
    let pubb = pubkey_from_slice(&p_b)?;
    let result =
        match puba.combine(&pubb) {
            Ok(x) => x,
            Err(_) => return Ok(JsValue::NULL),
        };
    if is_compressed {
        Ok(JsValue::from(Buffer::from(Box::new(result.serialize()))))
    } else {
        Ok(JsValue::from(Buffer::from(Box::new(result.serialize_uncompressed()))))
    }
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_pointAdd(arg0:
                                                        <JsBuffer as
                                                        wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                    arg1:
                                                        <JsBuffer as
                                                        wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                    arg2:
                                                        <Option<bool> as
                                                        wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<JsValue, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            let arg2 =
                unsafe {
                    <Option<bool> as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg2)
                };
            point_add(arg0, arg1, arg2)
        };
    <Result<JsValue, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn point_add_scalar(p: JsBuffer, tweak: JsBuffer,
                        compressed: Option<bool>)
 -> Result<JsValue, JsValue> {
    let is_compressed = compressed.unwrap_or(p.len() == 33);
    let mut puba = pubkey_from_slice(&p)?;
    check_tweak(&tweak)?;
    match puba.add_exp_assign(&SECP, &tweak) {
        Ok(x) => x,
        Err(_) => return Ok(JsValue::NULL),
    };
    if is_compressed {
        Ok(JsValue::from(Buffer::from(Box::new(puba.serialize()))))
    } else {
        Ok(JsValue::from(Buffer::from(Box::new(puba.serialize_uncompressed()))))
    }
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_pointAddScalar(arg0:
                                                              <JsBuffer as
                                                              wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                          arg1:
                                                              <JsBuffer as
                                                              wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                          arg2:
                                                              <Option<bool> as
                                                              wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<JsValue, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            let arg2 =
                unsafe {
                    <Option<bool> as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg2)
                };
            point_add_scalar(arg0, arg1, arg2)
        };
    <Result<JsValue, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn point_compress(p: JsBuffer, compressed: Option<bool>)
 -> Result<Buffer, JsValue> {
    let is_compressed = compressed.unwrap_or(p.len() == 33);
    let puba = pubkey_from_slice(&p)?;
    if is_compressed {
        Ok(Buffer::from(Box::new(puba.serialize())))
    } else { Ok(Buffer::from(Box::new(puba.serialize_uncompressed()))) }
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_pointCompress(arg0:
                                                             <JsBuffer as
                                                             wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                         arg1:
                                                             <Option<bool> as
                                                             wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<Buffer, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <Option<bool> as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            point_compress(arg0, arg1)
        };
    <Result<Buffer, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn point_from_scalar(d: JsBuffer, compressed: Option<bool>)
 -> Result<Buffer, JsValue> {
    let is_compressed = compressed.unwrap_or(true);
    let sk = seckey_from_slice(&d)?;
    let pk = PublicKey::from_secret_key(&SECP, &sk);
    if is_compressed {
        Ok(Buffer::from(Box::new(pk.serialize())))
    } else { Ok(Buffer::from(Box::new(pk.serialize_uncompressed()))) }
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_pointFromScalar(arg0:
                                                               <JsBuffer as
                                                               wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                           arg1:
                                                               <Option<bool>
                                                               as
                                                               wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<Buffer, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <Option<bool> as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            point_from_scalar(arg0, arg1)
        };
    <Result<Buffer, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn point_multiply(p: JsBuffer, tweak: JsBuffer, compressed: Option<bool>)
 -> Result<JsValue, JsValue> {
    let is_compressed = compressed.unwrap_or(p.len() == 33);
    let mut pubkey = pubkey_from_slice(&p)?;
    check_tweak(&tweak)?;
    match pubkey.mul_assign(&SECP, &tweak) {
        Ok(x) => x,
        Err(_) => return Ok(JsValue::NULL),
    };
    if is_compressed {
        Ok(JsValue::from(Buffer::from(Box::new(pubkey.serialize()))))
    } else {
        Ok(JsValue::from(Buffer::from(Box::new(pubkey.serialize_uncompressed()))))
    }
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_pointMultiply(arg0:
                                                             <JsBuffer as
                                                             wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                         arg1:
                                                             <JsBuffer as
                                                             wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                         arg2:
                                                             <Option<bool> as
                                                             wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<JsValue, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            let arg2 =
                unsafe {
                    <Option<bool> as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg2)
                };
            point_multiply(arg0, arg1, arg2)
        };
    <Result<JsValue, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn private_add(d: JsBuffer, tweak: JsBuffer) -> Result<JsValue, JsValue> {
    let mut sk1 = seckey_from_slice(&d)?;
    check_tweak(&tweak)?;
    match sk1.add_assign(&tweak) {
        Ok(x) => x,
        Err(_) => return Ok(JsValue::NULL),
    };
    let mut key = [0u8; 32];
    key[..32].clone_from_slice(&sk1[..]);
    Ok(JsValue::from(Buffer::from(Box::new(key))))
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_privateAdd(arg0:
                                                          <JsBuffer as
                                                          wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                      arg1:
                                                          <JsBuffer as
                                                          wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<JsValue, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            private_add(arg0, arg1)
        };
    <Result<JsValue, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn private_sub(d: JsBuffer, tweak: JsBuffer) -> Result<JsValue, JsValue> {
    let mut sk1 = seckey_from_slice(&d)?;
    check_tweak(&tweak)?;
    let mut tweak_clone = tweak.clone();
    unsafe {
        {
            match (&ffi::secp256k1_ec_privkey_negate(*SECP.ctx(),
                                                     tweak_clone.as_mut_c_ptr()),
                   &1) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                         "`,\n right: `",
                                                                                         "`"],
                                                                                       &match (&&*left_val,
                                                                                               &&*right_val)
                                                                                            {
                                                                                            (arg0,
                                                                                             arg1)
                                                                                            =>
                                                                                            [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                          ::core::fmt::Debug::fmt),
                                                                                             ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                          ::core::fmt::Debug::fmt)],
                                                                                        }))
                        }
                    }
                }
            }
        };
    }
    match sk1.add_assign(&tweak_clone) {
        Ok(x) => x,
        Err(_) => return Ok(JsValue::NULL),
    };
    let mut key = [0u8; 32];
    key[..32].clone_from_slice(&sk1[..]);
    Ok(JsValue::from(Buffer::from(Box::new(key))))
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_privateSub(arg0:
                                                          <JsBuffer as
                                                          wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                      arg1:
                                                          <JsBuffer as
                                                          wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<JsValue, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            private_sub(arg0, arg1)
        };
    <Result<JsValue, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn sign(hash: JsBuffer, x: JsBuffer) -> Result<Buffer, JsValue> {
    let msg_hash = message_from_slice(&hash)?;
    let pk = seckey_from_slice(&x)?;
    Ok(Buffer::from(Box::new(SECP.sign(&msg_hash, &pk).serialize_compact())))
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_sign(arg0:
                                                    <JsBuffer as
                                                    wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                arg1:
                                                    <JsBuffer as
                                                    wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<Buffer, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            sign(arg0, arg1)
        };
    <Result<Buffer, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn sign_with_entropy(hash: JsBuffer, x: JsBuffer,
                         add_data: Option<JsBuffer>)
 -> Result<Buffer, JsValue> {
    let msg = message_from_slice(&hash)?;
    let sk = seckey_from_slice(&x)?;
    if add_data == None {
        return Ok(Buffer::from(Box::new(SECP.sign(&msg,
                                                  &sk).serialize_compact())));
    }
    let extra_bytes = add_data.unwrap();
    if extra_bytes.len() != 32 { Err(Error::BadExtraData)? }
    let mut ret = ffi::Signature::new();
    unsafe {
        {
            match (&ffi::secp256k1_ecdsa_sign(*SECP.ctx(), &mut ret,
                                              msg.as_c_ptr(), sk.as_c_ptr(),
                                              ffi::secp256k1_nonce_function_rfc6979,
                                              extra_bytes.as_c_ptr() as
                                                  *const ffi::types::c_void),
                   &1) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                         "`,\n right: `",
                                                                                         "`"],
                                                                                       &match (&&*left_val,
                                                                                               &&*right_val)
                                                                                            {
                                                                                            (arg0,
                                                                                             arg1)
                                                                                            =>
                                                                                            [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                          ::core::fmt::Debug::fmt),
                                                                                             ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                          ::core::fmt::Debug::fmt)],
                                                                                        }))
                        }
                    }
                }
            }
        };
    }
    Ok(Buffer::from(Box::new(secp256k1::Signature::from(ret).serialize_compact())))
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_signWithEntropy(arg0:
                                                               <JsBuffer as
                                                               wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                           arg1:
                                                               <JsBuffer as
                                                               wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                           arg2:
                                                               <Option<JsBuffer>
                                                               as
                                                               wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<Buffer, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            let arg2 =
                unsafe {
                    <Option<JsBuffer> as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg2)
                };
            sign_with_entropy(arg0, arg1, arg2)
        };
    <Result<Buffer, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
pub fn verify(hash: JsBuffer, q: JsBuffer, signature: JsBuffer,
              strict: Option<bool>) -> Result<bool, JsValue> {
    let is_strict = strict.unwrap_or(false);
    let pubkey = pubkey_from_slice(&q)?;
    let msg = message_from_slice(&hash)?;
    let mut sig = signature_from_slice(&signature)?;
    if !is_strict { sig.normalize_s(); }
    match SECP.verify(&msg, &sig, &pubkey) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
#[allow(non_snake_case)]
#[allow(clippy :: all)]
pub extern "C" fn __wasm_bindgen_generated_verify(arg0:
                                                      <JsBuffer as
                                                      wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                  arg1:
                                                      <JsBuffer as
                                                      wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                  arg2:
                                                      <JsBuffer as
                                                      wasm_bindgen::convert::FromWasmAbi>::Abi,
                                                  arg3:
                                                      <Option<bool> as
                                                      wasm_bindgen::convert::FromWasmAbi>::Abi)
 -> <Result<bool, JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
    let _ret =
        {
            let arg0 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
                };
            let arg1 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
                };
            let arg2 =
                unsafe {
                    <JsBuffer as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg2)
                };
            let arg3 =
                unsafe {
                    <Option<bool> as
                        wasm_bindgen::convert::FromWasmAbi>::from_abi(arg3)
                };
            verify(arg0, arg1, arg2, arg3)
        };
    <Result<bool, JsValue> as
        wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
