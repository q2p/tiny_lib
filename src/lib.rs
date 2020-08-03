#![no_std]
#![crate_type="lib"]
#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(const_mut_refs)]
#![feature(clamp)]
#![feature(tau_constant)]
#![feature(test)]

pub mod util;
pub mod trig;
pub mod vector;
pub mod matrices;
pub mod prng;
pub mod hasher;