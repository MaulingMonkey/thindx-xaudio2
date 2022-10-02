//! 🚫 XAudio 2.0 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (March 2008)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

// nothing as prev
use winapi::shared::guiddef::GUID;

pub const CLSID_XAudio2         : GUID = super::guid(0xfac23f48, 0x31f5, 0x45a8, 0xb49b5225d61401aa);
pub const CLSID_XAudio2_Debug   : GUID = super::guid(0xfac23f48, 0x31f5, 0x45a8, 0xb49b5225d61401db);
pub const IID_IXAudio2          : GUID = super::guid(0x8bcf1f58, 0x9fe7, 0x4583, 0x8ac6e2adc465c8bb);
