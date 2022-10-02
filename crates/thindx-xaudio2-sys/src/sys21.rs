//! 🚫 XAudio 2.1 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (June 2008)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

use super::xaudio2_0 as prev;
use winapi::shared::guiddef::GUID;

pub const CLSID_XAudio2         : GUID = super::guid(0xe21a7345, 0xeb21, 0x468e, 0xbe50804db97cf708);
pub const CLSID_XAudio2_Debug   : GUID = super::guid(0xf7a76c21, 0x53d4, 0x46bb, 0xac538b459cae46bd);
pub const IID_IXAudio2          : GUID = prev::IID_IXAudio2; // Might be a different interface despite reusing the GUID
