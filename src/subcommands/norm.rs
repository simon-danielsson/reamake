use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use crate::{
    AUDIO_EXTENSIONS,
    utils::{args::Arguments, messages},
};

pub fn run(args: &Arguments) -> io::Result<()> {
    let mut s = StemNorm::new()?;

    if args.opt_target.exists() {
        s.dir = args.opt_target.clone();
    }

    // check for audio files
    s.scan()?;

    if s.audio_files.is_empty() {
        println!("{}", messages::NO_AUDIO_FOUND);
        return Ok(());
    }

    for f in &s.audio_files {
        println!("Processing: {:?}", f);
        let _ = process_audio::process(f.to_path_buf());
    }

    println!("Finished!");
    Ok(())
}

struct StemNorm {
    dir: PathBuf,
    audio_files: Vec<PathBuf>,
}

impl StemNorm {
    fn new() -> io::Result<Self> {
        Ok(Self {
            dir: env::current_dir()?,
            audio_files: Vec::new(),
        })
    }

    fn scan(&mut self) -> io::Result<()> {
        let dir = self.dir.clone();
        self.scan_dir(&dir)
    }

    fn scan_dir(&mut self, dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                self.scan_dir(&path)?;
            } else if file_type.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if AUDIO_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                        self.audio_files.push(path);
                    }
                }
            }
        }
        Ok(())
    }
}

mod process_audio {

    use hound::{SampleFormat, WavSpec, WavWriter};
    use std::convert::TryInto;
    use std::fs::File;
    use std::path::PathBuf;
    use symphonia::core::audio::{AudioBufferRef, Signal};
    use symphonia::core::codecs::DecoderOptions;
    use symphonia::core::formats::FormatOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;
    use symphonia::default::{get_codecs, get_probe};

    pub fn process(input: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(&input)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = &input.extension() {
            hint.with_extension(&ext.to_string_lossy());
        }

        let probed = get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;
        let mut format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.sample_rate.is_some())
            .expect("No supported audio tracks found");
        let track_id = track.id;
        let sample_rate = track.codec_params.sample_rate.unwrap();
        let mut channels = track.codec_params.channels.unwrap().count();

        let bits_per_sample = track.codec_params.bits_per_sample.unwrap_or(16);
        let sample_format = match track.codec_params.sample_format {
            Some(symphonia::core::sample::SampleFormat::F32) => SampleFormat::Float,
            _ => SampleFormat::Int,
        };

        let mut decoder =
        get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

        let mut samples: Vec<f32> = Vec::new();
        while let Ok(packet) = format.next_packet() {
            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet)? {
                AudioBufferRef::F32(buf) => {
                    channels = buf.spec().channels.count();
                    for frame in 0..buf.frames() {
                        for ch in 0..channels {
                            samples.push(buf.chan(ch)[frame]);
                        }
                    }
                }
                AudioBufferRef::F64(buf) => {
                    channels = buf.spec().channels.count();
                    for frame in 0..buf.frames() {
                        for ch in 0..channels {
                            samples.push(buf.chan(ch)[frame] as f32);
                        }
                    }
                }
                AudioBufferRef::U8(buf) => {
                    channels = buf.spec().channels.count();
                    for frame in 0..buf.frames() {
                        for ch in 0..channels {
                            samples.push((buf.chan(ch)[frame] as f32
                                / 128.0) - 1.0);
                        }
                    }
                }
                AudioBufferRef::U16(buf) => {
                    channels = buf.spec().channels.count();
                    for frame in 0..buf.frames() {
                        for ch in 0..channels {
                            samples.push((buf.chan(ch)[frame] as f32
                                / 32768.0) - 1.0);
                        }
                    }
                }
                AudioBufferRef::U24(buf) => {
                    channels = buf.spec().channels.count();
                    for frame in 0..buf.frames() {
                        for ch in 0..channels {
                            samples.push(buf.chan(ch)[frame].inner()
                                as f32 / 8_388_607.5 - 1.0);
                        }
                    }
                }
                AudioBufferRef::U32(buf) => {
                    channels = buf.spec().channels.count();
                    for frame in 0..buf.frames() {
                        for ch in 0..channels {
                            samples.push((buf.chan(ch)[frame] as f32
                                / 2_147_483_648.0) - 1.0);
                        }
                    }
                }
                AudioBufferRef::S16(buf) => {
                    channels = buf.spec().channels.count();
                    for frame in 0..buf.frames() {
                        for ch in 0..channels {
                            samples.push(buf.chan(ch)[frame] as f32
                                / 32768.0);
                        }
                    }
                }
                AudioBufferRef::S24(buf) => {
                    channels = buf.spec().channels.count();
                    for frame in 0..buf.frames() {
                        for ch in 0..channels {
                            samples.push(buf.chan(ch)[frame].inner()
                                as f32 / 8_388_607.0);
                        }
                    }
                }
                AudioBufferRef::S32(buf) => {
                    channels = buf.spec().channels.count();
                    for frame in 0..buf.frames() {
                        for ch in 0..channels {
                            samples.push(buf.chan(ch)[frame] as f32
                                / 2_147_483_648.0);
                        }
                    }
                }
                _ => {}
            }
        }

        if samples.is_empty() || channels == 0 {
            return Err("No audio samples decoded or channels missing".into());
        }

        let mut folded_samples: Vec<f32> = Vec::new();
        if channels == 2 {
            let frames = samples.len() / 2;
            let mut identical = true;
            let mut left_rms = 0f32;
            let mut right_rms = 0f32;

            for i in 0..frames {
                let l = samples[2 * i];
                let r = samples[2 * i + 1];
                if (l - r).abs() > 1e-6 {
                    identical = false;
                }
                left_rms += l * l;
                right_rms += r * r;
            }

            left_rms = (left_rms / frames as f32).sqrt();
            right_rms = (right_rms / frames as f32).sqrt();

            if identical || left_rms < 1e-6 || right_rms < 1e-6 {
                for i in 0..frames {
                    folded_samples
                        .push((samples[2 * i] + samples[2 * i + 1]) / 2.0);
                }
                channels = 1;
            } else {
                folded_samples = samples.clone();
            }
        } else {
            folded_samples = samples.clone();
        }

        let target_db = -1.0;
        let target_amp = 10f32.powf(target_db / 20.0);
        let peak = folded_samples.iter().map(|s| s.abs()).fold(0.0, f32::max);
        if peak > 0.0 {
            let gain = target_amp / peak;
            for s in folded_samples.iter_mut() {
                *s *= gain;
            }
        }

        let spec = WavSpec {
            channels: channels as u16,
            sample_rate,
            bits_per_sample: bits_per_sample.try_into()?,
            sample_format,
        };
        let mut writer = WavWriter::create(input, spec)?;

        for s in folded_samples.iter() {
            match bits_per_sample {
                8 => {
                    let val = ((s * 127.0) + 128.0).clamp(0.0, 255.0) as i8;
                    writer.write_sample(val)?;
                }
                16 => {
                    let val = (s * 32767.0).clamp(-32768.0, 32767.0) as i16;
                    writer.write_sample(val)?;
                }
                24 => {
                    let val = (s * 8_388_607.0).clamp(-8_388_608.0, 8_388_607.0)
                    as i32;
                    writer.write_sample(val)?;
                }
                32 => {
                    if sample_format == SampleFormat::Float {
                        writer.write_sample(*s)?;
                    } else {
                        let val = (s * 2_147_483_647.0)
                            .clamp(-2_147_483_648.0, 2_147_483_647.0)
                        as i32;
                        writer.write_sample(val)?;
                    }
                }
                _ => return Err("Unsupported bit depth".into()),
            }
        }

        writer.finalize()?;

        Ok(())
    }
}
