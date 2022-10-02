//! 🚫 XAudio 2.4 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (March 2009)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

use super::xaudio2_3 as prev;
use winapi::shared::guiddef::GUID;

pub const CLSID_XAudio2         : GUID = super::guid(0x03219e78, 0x5bc3, 0x44d1, 0xb92ef63d89cc6526);
pub const CLSID_XAudio2_Debug   : GUID = super::guid(0x4256535c, 0x1ea4, 0x4d4b, 0x8ad5f9db762eca9e);
pub const IID_IXAudio2          : GUID = prev::IID_IXAudio2; // Might be a different interface despite reusing the GUID
