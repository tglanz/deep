extern crate deep_core as core;

mod tensor_arithmetic;
mod cpu_backend;

pub use self::cpu_backend::{CpuBackend};