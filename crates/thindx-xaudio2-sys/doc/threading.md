# Thread Safety

XAudio2's thread safety guarantees are underdocumented / implied by documentation stating what *isn't* thread safe.

*   [XAudio2Create](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-xaudio2create) has flags controlling which processor(s) XAudio2 creates worker threads for.

    Ergo, XAudio2 clearly needs to implement a bunch of syncronization code as is.

*   [IXAudio2Voice::DestroyVoice](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice#remarks) notes:

    *   "DestroyVoice waits for the audio processing thread to be idle, so it can take a little while (typically no more than a couple of milliseconds). This is necessary to guarantee that the voice will no longer make any callbacks or read any audio data, so the application can safely free up these resources as soon as the call returns."

        Ergo, ???

    *   "It is invalid to call DestroyVoice from within a callback (that is, IXAudio2EngineCallback or IXAudio2VoiceCallback)."

        This is a strictly weaker limitation than, say, "invalid to call DestroyVoice from a thread other than the one that created the IXAudio2."
        I think `XAudio2 : Sync` if the quoted limitation is enforced (easy enough by reusing existing thread guard checks?)

*   [CXAPOParametersBase::BeginProcess](https://learn.microsoft.com/en-us/windows/win32/api/xapobase/nf-xapobase-cxapoparametersbase-beginprocess) notes:

    "XAPOs must call this method within their IXAPO::Process implementation to access the current process parameters in a thread-safe manner."

    As noted by [How to: Add Run-time Parameter Support to an XAPO](https://learn.microsoft.com/en-us/windows/win32/xaudio2/how-to--add-run-time-parameter-support-to-an-xapo):

    "Adding these methods to IXAPO::Process allows CXAPOParametersBase to keep its copies of the effect parameters in a thread-safe state. Call CXAPOParametersBase::BeginProcess at the beginning of IXAPO::Process, and CXAPOParametersBase::EndProcess at the end of IXAPO::Process."
