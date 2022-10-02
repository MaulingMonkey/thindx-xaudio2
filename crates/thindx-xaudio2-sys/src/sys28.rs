//! ✔️ XAudio 2.8 - Windows 8+ via preinstall
//!
//! Introduced in the [Windows 8 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#earlier-releases)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.8 (Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-28-windows-8x)
//! *   [XAudio2 and Windows 8](https://walbourn.github.io/xaudio2-and-windows-8/)

// don't use prev
use abistr::*;
use winapi::shared::guiddef::GUID;

#[doc = "`\"xaudio2_8.dll\"`"] pub const XAUDIO2_DLL    : &'static str              =           "xaudio2_8.dll";
#[doc = "`\"xaudio2_8.dll\"`"] pub const XAUDIO2_DLL_A  : CStrNonNull<'static, u8 > = cstr8!(   "xaudio2_8.dll");
#[doc = "`\"xaudio2_8.dll\"`"] pub const XAUDIO2_DLL_W  : CStrNonNull<'static, u16> = cstr16!(  "xaudio2_8.dll");

// There is no debug variant of xaudio2_8.dll.
// As such, these constants weren't initially defined for xaudio2.h.
// However, these are convenient if targeting 2.8 and 2.9 in the same build.
// As such, define them, but hide them from the documentation.
#[doc(hidden)] pub const XAUDIO2D_DLL   : &'static str              =           "xaudio2_8.dll";
#[doc(hidden)] pub const XAUDIO2D_DLL_A : CStrNonNull<'static, u8 > = cstr8!(   "xaudio2_8.dll");
#[doc(hidden)] pub const XAUDIO2D_DLL_W : CStrNonNull<'static, u16> = cstr16!(  "xaudio2_8.dll");

#[doc = "`60d8dac8-5aa1-4e8e-b597-2f5e2883d484`"] pub const IID_IXAudio2 : GUID = super::guid(0x60d8dac8, 0x5aa1, 0x4e8e, 0xb5972f5e2883d484);
