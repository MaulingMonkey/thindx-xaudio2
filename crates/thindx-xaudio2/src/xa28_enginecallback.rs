use super::*;

use winresult::*;



/// [IXAudio2EngineCallback] in trait form
pub trait EngineCallback : Sized {
    /// Convert `self` into a [IXAudio2EngineCallback] implementation suitable for passing to [IXAudio2Ext::register_for_callbacks]
    fn wrap(self) -> EngineCallbackWrapper<Self> { EngineCallbackWrapper::new(self) }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2enginecallback-onprocessingpassstart)\]
    /// Called by XAudio2 just before an audio processing pass begins.
    fn on_processing_pass_start(&self);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2enginecallback-onprocessingpassend)\]
    /// Called just after an audio processing pass ends.
    fn on_processing_pass_end(&self);

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2enginecallback-oncriticalerror)\]
    /// Called in the event of a critical system error which requires XAudio2 to be closed down and restarted.  The error code is given in Error.
    fn on_critical_error(&self, error: HResult);
}

#[repr(C)] pub struct EngineCallbackWrapper<EC: EngineCallback> {
    vtbl:       *const IXAudio2EngineCallbackVtbl,
    callbacks:  EC,
}

impl<EC: EngineCallback> EngineCallbackWrapper<EC> {
    pub fn new(callbacks: EC) -> Self { Self { vtbl: &Self::VTBL, callbacks } }
    pub fn as_interface(&self) -> &IXAudio2EngineCallback { unsafe { core::mem::transmute(self) } }

    const VTBL : IXAudio2EngineCallbackVtbl = IXAudio2EngineCallbackVtbl {
        OnProcessingPassStart:  Self::on_processing_pass_start,
        OnProcessingPassEnd:    Self::on_processing_pass_end,
        OnCriticalError:        Self::on_critical_error,
    };

    unsafe extern "system" fn on_processing_pass_start(this: *const IXAudio2EngineCallback) {
        let this = unsafe { &*(this as *const Self) };
        this.callbacks.on_processing_pass_start()
    }

    unsafe extern "system" fn on_processing_pass_end(this: *const IXAudio2EngineCallback) {
        let this = unsafe { &*(this as *const Self) };
        this.callbacks.on_processing_pass_end()
    }

    unsafe extern "system" fn on_critical_error(this: *const IXAudio2EngineCallback, error: HResult) {
        let this = unsafe { &*(this as *const Self) };
        this.callbacks.on_critical_error(error)
    }
}
