//! ✔️ XAudio 2.9 - Windows 7 SP1+ [via Redist](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable), Windows 10+ via preinstall
//!
//! Introduced in the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#windows-10)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.9 (Windows 10 and redistributable for Windows 7 and Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-29-windows-10-and-redistributable-for-windows-7-and-windows-8x)
//! *   [Developer guide for redistributable version of XAudio 2.9](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable)

pub use thindx_xaudio2_sys::xaudio2_9 as sys;

pub use sys::XAUDIO2_DLL    as DLL;
pub use sys::XAUDIO2_DLL_A  as DLL_A;
pub use sys::XAUDIO2_DLL_W  as DLL_W;

pub use sys::XAUDIO2D_DLL    as D_DLL;
pub use sys::XAUDIO2D_DLL_A  as D_DLL_A;
pub use sys::XAUDIO2D_DLL_W  as D_DLL_W;
