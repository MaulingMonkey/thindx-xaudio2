//! 🚫 XAudio 2.7 - Windows XP? via redist
//!
//! * SDK:      [DirectX SDK (June 2010)]((https://www.microsoft.com/en-us/download/details.aspx?id=6812))
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.7 and earlier (Windows 7)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-27-and-earlier-windows-7)
//! *   [XAudio does not ship with Windows 7](https://stackoverflow.com/a/49524446/953531)
//! *   [Known Issues - XAudio 2.7](https://walbourn.github.io/known-issues-xaudio-2-7/)
//! *   [The Zombie DirectX SDK](https://walbourn.github.io/the-zombie-directx-sdk/)
//! *   [Not So Direct Setup](https://walbourn.github.io/not-so-direct-setup/)

#![deprecated = "Not currently implemented.  Prefer XAudio 2.9, which has redists backported to Windows 7 SP 1."]
#![allow(deprecated)]

use super::xaudio2_6 as prev;
use winapi::shared::guiddef::GUID;

pub const CLSID_XAudio2         : GUID = super::guid(0x5a508685, 0xa254, 0x4fba, 0x9b829a24b00306af);
pub const CLSID_XAudio2_Debug   : GUID = super::guid(0xdb05ea35, 0x0329, 0x4d4b, 0xa53a6dead03d3852);
pub const IID_IXAudio2          : GUID = prev::IID_IXAudio2; // Might be a different interface despite reusing the GUID
