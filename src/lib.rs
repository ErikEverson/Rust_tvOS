use::std::*;
use::core::*;
use::anyhow::*;
use::anyhow::*;
use::arrayvec::*;
use::base64::*;
use::blake3::*;
use::bytes::*;
use::cfg_eval::*;
use::digest::*;
use::macro_rules_attribute::*;
use::paste::*;
use::serde::*;
use::serde_cbor::*;
use::serde_json::*;
use::sha2::*;
use::snap::*;
use::tap::*;
use::thiserror::*;
use::tokio::*;

pub fn foo() {
    println!("Hallo, Rust library here!")
}

#[cfg(any(all(target_os = "linux", not(target_env = "musl")), target_os = "ios", target_os = "tvos"))]
const DEFAULT_ACTIVE_BLE: usize = 3;
#[cfg(not(any(all(target_os = "linux", not(target_env = "musl")), target_os = "ios", target_os = "tvos")))]
const DEFAULT_ACTIVE_BLE: usize = 2;

#[cfg(target_os = "tvos")]
pub fn footv() {
    println!("Hallo, Rust library here!")
}