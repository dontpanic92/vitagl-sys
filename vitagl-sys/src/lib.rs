#![allow(nonstandard_style)]

#[link(name = "vitaGL", kind = "static")]
extern "C" {}

#[link(name = "vitashark", kind = "static")]
extern "C" {}

#[link(name = "SceShaccCg_stub", kind = "static")]
extern "C" {}

#[link(name = "SceShaccCgExt", kind = "static")]
extern "C" {}

#[link(name = "taihen_stub", kind = "static")]
extern "C" {}

#[link(name = "mathneon", kind = "static")]
extern "C" {}

mod bindings;

pub use bindings::*;
