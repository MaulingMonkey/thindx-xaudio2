//! ✔️ XAudio 2.9 - Windows 7 SP1+ [via Redist](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable), Windows 10+ via preinstall
//!
//! Introduced in the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/sdk-archive/#windows-10)
//!
//! ### References
//! *   [XAudio2 Versions: XAudio 2.9 (Windows 10 and redistributable for Windows 7 and Windows 8.x)](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-29-windows-10-and-redistributable-for-windows-7-and-windows-8x)
//! *   [Developer guide for redistributable version of XAudio 2.9](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable)

mod xa29_exports; pub use xa29_exports::*;



/// Raw low level FFI bindings
///
pub use xaudio2_sys::xaudio2_9 as sys;

pub use sys::XAUDIO2_DLL    as DLL;
pub use sys::XAUDIO2_DLL_A  as DLL_A;
pub use sys::XAUDIO2_DLL_W  as DLL_W;

pub use sys::XAUDIO2D_DLL    as D_DLL;
pub use sys::XAUDIO2D_DLL_A  as D_DLL_A;
pub use sys::XAUDIO2D_DLL_W  as D_DLL_W;

pub use sys::{
    XAUDIO2_PROCESSOR  as Processor,
    Processor1,
    Processor2,
    Processor3,
    Processor4,
    Processor5,
    Processor6,
    Processor7,
    Processor8,
    Processor9,
    Processor10,
    Processor11,
    Processor12,
    Processor13,
    Processor14,
    Processor15,
    Processor16,
    Processor17,
    Processor18,
    Processor19,
    Processor20,
    Processor21,
    Processor22,
    Processor23,
    Processor24,
    Processor25,
    Processor26,
    Processor27,
    Processor28,
    Processor29,
    Processor30,
    Processor31,
    Processor32,
    XAUDIO2_USE_DEFAULT_PROCESSOR as USE_DEFAULT_PROCESSOR,
};
#[allow(deprecated)] #[doc(hidden)] pub use sys::XAUDIO2_DEFAULT_PROCESSOR as DEFAULT_PROCESSOR;

#[doc(hidden)] pub use sys::IXAudio2; // Might not remain pub

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2create)\] XAudio2Create:
/// Creates a new [IXAudio2] instance, which you can use to start using XAudio2.
///
/// | Argument  | Description |
/// | --------- | ----------- |
/// | flags     | Must be [None].
/// | processor | [Processor1] ..= [Processor32], <strike>[DEFAULT_PROCESSOR],</strike> or [USE_DEFAULT_PROCESSOR].
///
/// ### Example
/// ```
/// let xaudio2 = xaudio2::xaudio2_9::create(None, xaudio2::xaudio2_9::USE_DEFAULT_PROCESSOR).unwrap();
/// let ext = xaudio2.try_cast::<xaudio2::xaudio2_9::sys::IXAudio2Extension>().unwrap();
/// ```
pub fn create(flags: Option<core::convert::Infallible>, processor: Processor) -> Result<mcom::Rc<IXAudio2>, winresult::ErrorHResultOrCode> {
    #![allow(non_snake_case)]

    let exports = match Exports::from_default_path_cached() {
        Ok(e) => e,
        Err(err) => {
            if let Some(code) = err.raw_os_error() {
                return Err(winresult::ErrorHResultOrCode::from(code));
            } else {
                return Err(winresult::ERROR::INVALID_LIBRARY.into())
            }
        }
    };

    let mut xaudio2 = core::ptr::null_mut();
    let _ = flags;
    let flags = 0;
    const NTDDI_VERSION : u32 = 0x0A00000C; // NTDDI_WIN10_NI - see C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\shared\sdkddkver.h

    let hr = if let Some(XAudio2CreateWithVersionInfo) = exports.XAudio2CreateWithVersionInfo {
        unsafe { XAudio2CreateWithVersionInfo(&mut xaudio2, flags, processor, NTDDI_VERSION) }
    } else if let Some(XAudio2Create) = exports.XAudio2Create {
        unsafe { XAudio2Create(&mut xaudio2, flags, processor) }
    } else {
        // The real SDK uses GetLastError() after GetProcAddress fails.
        // I instead hardcode the error code, since `exports` might be a cached copy of XAudio2's exports, loaded long ago.
        winresult::HResultError::from_win32(winresult::ERROR::PROC_NOT_FOUND).into()
    };

    let xaudio2 = unsafe { mcom::Rc::from_raw_opt(xaudio2) };
    hr.succeeded()?;
    let xaudio2 = xaudio2.ok_or(winresult::ERROR::NOINTERFACE)?; // XAudio2Create "succeeded" but gave us a null ptr?
    Ok(xaudio2)
}
