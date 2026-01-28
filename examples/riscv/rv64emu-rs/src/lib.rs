#![allow(clippy::struct_field_names)]
#![allow(clippy::non_send_fields_in_send_ty)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::unused_self)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::similar_names)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::manual_let_else)]
#![cfg_attr(not(feature = "std"), no_std)]
#[macro_use]
extern crate alloc;

pub mod config;
pub mod dbg;
pub mod device;
pub mod difftest;
pub mod rv64core;
pub mod rvsim;
pub mod tools;

#[cfg(feature = "rv_debug_trace")]
pub mod trace;

// cfg_if::cfg_if! {
//     if #[cfg(feature = "alloc")] {
//         #[macro_use]
//         extern crate alloc;
//         pub mod device;
//         pub mod difftest;
//         pub mod rv64core;
//         pub mod rvsim;
//         pub mod tools;
//         #[cfg(feature = "rv_debug_trace")]
//         pub mod trace;
//     }
// }
