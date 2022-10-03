//! 🚫 XAudio 2.5 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (August 2009)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

/// Raw low level FFI bindings
///
pub use thindx_xaudio2_sys::xaudio2_5 as sys;
