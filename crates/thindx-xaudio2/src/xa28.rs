//! ✔️ XAudio 2.8 - Windows 8+ via preinstall
//!
//! Introduced in the [Windows 8 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#earlier-releases)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.8 (Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-28-windows-8x)
//! *   [XAudio2 and Windows 8](https://walbourn.github.io/xaudio2-and-windows-8/)

pub use thindx_xaudio2_sys::xaudio2_8 as sys;

pub use sys::XAUDIO2_DLL    as DLL;
pub use sys::XAUDIO2_DLL_A  as DLL_A;
pub use sys::XAUDIO2_DLL_W  as DLL_W;

pub use sys::XAUDIO2D_DLL    as D_DLL;
pub use sys::XAUDIO2D_DLL_A  as D_DLL_A;
pub use sys::XAUDIO2D_DLL_W  as D_DLL_W;
