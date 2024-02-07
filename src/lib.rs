use::std::*;
use::core::*;
use::anyhow::*;
// use::async::*;
use::async_fn::*;
use::backtrace::*;
use::base64::*;
use::cast::*;
// use::cfg::*;
use::cfg_eval::*;
// use::crossbeam::*;
// use::crossbeam::*;
// use::extension::*;
// use::extern::*;
use::futures::*;
use::lazy_static::*;
use::log::*;
use::macro_rules_attribute::*;
// use::num::*;
use::num_cpus::*;
use::paste::*;
use::serde::*;
use::serde_cbor::*;
use::serde_json::*;
use::static_assertions::*;
use::tap::*;
use::thiserror::*;
use::tracing::*;
use::tracing::*;
use::tracing::*;
use::url::*;
use::with_locals::*;

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