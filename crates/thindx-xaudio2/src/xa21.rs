//! 🚫 XAudio 2.1 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (June 2008)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

/// Raw low level FFI bindings
///
pub use xaudio2_sys::xaudio2_1 as sys;
