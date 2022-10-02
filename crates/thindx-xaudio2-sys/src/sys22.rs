//! 🚫 XAudio 2.2 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (August 2008)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

use super::xaudio2_1 as prev;
use winapi::shared::guiddef::GUID;

pub const CLSID_XAudio2         : GUID = super::guid(0xb802058a, 0x464a, 0x42db, 0xbc10b650d6f2586a);
pub const CLSID_XAudio2_Debug   : GUID = super::guid(0x97dfb7e7, 0x5161, 0x4015, 0x87a9c79e6a1952cc);
pub const IID_IXAudio2          : GUID = prev::IID_IXAudio2; // Might be a different interface despite reusing the GUID
