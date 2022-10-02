//! 🚫 XAudio 2.3 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (November 2008)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

use super::xaudio2_2 as prev;
use winapi::shared::guiddef::GUID;

pub const CLSID_XAudio2         : GUID = super::guid(0x4c5e637a, 0x16c7, 0x4de3, 0x9c465ed22181962d);
pub const CLSID_XAudio2_Debug   : GUID = super::guid(0xef0aa05d, 0x8075, 0x4e5d, 0xbead45be0c3ccbb3);
pub const IID_IXAudio2          : GUID = prev::IID_IXAudio2; // Might be a different interface despite reusing the GUID
