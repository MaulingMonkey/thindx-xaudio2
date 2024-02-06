#![doc = include_str!("../Readme.md")]
#![cfg(windows)]
#![forbid(unsafe_op_in_unsafe_fn)]

#[cfg(doc)] #[path = "../doc/_doc.rs"] pub mod _doc;
pub use thindx_xaudio2_sys as sys;

#[path = "util/_util.rs"] mod util; pub use util::*;
#[path = "xa20.rs"] pub mod xaudio2_0;
#[path = "xa21.rs"] pub mod xaudio2_1;
#[path = "xa22.rs"] pub mod xaudio2_2;
#[path = "xa23.rs"] pub mod xaudio2_3;
#[path = "xa24.rs"] pub mod xaudio2_4;
#[path = "xa25.rs"] pub mod xaudio2_5;
#[path = "xa26.rs"] pub mod xaudio2_6;
#[path = "xa27.rs"] pub mod xaudio2_7;
#[path = "xa28/_xa28.rs"] pub mod xaudio2_8;
#[path = "xa29/_xa29.rs"] pub mod xaudio2_9;
