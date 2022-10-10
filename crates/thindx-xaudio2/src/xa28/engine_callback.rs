use crate::util::xaudio2_thread_guard;
use super::*;

use thindx_xaudio2_sys::FromVtable;
use winresult::*;



/// [IXAudio2EngineCallback] in trait form
pub trait EngineCallback
    : Sized         // required for `wrap`
    + Sync          // all methods are executed on an XAudio thread
    //+ Send        // is not necessary.  IXAudio2EngineCallback isn't an IUnknown, so no IUnknown::Release to worry about.  No other method transfers ownership of the EngineCallback to XAudio2 either.
    //+ 'static     // is not fundamental.  While all methods currently require 'static, a future scoped registration might not.
{
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
    interface:  IXAudio2EngineCallback,
    callbacks:  EC,
}

impl<EC: EngineCallback> core::ops::Deref for EngineCallbackWrapper<EC> {
    type Target = IXAudio2EngineCallback;
    fn deref(&self) -> &IXAudio2EngineCallback {
        // IXAudio2EngineCallback doesn't implement IUnknown, nor other (de)allocation that would require provenance beyond `self`.
        // As such, "fixing" provenance here is fine.  Probably.  Maybe.
        // ref: https://github.com/retep998/winapi-rs/issues/1025
        let this : *const Self = self;
        let _ = sptr::Strict::expose_addr(this);

        &self.interface
    }
}

impl<EC: EngineCallback> EngineCallbackWrapper<EC> {
    pub fn new(callbacks: EC) -> Self { Self { interface: unsafe { IXAudio2EngineCallback::from_vtable(&Self::VTBL) }, callbacks } }

    const VTBL : IXAudio2EngineCallbackVtbl = IXAudio2EngineCallbackVtbl {
        OnProcessingPassStart:  Self::on_processing_pass_start,
        OnProcessingPassEnd:    Self::on_processing_pass_end,
        OnCriticalError:        Self::on_critical_error,
    };

    unsafe extern "system" fn on_processing_pass_start(this: *const IXAudio2EngineCallback) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            this.callbacks.on_processing_pass_start()
        })
    }

    unsafe extern "system" fn on_processing_pass_end(this: *const IXAudio2EngineCallback) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            this.callbacks.on_processing_pass_end()
        })
    }

    unsafe extern "system" fn on_critical_error(this: *const IXAudio2EngineCallback, error: HResult) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            this.callbacks.on_critical_error(error)
        })
    }
}

#[test] fn test() {
    use crate::xaudio2_9::*; // XXX: no xaudio2::create for 2.8 yet
    struct EC;
    impl EngineCallback for EC {
        fn on_processing_pass_start(&self) {}
        fn on_processing_pass_end(&self) {}
        fn on_critical_error(&self, error: HResult) { panic!("{error:?}") }
    }
    let ec = Box::leak(Box::new(EC.wrap()));

    mcom::init::mta().expect("mcom::init::mta");
    let xaudio2 = unsafe { xaudio2::create(None, None) }.expect("xaudio2::create");

    // validate that unregistering never-registered callbacks causes no problems
    xaudio2.unregister_for_callbacks(ec);

    // normal reg + unreg
    xaudio2.register_for_callbacks(ec).expect("register_for_callbacks");
    xaudio2.unregister_for_callbacks(ec);

    // validate double unregister causes no problems
    xaudio2.unregister_for_callbacks(ec);

    // validate double register+unregister causes no problems
    xaudio2.register_for_callbacks(ec).expect("register_for_callbacks");
    xaudio2.register_for_callbacks(ec).expect("register_for_callbacks");
    xaudio2.unregister_for_callbacks(ec);
    xaudio2.unregister_for_callbacks(ec);
    xaudio2.unregister_for_callbacks(ec);
}
