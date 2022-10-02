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

pub use thindx_xaudio2_sys::xaudio2_7 as sys;
