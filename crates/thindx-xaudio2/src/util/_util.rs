#[allow(unused_imports)] use crate::xaudio2_9::xaudio2::sys::*;
use std::panic::*;



/// Guard against bad behavior in Rust code when running in the context of an XAudio2 thread.
///
/// > It is currently undefined behavior to unwind from Rust code into foreign code
/// > <https://doc.rust-lang.org/std/panic/fn.catch_unwind.html>
///
/// As such, manually catch and abort.  Additionally, if [IXAudio2Voice] or [IXAudio2] ever gains [Send], this should protect against DestroyVoice:
///
/// > It is invalid to call DestroyVoice from within a callback (that is, IXAudio2EngineCallback or IXAudio2VoiceCallback).
/// > <https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice>
pub(crate) fn xaudio2_thread_guard<R>(f: impl FnOnce() -> R + UnwindSafe) -> R {
    match catch_unwind(f) {
        Ok(r) => r,
        Err(panic) => {
            eprintln!(concat!(
                "\u{001B}[31;1mbug\u{001B}[37m:\u{001B}[0m unwinding panic! in a XAudio2 callback / on a XAudio2 thread.\n",
                "Since unwinding across a COM/FFI boundary would be undefined behavior, and there's nothing to catch the panic anyways, this will abort the process.\n",
                "{panic:?}",
            ), panic=panic);
            bugsalot::debugger::break_if_attached();
            std::process::abort();
        },
    }
}
