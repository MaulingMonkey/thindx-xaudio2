//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-apis-portal)\]

#![cfg(windows)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub use thindx_xaudio2_sys as sys;

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
