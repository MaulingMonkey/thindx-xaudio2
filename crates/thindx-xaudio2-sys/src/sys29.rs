//! ✔️ XAudio 2.9 - Windows 7 SP1+ [via Redist](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable), Windows 10+ via preinstall
//!
//! Introduced in the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#windows-10)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.9 (Windows 10 and redistributable for Windows 7 and Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-29-windows-10-and-redistributable-for-windows-7-and-windows-8x)
//! *   [Developer guide for redistributable version of XAudio 2.9](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable)

//use super::xaudio2_8 as prev;
use abistr::*;
use winapi::shared::guiddef::GUID;

#[doc = "`\"xaudio2_9.dll\"`"] pub const XAUDIO2_DLL        : &'static str              =           "xaudio2_9.dll";
#[doc = "`\"xaudio2_9.dll\"`"] pub const XAUDIO2_DLL_A      : CStrNonNull<'static, u8 > = cstr8!(   "xaudio2_9.dll");
#[doc = "`\"xaudio2_9.dll\"`"] pub const XAUDIO2_DLL_W      : CStrNonNull<'static, u16> = cstr16!(  "xaudio2_9.dll");

#[doc = "`\"xaudio2_9d.dll\"`"] pub const XAUDIO2D_DLL      : &'static str              =           "xaudio2_9d.dll";
#[doc = "`\"xaudio2_9d.dll\"`"] pub const XAUDIO2D_DLL_A    : CStrNonNull<'static, u8 > = cstr8!(   "xaudio2_9d.dll");
#[doc = "`\"xaudio2_9d.dll\"`"] pub const XAUDIO2D_DLL_W    : CStrNonNull<'static, u16> = cstr16!(  "xaudio2_9d.dll");

#[doc = "`2B02E3CF-2E0B-4ec3-BE45-1B2A3FE7210D`"] pub const IID_IXAudio2          : GUID = super::guid(0x2B02E3CF, 0x2E0B, 0x4ec3, 0xBE451B2A3FE7210D);
#[doc = "`84ac29bb-d619-44d2-b197-e4acf7df3ed6`"] pub const IID_IXAudio2Extension : GUID = super::guid(0x84ac29bb, 0xd619, 0x44d2, 0xb197e4acf7df3ed6);
