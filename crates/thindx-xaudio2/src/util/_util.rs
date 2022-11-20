#[allow(unused_imports)] use crate::xaudio2_9::xaudio2::sys::*;
use std::io::Write;
use std::panic::*;
use std::sync::atomic::*;



/// Disable the use of [std::panic::catch_unwind] to guard against panics unwinding over an FFI boundary in XAudio2 threads executing callbacks.
/// This improves your callstacks such that they include the original `panic!` for your debugger and native crash collection tools,
/// but might *technically* be undefined behavior.
///
/// As such, you are *strongly* discouraged from calling this from a safe fn unless you're either:
/// *   Writing an org-internal non-public crate
/// *   Writing an application, not a library.
///
///  (e.g. you're in a position to easily fix and rerelease if this causes problems)
///
/// ### References
///
/// > It is currently undefined behavior to unwind from Rust code into foreign code
/// >
/// > <https://doc.rust-lang.org/std/panic/fn.catch_unwind.html>
///
/// Although the Rustonomicon disagrees:
///
/// > If an unwinding operation does encounter an ABI boundary that is not permitted to unwind, the behavior depends on the source of the unwinding (Rust panic or a foreign exception):
/// > panic will cause the process to safely abort.
/// >
/// > <https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-unwinding>
///
/// There's also an [RFC](https://github.com/rust-lang/rfcs/blob/master/text/2945-c-unwind-abi.md) and an
/// [entire working group](https://github.com/rust-lang/project-ffi-unwind) tied up in attempting to figure out how to
/// best handle unwinds and FFI.
pub unsafe fn disable_catch_unwind() { CATCH_UNWIND.store(false, Ordering::Relaxed) }

/// Re-enable the use of [std::panic::catch_unwind] (the default behavior.)
///
/// Probably pretty pointless, but might be useful if some jerk in some crate called [disable_catch_unwind] when they shouldn't have.
pub fn enable_catch_unwind() { CATCH_UNWIND.store(true, Ordering::Relaxed) }

static CATCH_UNWIND : AtomicBool = AtomicBool::new(true);



/// Guard against bad behavior in Rust code when running in the context of an XAudio2 thread.
///
/// > It is currently undefined behavior to unwind from Rust code into foreign code
/// > <https://doc.rust-lang.org/std/panic/fn.catch_unwind.html>
///
/// As such, manually catch and abort.  Additionally, if [IXAudio2Voice] or [IXAudio2] ever gains [Send], this should protect against DestroyVoice:
///
/// > It is invalid to call DestroyVoice from within a callback (that is, IXAudio2EngineCallback or IXAudio2VoiceCallback).
/// > <https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice>
///
/// CreateSourceVoice is already protected against, supposedly:
///
/// > It is invalid to call CreateSourceVoice from within a callback (that is, IXAudio2EngineCallback or IXAudio2VoiceCallback).
/// > If you call CreateSourceVoice within a callback, it returns XAUDIO2_E_INVALID_CALL.
/// > <https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice>
pub(crate) fn xaudio2_thread_guard<R>(f: impl FnOnce() -> R + UnwindSafe) -> R {
    if !CATCH_UNWIND.load(Ordering::Relaxed) { return f() }

    let _ = match catch_unwind(f) {
        Ok(r) => return r,
        Err(panic) => || -> std::io::Result<()> {
            let mut stderr = std::io::stderr().lock();

            write!(stderr, concat!(
                "\u{001B}[31;1mbug\u{001B}[37m:\u{001B}[0m Unwinding panic! thrown in a XAudio2 callback / on a XAudio2 thread.\n",
                "     Unwinding across a COM/FFI boundary might be undefined behavior.\n",
                "     For better callstacks, call `enable_catch_unwind` after reading the docs.\n",
                "     XAudio2's thread handles no exceptions, so this will be fatal regardless.\n",
            ))?;

            let panic = if let Some(s) = panic.downcast_ref::<String>() {
                Some(s.as_str())
            } else if let Some(s) = panic.downcast_ref::<&str>() {
                Some(*s)
            } else {
                None
            };

            if let Some(panic) = panic {
                write!(stderr, "\n")?;
                for line in panic.split('\n') {
                    write!(stderr, "     {}\n", line.strip_suffix("\r").unwrap_or(line))?;
                }
            }

            Ok(())
        }(),
    };
    std::process::abort();
}
