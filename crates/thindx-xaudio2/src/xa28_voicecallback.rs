use super::*;

use winapi::Interface;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::{REFIID, IsEqualGUID};
use winapi::shared::winerror::HRESULT;
use winapi::um::unknwnbase::{IUnknownVtbl, IUnknown};
use winresult::*;

use std::sync::Arc;
use core::ptr::null_mut;



/// [IXAudio2VoiceCallback] in trait form
pub trait VoiceCallback : Send + Sync + Sized + 'static {
    type BufferContext : Send + Sync + Sized + 'static;

    /// Convert `self` into a [mcom::Rc]<[IXAudio2VoiceCallback]> suitable for passing to [IXAudio2Ext::create_source_voice_unchecked]
    fn into_com_object(self) -> mcom::Rc<IXAudio2VoiceCallback> { VoiceCallbackWrapper::new(self) }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onvoiceprocessingpassstart)\]
    /// Called just before this voice's processing pass begins.
    fn on_voice_processing_pass_start(&self, bytes_required: u32) { let _ = bytes_required; }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onvoiceprocessingpassend)\]
    /// Called just after this voice's processing pass ends.
    fn on_voice_processing_pass_end(&self) {}

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onstreamend)\]
    /// Called when this voice has just finished playing a buffer stream
    /// (as marked with the [XAUDIO2_END_OF_STREAM] flag on the last buffer).
    fn on_stream_end(&self) {}

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onbufferstart)\]
    /// Called when this voice is about to start processing a new buffer.
    fn on_buffer_start(&self, buffer_context: &Self::BufferContext) { let _ = buffer_context; }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onbufferend)\]
    /// Called when this voice has just finished processing a buffer.
    /// The buffer can now be reused or destroyed.
    fn on_buffer_end(&self, buffer_context: Self::BufferContext) { let _ = buffer_context; }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onloopend)\]
    /// Called when this voice has just reached the end position of a loop.
    fn on_loop_end(&self, buffer_context: &Self::BufferContext) { let _ = buffer_context; }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onvoiceerror)\]
    /// Called in the event of a critical error during voice processing,
    /// such as a failing xAPO or an error from the hardware XMA decoder.
    /// The voice may have to be destroyed and re-created to recover from
    /// the error.  The callback arguments report which buffer was being
    /// processed when the error occurred, and its HRESULT code.
    fn on_voice_error(&self, buffer_context: &Self::BufferContext, error: HResult);
}

#[repr(C)] struct VoiceCallbackWrapper<VC: VoiceCallback> {
    vtbl:       *const IXAudio2VoiceCallbackVtbl,
    callbacks:  VC,
}

impl<VC: VoiceCallback> VoiceCallbackWrapper<VC> {
    fn new(callbacks: VC) -> mcom::Rc<IXAudio2VoiceCallback> {
        unsafe { mcom::Rc::from_raw(Arc::into_raw(Arc::new(VoiceCallbackWrapper {
            vtbl: &Self::VTBL,
            callbacks
        })) as *mut _) }
    }

    const VTBL : IXAudio2VoiceCallbackVtbl = IXAudio2VoiceCallbackVtbl {
        base: IUnknownVtbl {
            AddRef:                 Self::add_ref,
            Release:                Self::release,
            QueryInterface:         Self::query_interface,
        },
        OnVoiceProcessingPassStart: Self::on_voice_processing_pass_start,
        OnVoiceProcessingPassEnd:   Self::on_voice_processing_pass_end,
        OnStreamEnd:                Self::on_stream_end,
        OnBufferStart:              Self::on_buffer_start,
        OnBufferEnd:                Self::on_buffer_end,
        OnLoopEnd:                  Self::on_loop_end,
        OnVoiceError:               Self::on_voice_error,
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
        //  VoiceCallbackWrapper::<VC>::uuidof()
        //  VoiceCallback::uuidof()
        //  IXAudio2VoiceCallback::uuidof()
        if IsEqualGUID(riid, &IUnknown::uuidof()) {
            unsafe { Self::add_ref(this as *mut _) };
            unsafe { *object = this as *mut _ };
            S::OK.into()
        } else {
            unsafe { *object = null_mut() };
            E::NOINTERFACE.into()
        }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onvoiceprocessingpassstart)\]
    unsafe extern "system" fn on_voice_processing_pass_start(this: *const IXAudio2VoiceCallback, bytes_required: u32) {
        let this = unsafe { &*(this as *const Self) };
        this.callbacks.on_voice_processing_pass_start(bytes_required)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onvoiceprocessingpassend)\]
    unsafe extern "system" fn on_voice_processing_pass_end(this: *const IXAudio2VoiceCallback) {
        let this = unsafe { &*(this as *const Self) };
        this.callbacks.on_voice_processing_pass_end()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onstreamend)\]
    unsafe extern "system" fn on_stream_end(this: *const IXAudio2VoiceCallback) {
        let this = unsafe { &*(this as *const Self) };
        this.callbacks.on_stream_end()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onbufferstart)\]
    unsafe extern "system" fn on_buffer_start(this: *const IXAudio2VoiceCallback, buffer_context: *mut c_void) {
        let this = unsafe { &*(this as *const Self) };
        let buffer_context = unsafe { &*(buffer_context as *const VC::BufferContext) };
        this.callbacks.on_buffer_start(buffer_context)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onbufferend)\]
    unsafe extern "system" fn on_buffer_end(this: *const IXAudio2VoiceCallback, buffer_context: *mut c_void) {
        let this = unsafe { &*(this as *const Self) };
        let buffer_context = *unsafe { Box::from_raw(buffer_context as *mut VC::BufferContext) };
        this.callbacks.on_buffer_end(buffer_context)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onloopend)\]
    unsafe extern "system" fn on_loop_end(this: *const IXAudio2VoiceCallback, buffer_context: *mut c_void) {
        let this = unsafe { &*(this as *const Self) };
        let buffer_context = unsafe { &*(buffer_context as *const VC::BufferContext) };
        this.callbacks.on_loop_end(buffer_context)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onvoiceerror)\]
    unsafe extern "system" fn on_voice_error(this: *const IXAudio2VoiceCallback, buffer_context: *mut c_void, error: HResult) {
        let this = unsafe { &*(this as *const Self) };
        let buffer_context = unsafe { &*(buffer_context as *const VC::BufferContext) };
        this.callbacks.on_voice_error(buffer_context, error)
    }

}