use crate::util::xaudio2_thread_guard;
#[allow(unused_imports)] use super::*;
use super::xaudio2::sys::*;
use thindx_xaudio2_sys::FromVtable;
use core::ffi::c_void;



/// [IXAudio2VoiceCallback] in trait form
pub trait VoiceCallback : Send + Sync + Sized + 'static {
    type BufferContext : Send + Sync + Sized + 'static;

    /// Convert `self` into a [IXAudio2VoiceCallback] implementation suitable for passing to [IXAudio2Ext::create_source_voice_unchecked]
    fn wrap(self) -> VoiceCallbackWrapper<Self> { VoiceCallbackWrapper::new(self) }

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
    fn on_voice_error(&self, buffer_context: &Self::BufferContext, error: xaudio2::HResult);
}

#[repr(C)] pub struct VoiceCallbackWrapper<VC: VoiceCallback> {
    interface:  IXAudio2VoiceCallback,
    callbacks:  VC,
}

impl<EC: VoiceCallback> core::ops::Deref for VoiceCallbackWrapper<EC> {
    type Target = IXAudio2VoiceCallback;
    fn deref(&self) -> &IXAudio2VoiceCallback {
        // IXAudio2VoiceCallback doesn't implement IUnknown, nor other (de)allocation that would require provenance beyond `self`.
        // As such, "fixing" provenance here is fine.  Probably.  Maybe.
        // ref: https://github.com/retep998/winapi-rs/issues/1025
        let this : *const Self = self;
        let _ = sptr::Strict::expose_addr(this);

        &self.interface
    }
}

impl<VC: VoiceCallback> VoiceCallbackWrapper<VC> {
    pub fn new(callbacks: VC) -> Self { Self { interface: unsafe { IXAudio2VoiceCallback::from_vtable(&Self::VTBL) }, callbacks } }

    const VTBL : IXAudio2VoiceCallbackVtbl = IXAudio2VoiceCallbackVtbl {
        OnVoiceProcessingPassStart: Self::on_voice_processing_pass_start,
        OnVoiceProcessingPassEnd:   Self::on_voice_processing_pass_end,
        OnStreamEnd:                Self::on_stream_end,
        OnBufferStart:              Self::on_buffer_start,
        OnBufferEnd:                Self::on_buffer_end,
        OnLoopEnd:                  Self::on_loop_end,
        OnVoiceError:               Self::on_voice_error,
    };

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onvoiceprocessingpassstart)\]
    unsafe extern "system" fn on_voice_processing_pass_start(this: *const IXAudio2VoiceCallback, bytes_required: u32) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            this.callbacks.on_voice_processing_pass_start(bytes_required)
        })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onvoiceprocessingpassend)\]
    unsafe extern "system" fn on_voice_processing_pass_end(this: *const IXAudio2VoiceCallback) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            this.callbacks.on_voice_processing_pass_end()
        })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onstreamend)\]
    unsafe extern "system" fn on_stream_end(this: *const IXAudio2VoiceCallback) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            this.callbacks.on_stream_end()
        })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onbufferstart)\]
    unsafe extern "system" fn on_buffer_start(this: *const IXAudio2VoiceCallback, buffer_context: *mut c_void) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            let buffer_context = unsafe { &*(buffer_context as *const SourceBuffer<VC::BufferContext>) };
            this.callbacks.on_buffer_start(&buffer_context.context)
        })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onbufferend)\]
    unsafe extern "system" fn on_buffer_end(this: *const IXAudio2VoiceCallback, buffer_context: *mut c_void) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            let buffer_context = *unsafe { Box::from_raw(buffer_context as *mut SourceBuffer<VC::BufferContext>) };
            this.callbacks.on_buffer_end(buffer_context.context);
        })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onloopend)\]
    unsafe extern "system" fn on_loop_end(this: *const IXAudio2VoiceCallback, buffer_context: *mut c_void) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            let buffer_context = unsafe { &*(buffer_context as *const SourceBuffer<VC::BufferContext>) };
            this.callbacks.on_loop_end(&buffer_context.context)
        })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voicecallback-onvoiceerror)\]
    unsafe extern "system" fn on_voice_error(this: *const IXAudio2VoiceCallback, buffer_context: *mut c_void, error: xaudio2::HResult) {
        xaudio2_thread_guard(||{
            let this : &Self = unsafe { &*sptr::from_exposed_addr(sptr::Strict::addr(this)) };
            let buffer_context = unsafe { &*(buffer_context as *const SourceBuffer<VC::BufferContext>) };
            this.callbacks.on_voice_error(&buffer_context.context, error)
        })
    }

}
