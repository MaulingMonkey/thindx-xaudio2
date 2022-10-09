//! Play a .wav file by passing it directly to XAudio2.
//!
//! ### References
//! *   [WAVE PCM soundfile format](http://soundfile.sapp.org/doc/WaveFormat/)
//! *   [RIFF File Structure](https://johnloomis.org/cpe102/asgn/asgn1/riff.html)
//! *   [Waveform Audio File Format](https://en.wikipedia.org/wiki/WAV)
//! *   [slimdx/source/multimedia/WaveStream.cpp](https://github.com/SlimDX/slimdx/blob/284f3ab1ddadc17b4091bfa7c7b9faed8bf0ded8/source/multimedia/WaveStream.cpp#L98)

#![forbid(unsafe_op_in_unsafe_fn)]

use thindx_xaudio2::xaudio2_9::*;
use thindx_xaudio2::xaudio2_9::xaudio2::*;

use winapi::shared::mmreg::WAVEFORMATEX;

use std::mem::*;



fn main() {
    let mut args = std::env::args_os();
    let _exe = args.next();
    let wav = args.next().expect("expected usage: play-wav.exe \"C:\\Windows\\Media\\notify.wav\" 100%");
    let vol = if let Some(vol) = args.next() {
        let vol = vol.into_string().expect("invalid volume: invalid utf16");
        let vol = vol.strip_suffix("%").unwrap_or(&vol);
        let vol = vol.parse::<f32>().expect("invalid volume: not a number");
        vol / 100.0
    } else {
        1.0
    };

    let wav = std::fs::read(wav).expect("unable to open file");
    let (riff, wave, mut wave_data, _after_wave_data) = riff_chunk_type(&wav).expect("invalid .wav file: unable to read riff chunk header");
    assert!(riff == *b"RIFF", "invalid .wav file: not a RIFF container");
    assert!(wave == *b"WAVE", "invalid .wav file: wrong subtyle");

    let mut fmt     : Option<SourceFormat>  = None;
    let mut data    : Option<&[u8]>         = None;

    while let Some((ty, fmt_data, after)) = riff_chunk(wave_data) {
        match &ty {
            b"fmt " => {
                assert!(fmt.is_none(),          "invalid .wav file: `fmt ` chunk encountered multiple times");
                assert!(fmt_data.len() >= 16,   "invalid .wav file: `fmt ` chunk too small ({} bytes)", fmt_data.len());
                // PCMWAVEFORMAT ~ WAVEFORMATEX - 2 byte cbSize
                let mut waveformat = WAVEFORMATEX::default(); // XXX: this is begging to be made Pod
                let waveformat_bytes = unsafe { std::slice::from_raw_parts_mut(&mut waveformat as *mut _ as *mut _, size_of::<WAVEFORMATEX>()) };
                waveformat_bytes[..16].copy_from_slice(&fmt_data[..16]); // XXX: handle larger types too
                fmt = Some(unsafe { SourceFormat::from_wave_format_ex(waveformat) });
            },
            b"data" => {
                assert!(data.is_none(), "invalid .wav file: `data` chunks encountered multiple times");
                data = Some(fmt_data);
            },
            _ => {
            },
        }
        wave_data = after; // next chunk
    }

    let fmt  = fmt .expect("invalid .wav file: missing `fmt ` chunk");
    let data = data.expect("invalid .wav file: missing `data` chunk");

    mcom::init::mta().expect("unable to initialize XAudio2: failed to initialize COM");

    let xaudio2 = unsafe { xaudio2::create(None, None) };
    let xaudio2 = xaudio2.expect("unable to initialize XAudio2: failed to create main factory");

    let master = xaudio2.create_mastering_voice(xaudio2::DEFAULT_CHANNELS, xaudio2::DEFAULT_SAMPLERATE, 0, (), None, xaudio2::DEFAULT_AUDIO_CATEGORY);
    let master = master.expect("unable to initialize XAudio2: failed to create mastering voice");
    let _ = master.set_volume(vol, 0);

    struct ExitOnBufferEnd;
    impl xaudio2::VoiceCallback for ExitOnBufferEnd {
        type BufferContext = ();
        fn on_buffer_end(&self, _: Self::BufferContext) { std::process::exit(0) }
        fn on_voice_error(&self, _: &Self::BufferContext, error: winresult::HResult) { panic!("{error:?}") }
    }
    let callback = xaudio2::VoiceCallbackWrapper::new(ExitOnBufferEnd);

    let source_voice = xaudio2.create_source_voice_dynamic(&fmt, 0, DEFAULT_FREQ_RATIO, &callback, None, None);
    let source_voice = source_voice.expect("unable to initialize XAudio2: failed to create source voice (bad format for XAudio2?)");

    unsafe { source_voice.submit_source_buffer_blob_unchecked(
        sys::XAUDIO2_END_OF_STREAM,
        Box::new(data.iter().copied().collect::<Vec<u8>>()),
        ..,
        .., LoopCount::NO_LOOP,
        ()
    )}.expect("unable to submit source buffer");

    source_voice.start(0, 0).expect("unable to start source voice");

    // wait for ExitOnBufferEnd::on_buffer_end
    loop { std::thread::sleep(std::time::Duration::from_secs(24*60*60)) }
}


type FOURCC = [u8; 4];

/// Returns: id, chunk data, after
fn riff_chunk(bytes: &[u8]) -> Option<(FOURCC, &[u8], &[u8])> {
    let (id,    bytes) = try_split_fourcc_rest(bytes)?;
    let (size,  bytes) = try_split_usize32le_rest(bytes)?;
    let (chunk, after) = try_split_at(bytes, size)?;
    Some((id, chunk, after))
}

/// Returns: id, type, chunk data, after
fn riff_chunk_type(bytes: &[u8]) -> Option<(FOURCC, FOURCC, &[u8], &[u8])> {
    let (id, chunk, after) = riff_chunk(bytes)?;
    let (ty, chunk) = try_split_fourcc_rest(chunk)?;
    Some((id, ty, chunk, after))
}

fn try_split_at(bytes: &[u8], mid: usize) -> Option<(&[u8], &[u8])> {
    if mid > bytes.len() { return None }
    Some(bytes.split_at(mid))
}

fn try_split_fourcc_rest(bytes: &[u8]) -> Option<(FOURCC, &[u8])> {
    let (fourcc, rest) = try_split_at(bytes, 4)?;
    Some((bytemuck::pod_read_unaligned(fourcc), rest))
}

fn try_split_usize32le_rest(bytes: &[u8]) -> Option<(usize, &[u8])> {
    let (fourcc, rest) = try_split_fourcc_rest(bytes)?;
    Some((u32::from_le_bytes(fourcc) as usize, rest))
}
