use super::*;

use winapi::Interface;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::{REFIID, IsEqualGUID};
use winapi::shared::winerror::HRESULT;
use winapi::um::unknwnbase::{IUnknownVtbl, IUnknown};
use winresult::*;

use std::sync::Arc;
use core::ptr::null_mut;



/// [IXAudio2EngineCallback] in trait form
pub trait EngineCallback : Sized {
    /// Convert `self` into a [mcom::Rc]<[IXAudio2EngineCallback]> suitable for passing to [IXAudio2Ext]::\[[un](IXAudio2Ext::unregister_for_callbacks)\][register_for_callbacks](IXAudio2Ext::register_for_callbacks).
    fn into_com_object(self) -> mcom::Rc<IXAudio2EngineCallback> { EngineCallbackWrapper::new(self) }

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

#[repr(C)] struct EngineCallbackWrapper<EC: EngineCallback> {
    vtbl:       *const IXAudio2EngineCallbackVtbl,
    callbacks:  EC,
}

impl<EC: EngineCallback> EngineCallbackWrapper<EC> {
    fn new(callbacks: EC) -> mcom::Rc<IXAudio2EngineCallback> {
        unsafe { mcom::Rc::from_raw(Arc::into_raw(Arc::new(EngineCallbackWrapper {
            vtbl: &Self::VTBL,
            callbacks
        })) as *mut _) }
    }

    const VTBL : IXAudio2EngineCallbackVtbl = IXAudio2EngineCallbackVtbl {
        base: IUnknownVtbl {
            AddRef:         Self::add_ref,
            Release:        Self::release,
            QueryInterface: Self::query_interface,
        },
        OnProcessingPassStart:  Self::on_processing_pass_start,
        OnProcessingPassEnd:    Self::on_processing_pass_end,
        OnCriticalError:        Self::on_critical_error,
    };

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)\]
    unsafe extern "system" fn add_ref(this: *mut IUnknown) -> u32 {
        let this = this as *const Self;
        unsafe { Arc::increment_strong_count(this) };
        0 // "The method returns the new reference count. This value is intended to be used only for test purposes."
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)\]
    unsafe extern "system" fn release(this: *mut IUnknown) -> u32 {
        let this = this as *const Self;
        unsafe { Arc::decrement_strong_count(this) };
        0 // "The method returns the new reference count. This value is intended to be used only for test purposes."
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))\]
    unsafe extern "system" fn query_interface(this: *mut IUnknown, riid: REFIID, object: *mut *mut c_void) -> HRESULT {
        debug_assert!(!riid.is_null());
        debug_assert!(!object.is_null());

        let riid = unsafe { &*riid };

        // the following do not exist to compare against:
        //  EngineCallbackWrapper::<EC>::uuidof()
        //  EngineCallback::uuidof()
        //  IXAudio2EngineCallback::uuidof()
        if IsEqualGUID(riid, &IUnknown::uuidof()) {
            unsafe { Self::add_ref(this as *mut _) };
            unsafe { *object = this as *mut _ };
            S::OK.into()
        } else {
            unsafe { *object = null_mut() };
            E::NOINTERFACE.into()
        }
    }

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
