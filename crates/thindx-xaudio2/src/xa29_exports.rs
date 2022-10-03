use super::sys::*;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::REFCLSID;
use winapi::um::unknwnbase::IUnknown;
use winresult::HResult;



const X3DAUDIO_HANDLE_BYTESIZE : usize = 20; // TODO: offload to -sys ?
#[allow(non_camel_case_types)] type X3DAUDIO_HANDLE = [u8; X3DAUDIO_HANDLE_BYTESIZE]; // TODO: offload to -sys ?

#[allow(non_camel_case_types)] type X3DAUDIO_LISTENER      = c_void;    // TODO: remove
#[allow(non_camel_case_types)] type X3DAUDIO_EMITTER       = c_void;    // TODO: remove
#[allow(non_camel_case_types)] type X3DAUDIO_DSP_SETTINGS  = c_void;    // TODO: remove



/// `XAudio2_9.dll` exports.
///
/// Based off of `dumpbin /exports C:\Windows\System32\XAudio2_9.dll` on Windows 10.0.19043.2006:
/// ```text
/// ordinal hint RVA      name
///       2    0 0003F5C0 CreateAudioReverb = CreateAudioReverb
///       7    1 0003F640 CreateAudioReverbV2_8 = CreateAudioReverbV2_8
///       3    2 0003F4E0 CreateAudioVolumeMeter = CreateAudioVolumeMeter
///       4    3 00067820 CreateFX = CreateFX
///       5    4 0003EF20 X3DAudioCalculate = X3DAudioCalculate
///       6    5 0003D910 X3DAudioInitialize = X3DAudioInitialize
///       1    6 00003E70 XAudio2Create = XAudio2CreateV2_9
///       8    7 00003E70 XAudio2CreateV2_9 = XAudio2CreateV2_9
///      10    8 00003EB0 XAudio2CreateWithSharedContexts = XAudio2CreateWithSharedContexts
///       9    9 00003E90 XAudio2CreateWithVersionInfo = XAudio2CreateWithVersionInfo
/// ```
#[allow(non_snake_case)] // match original naming instead
#[non_exhaustive] pub struct Exports {
    /// \[<strike>microsoft.com</strike>\]
    /// Implementation detail of [XAudio2CreateReverb].<br>
    /// Unlike [XAudio2CreateReverb], this has no `Flags` parameter (it is unused and discarded.)
    ///
    /// | Argument          | Description   |
    /// | ----------------- | ------------- |
    /// | ppApo             | Output audio effect.  Can be [QueryInterface]d or [mcom::Rc::try_cast]ed to [IXAPO] or [IXAPOParameters].
    ///
    /// [QueryInterface]:       https://learn.microsoft.com/en-us/cpp/atl/queryinterface?view=msvc-170
    /// [IXAPO]:                https://learn.microsoft.com/en-us/windows/win32/api/xapo/nn-xapo-ixapo
    /// [IXAPOParameters]:      https://learn.microsoft.com/en-us/windows/win32/api/xapo/nn-xapo-ixapoparameters
    /// [XAudio2CreateReverb]:  https://learn.microsoft.com/en-us/windows/win32/api/xaudio2fx/nf-xaudio2fx-xaudio2createreverb
    pub CreateAudioReverb:      Option<unsafe extern "stdcall" fn(ppApo: *mut *mut IUnknown) -> HResult>,
    // XAUDIO2FX_STDAPI -> STDAPI -> EXTERN_C HRESULT STDAPICALLTYPE -> __stdcall ?


    /// \[<strike>microsoft.com</strike>\]
    /// Probably has the same signautre as CreateAudioReverb, but neither CreateAudioReverb nor CreateAudioReverbV2_8 alias each other, so I can't confirm.
    #[allow(dead_code)] CreateAudioReverbV2_8: Option<Option<&'static ()>>,


    /// \[<strike>microsoft.com</strike>\]
    /// Implementation detail of [XAudio2CreateVolumeMeter].<br>
    /// Unlike [XAudio2CreateVolumeMeter], this has no `Flags` parameter (it is unused and discarded.)
    ///
    /// | Argument          | Description   |
    /// | ----------------- | ------------- |
    /// | ppApo             | Output audio effect.  Can be [QueryInterface]d or [mcom::Rc::try_cast]ed to [IXAPO] or [IXAPOParameters].
    ///
    /// [QueryInterface]:           https://learn.microsoft.com/en-us/cpp/atl/queryinterface?view=msvc-170
    /// [IXAPO]:                    https://learn.microsoft.com/en-us/windows/win32/api/xapo/nn-xapo-ixapo
    /// [IXAPOParameters]:          https://learn.microsoft.com/en-us/windows/win32/api/xapo/nn-xapo-ixapoparameters
    /// [XAudio2CreateVolumeMeter]: https://learn.microsoft.com/en-us/windows/win32/api/xaudio2fx/nf-xaudio2fx-xaudio2createvolumemeter
    pub CreateAudioVolumeMeter: Option<unsafe extern "stdcall" fn(ppApo: *mut *mut IUnknown) -> HResult>,
    // XAUDIO2FX_STDAPI -> STDAPI -> EXTERN_C HRESULT STDAPICALLTYPE -> __stdcall ?


    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xapofx/nf-xapofx-createfx)\]
    /// Creates an instance of the requested [XAPOFX](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xapofx-overview) effect.
    ///
    /// | Argument          | Description   |
    /// | ----------------- | ------------- |
    /// | clsid             | Effect class ID (e.g. `CLSID_{FXEQ,FXMasteringLimiter,FXReverb,FXEcho,...}`)
    /// | pEffect           | Output audio effect.  Can be [QueryInterface]d or [mcom::Rc::try_cast]ed to [IXAPO] or [IXAPOParameters].
    /// | pInitDat          | Pointer to effect class specific parameter structure (e.g. `FXECHO_PARAMETERS` for `CLSID_FXEcho`.)
    /// | InitDataByteSize  | Size of effect class specific parameter structure (e.g. `size_of::<FXECHO_PARAMETERS>()`.)
    ///
    /// [QueryInterface]:           https://learn.microsoft.com/en-us/cpp/atl/queryinterface?view=msvc-170
    /// [IXAPO]:                    https://learn.microsoft.com/en-us/windows/win32/api/xapo/nn-xapo-ixapo
    /// [IXAPOParameters]:          https://learn.microsoft.com/en-us/windows/win32/api/xapo/nn-xapo-ixapoparameters
    pub CreateFX: Option<unsafe extern "cdecl" fn(clsid: REFCLSID, pEffect: *mut *mut IUnknown, pInitDat: *const c_void, InitDataByteSize: u32) -> HResult>,
    // FX_API_ -> STDAPIV_ -> EXTERN_C STDAPIVCALLTYPE -> __cdecl


    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/x3daudio/nf-x3daudio-x3daudiocalculate)\]
    /// Calculates DSP settings with respect to 3D parameters.
    ///
    /// | Argument          | Description   |
    /// | ----------------- | ------------- |
    /// | Instance          | 3D audio instance handle (acquired with [X3DAudioInitialize])
    /// | pListener         | Represents the point of reception
    /// | pEmitter          | Represents the sound source
    /// | Flags             | `X3DAUDIO_CALCULATE_*`
    /// | pDSPSettings      | Calculation results
    ///
    /// [X3DAudioInitialize]:   https://learn.microsoft.com/en-us/windows/win32/api/x3daudio/nf-x3daudio-x3daudioinitialize
    pub X3DAudioCalculate:      Option<unsafe extern "cdecl" fn(Instance: &X3DAUDIO_HANDLE, pListener: *const X3DAUDIO_LISTENER, pEmitter: *const X3DAUDIO_EMITTER, Flags: u32, pDSPSettings: *mut X3DAUDIO_DSP_SETTINGS)>,
    // X3DAUDIO_API_ -> STDAPIVCALLTYPE -> __cdecl


    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/x3daudio/nf-x3daudio-x3daudioinitialize)\]
    /// Sets all global 3D audio constants.
    ///
    /// | Argument              | Description   |
    /// | --------------------- | ------------- |
    /// | SpeakerChannelMask    | Assignment of channels to speaker positions. This value must not be zero. The only permissible value on Xbox 360 is `SPEAKER_XBOX`.
    /// | SpeedOfSound          | Speed of sound, in user-defined world units per second. Use this value only for doppler calculations. It must be greater than or equal to [f32::MIN_POSITIVE].
    pub X3DAudioInitialize:     Option<unsafe extern "cdecl" fn(SpeakerChannelMask: u32, SpeedOfSound: f32, Instance: &mut X3DAUDIO_HANDLE) -> HResult>,
    // X3DAUDIO_API_ -> STDAPIVCALLTYPE -> __cdecl


    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2create)\]
    /// Creates a new XAudio2 object and returns a pointer to its [IXAudio2] interface.
    ///
    /// | Argument          | Default                       | Description   |
    /// | ----------------- | ----------------------------- | ------------- |
    /// | ppXAudio2         |                               | Returned [IXAudio2]
    /// | Flags             | 0                             | Reserved flags (must be 0)
    /// | XAudio2Processor  | [XAUDIO2_DEFAULT_PROCESSOR]   | What processor(s) to run XAudio2 on.
    pub XAudio2Create: Option<unsafe extern "system" fn(ppXAudio2: *mut *mut IXAudio2, Flags: u32, XAudio2Processor: XAUDIO2_PROCESSOR) -> HResult>,
    // It seems a bit silly for XAudio2Create to be optional.  However, the inline fn wrapper XAudio2Create as found in:
    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\xaudio2.h
    // handles the case where XAudio2Create is not present, so I've chosen to handle it too.


    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2create)\]
    /// Implementation detail.
    /// XAudio2Create aliases this symbol, at least on some versions of `XAudio2_9.dll`.
    /// Not sure if there's any legitimate reason to actually call this instead of XAudio2Create.
    ///
    /// | Argument          | Default                       | Description   |
    /// | ----------------- | ----------------------------- | ------------- |
    /// | ppXAudio2         |                               | Returned [IXAudio2]
    /// | Flags             | 0                             | Reserved flags (must be 0)
    /// | XAudio2Processor  | [XAUDIO2_DEFAULT_PROCESSOR]   | What processor(s) to run XAudio2 on.
    pub XAudio2CreateV2_9: Option<unsafe extern "system" fn(ppXAudio2: *mut *mut IXAudio2, Flags: u32, XAudio2Processor: XAUDIO2_PROCESSOR) -> HResult>,


    /// \[[microsoft.com](https://learn.microsoft.com/en-us/gaming/gdk/_content/gc/reference/audio/xaudio2xbox/functions/xaudio2createwithsharedcontexts)\]
    /// NDAed function requiring ID@Xbox access?
    #[allow(dead_code)] XAudio2CreateWithSharedContexts: Option<Option<&'static ()>>, // type unknown


    /// \[<strike>microsoft.com</strike>\]
    /// Implementation detail of XAudio2Create in XAudio 2.9 as of NTDDI_WIN10_RS5.
    ///
    /// | Argument          | Default                       | Description   |
    /// | ----------------- | ----------------------------- | ------------- |
    /// | ppXAudio2         |                               | Output IXAudio2, assuming things went well.
    /// | Flags             | 0                             | As per XAudio2Create
    /// | XAudio2Processor  | [XAUDIO2_DEFAULT_PROCESSOR]   | What processor(s) to run XAudio2 on.
    /// | ntddiVersion      | `NTDDI_VERSION`               | Highest OS version supported by the WDK being built with.
    pub XAudio2CreateWithVersionInfo: Option<unsafe extern "system" fn(ppXAudio2: *mut *mut IXAudio2, Flags: u32, XAudio2Processor: XAUDIO2_PROCESSOR, ntddiVersion: u32) -> HResult>,
}

impl Exports {
    pub fn from_default_path_cached() -> &'static std::io::Result<Self> {
        lazy_static::lazy_static! { static ref EXPORTS : std::io::Result<Exports> = Exports::from_default_path(); }
        &*EXPORTS
    }

    pub fn from_default_path() -> std::io::Result<Self> {
        Self::from_path(super::DLL)
    }

    pub fn from_path(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let lib = minidl::Library::load(path)?; // TODO: LOAD_LIBRARY_SEARCH_SYSTEM32: https://github.com/MaulingMonkey/minidl/issues/2
        Self::from_minidl_0_1(lib)
    }

    pub fn from_minidl_0_1(lib: minidl::Library) -> std::io::Result<Self> {
        Ok(Self {
            CreateAudioReverb:                  unsafe { lib.sym_opt("CreateAudioReverb\0") },
            CreateAudioReverbV2_8:              unsafe { lib.sym_opt("CreateAudioReverbV2_8\0") },
            CreateAudioVolumeMeter:             unsafe { lib.sym_opt("CreateAudioVolumeMeter\0") },
            CreateFX:                           unsafe { lib.sym_opt("CreateFX\0") },
            X3DAudioCalculate:                  unsafe { lib.sym_opt("X3DAudioCalculate\0") },
            X3DAudioInitialize:                 unsafe { lib.sym_opt("X3DAudioInitialize\0") },
            XAudio2Create:                      unsafe { lib.sym_opt("XAudio2Create\0") },
            XAudio2CreateV2_9:                  unsafe { lib.sym_opt("XAudio2CreateV2_9\0") },
            XAudio2CreateWithSharedContexts:    unsafe { lib.sym_opt("XAudio2CreateWithSharedContexts\0") },
            XAudio2CreateWithVersionInfo:       unsafe { lib.sym_opt("XAudio2CreateWithVersionInfo\0") },
        })
    }
}

#[test] fn exports() {
    let _exports = Exports::from_default_path().unwrap();
}
