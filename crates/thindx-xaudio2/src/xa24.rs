//! 🚫 XAudio 2.4 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (March 2009)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

/// Raw low level FFI bindings
///
pub use xaudio2_sys::xaudio2_4 as sys;
