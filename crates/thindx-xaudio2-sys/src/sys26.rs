//! 🚫 XAudio 2.6 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (February 2010)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

use super::xaudio2_5 as prev;
use winapi::shared::guiddef::GUID;

pub const CLSID_XAudio2         : GUID = super::guid(0x3eda9b49, 0x2085, 0x498b, 0x9bb239a6778493de);
pub const CLSID_XAudio2_Debug   : GUID = super::guid(0x47199894, 0x7cc2, 0x444d, 0x9873ced2562cc60e);
pub const IID_IXAudio2          : GUID = prev::IID_IXAudio2; // Might be a different interface despite reusing the GUID
