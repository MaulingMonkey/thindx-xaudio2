//! 🚫 XAudio 2.5 - Windows XP? via redist
//!
//! Introduced in the legacy DirectX SDK (August 2009)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

use super::xaudio2_4 as prev;
use winapi::shared::guiddef::GUID;

pub const CLSID_XAudio2         : GUID = super::guid(0x4c9b6dde, 0x6809, 0x46e6, 0xa2789b6a97588670);
pub const CLSID_XAudio2_Debug   : GUID = super::guid(0x715bdd1a, 0xaa82, 0x436b, 0xb0fa6acea39bd0a1);
pub const IID_IXAudio2          : GUID = prev::IID_IXAudio2; // Might be a different interface despite reusing the GUID
