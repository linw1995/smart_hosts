#![allow(dead_code)]
#![allow(unused_imports)]

pub struct Monitor {}

#[cfg(target_os = "macos")]
mod macos;
