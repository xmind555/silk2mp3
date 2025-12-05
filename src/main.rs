use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::builder::TypedValueParser;
use clap::Parser;
use silk_rs::decode_silk;
use anyhow::{Context, Result};
use mp3lame_encoder::{Builder, FlushNoGap, MonoPcm, Bitrate, Quality};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // input_path is required now, so remove arg_required_else_help(true)
struct Cli {
    #[arg(
        short, long,
        default_value_t=16000,
        value_parser = clap::builder::PossibleValuesParser::new(["8000", "16000", "24000", "32000", "44100", "48000"])
            .map(|s| s.parse::<u32>().unwrap()),
    )]
    sample_rate: u32,

    /// Input path (.silk file or a directory containing .silk files)
    input_path: String,
}

fn convert_silk_to_mp3(input_file_path: &Path, sample_rate: u32, output_file_path: &Path) -> Result<()> {
    if output_file_path.exists() {
        println!("Output file {:?} already exists, skipping conversion.", output_file_path);
        return Ok(());
    }

    println!("Reading file: {:?}", input_file_path);
    let input_bytes = std::fs::read(input_file_path).context(format!("Failed to read input file: {:?}", input_file_path))?;

    println!("Decoding Silk from {:?}", input_file_path);
    let pcm_bytes = decode_silk(input_bytes, sample_rate as i32).map_err(|e| anyhow::anyhow!("{}", e))?;

    let pcm_samples: Vec<i16> = pcm_bytes
        .chunks_exact(2)
        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    println!("Encoding to MP3 for {:?}", output_file_path);
    let mut mp3_encoder_builder = Builder::new().ok_or_else(|| anyhow::anyhow!("Failed to create MP3 encoder builder"))?;
    mp3_encoder_builder.set_num_channels(1).map_err(|e| anyhow::anyhow!("Failed to set channels: {:?}", e))?;
    mp3_encoder_builder.set_sample_rate(sample_rate).map_err(|e| anyhow::anyhow!("Failed to set sample rate: {:?}", e))?;
    mp3_encoder_builder.set_brate(Bitrate::Kbps128).map_err(|e| anyhow::anyhow!("Failed to set bitrate: {:?}", e))?;
    mp3_encoder_builder.set_quality(Quality::Good).map_err(|e| anyhow::anyhow!("Failed to set quality: {:?}", e))?;

    let mut mp3_encoder = mp3_encoder_builder.build().map_err(|e| anyhow::anyhow!("Failed to build MP3 encoder: {:?}", e))?;

    let mut mp3_out_buffer = Vec::new();
    let estimated_size = mp3lame_encoder::max_required_buffer_size(pcm_samples.len());
    mp3_out_buffer.reserve(estimated_size);

    let input_pcm = MonoPcm(&pcm_samples);

    let encoded_size = mp3_encoder.encode(input_pcm, mp3_out_buffer.spare_capacity_mut())
        .map_err(|_| anyhow::anyhow!("MP3 encoding failed for {:?}", input_file_path))?;

    unsafe {
        mp3_out_buffer.set_len(mp3_out_buffer.len() + encoded_size);
    }

    let flushed_size = mp3_encoder.flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut())
        .map_err(|_| anyhow::anyhow!("MP3 flush failed for {:?}", input_file_path))?;

    unsafe {
        mp3_out_buffer.set_len(mp3_out_buffer.len() + flushed_size);
    }

    let mut file = File::create(&output_file_path).context(format!("Failed to create output file: {:?}", output_file_path))?;
    file.write_all(&mp3_out_buffer).context(format!("Failed to write MP3 data to {:?}", output_file_path))?;

    println!("Converted {:?} to mp3 {:?} successfully!", input_file_path, output_file_path);
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let sample_rate = cli.sample_rate;
    let input_path_str = cli.input_path;
    let path = PathBuf::from(&input_path_str);

    if !path.exists() {
        anyhow::bail!("Input path does not exist: {:?}", path);
    }

    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) != Some("silk") {
            anyhow::bail!("Input file is not a .silk file: {:?}", path);
        }
        let out_file_path = path.with_extension("mp3");
        convert_silk_to_mp3(&path, sample_rate, &out_file_path)?;
    } else if path.is_dir() {
        println!("Processing directory: {:?}", path);
        for entry in WalkDir::new(&path) {
            let entry = entry.context("Failed to read directory entry")?;
            let file_path = entry.path();

            if file_path.is_file() && file_path.extension().and_then(|s| s.to_str()) == Some("silk") {
                let out_file_path = file_path.with_extension("mp3");
                match convert_silk_to_mp3(file_path, sample_rate, &out_file_path) {
                    Ok(_) => {},
                    Err(e) => eprintln!("Error converting {:?}: {}", file_path, e),
                }
            }
        }
        println!("Finished processing directory: {:?}", path);
    } else {
        anyhow::bail!("Input path is neither a file nor a directory: {:?}", path);
    }

    Ok(())
}
