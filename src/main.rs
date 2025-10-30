use std::path::{Path, PathBuf};
use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use image::imageops::FilterType;
use image::GenericImageView;
use image::io::Reader as ImageReader;
use rayon::prelude::*;
use walkdir::WalkDir;

/// Simple CLI to upscale all JPG/JPEG images in a folder by 4x.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Folder to scan for images (recursive)
    #[arg(value_name = "FOLDER")]
    folder: PathBuf,

    /// Number of threads to use (defaults to number of logical CPUs)
    #[arg(short, long)]
    threads: Option<usize>,
}

fn find_jpgs(folder: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(folder).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
            let ext = ext.to_lowercase();
            if ext == "jpg" || ext == "jpeg" {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    files
}

fn make_output_dir(input_folder: &Path) -> Result<PathBuf> {
    let out = input_folder.join("4x");
    if !out.exists() {
        fs::create_dir_all(&out).with_context(|| format!("creating output dir: {}", out.display()))?;
    }
    Ok(out)
}

fn upscale_image_file(input: &Path, out_dir: &Path) -> Result<()> {
    // Try to open and decode the image by guessing the format from file contents.
    // This helps when extensions are wrong or files have alternate container formats.
    let reader = ImageReader::open(input).with_context(|| format!("opening image {}", input.display()));
    let img = match reader {
        Ok(r) => match r.with_guessed_format() {
            Ok(r2) => r2.decode().with_context(|| format!("decoding image {}", input.display())),
            Err(_) => {
                // If guessing fails, try reading file bytes and loading from memory as a fallback.
                let bytes = fs::read(input).with_context(|| format!("reading bytes {}", input.display()))?;
                image::load_from_memory(&bytes)
                    .with_context(|| format!("loading image from memory {}", input.display()))
            }
        },
        Err(e) => {
            // If opening the file failed (permissions, not a file, etc.), return the error.
            return Err(e);
        }
    }?;
    let (w, h) = img.dimensions();
    let new_w = w.saturating_mul(4);
    let new_h = h.saturating_mul(4);

    // Use high-quality Lanczos3 filter for resizing.
    let resized = img.resize_exact(new_w, new_h, FilterType::Lanczos3);

    let filename = input
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("invalid filename for {}", input.display()))?;
    let out_path = out_dir.join(filename);
    resized
        .save(&out_path)
        .with_context(|| format!("saving image to {}", out_path.display()))?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(n) = args.threads {
        rayon::ThreadPoolBuilder::new().num_threads(n).build_global().ok();
    }

    let folder = args.folder;
    if !folder.exists() {
        return Err(anyhow::anyhow!("folder does not exist: {}", folder.display()));
    }

    println!("Scanning '{}' for jpg/jpeg images...", folder.display());
    let images = find_jpgs(&folder);
    println!("Found {} image(s)", images.len());

    if images.is_empty() {
        println!("Nothing to do.");
        return Ok(());
    }

    let out_dir = make_output_dir(&folder)?;

    // Process images in parallel.
    images.par_iter().for_each(|p| {
        match upscale_image_file(p, &out_dir) {
            Ok(()) => println!("Upscaled: {}", p.display()),
            Err(e) => eprintln!("Failed {}: {:?}", p.display(), e),
        }
    });

    println!("All done. Upscaled images in: {}", out_dir.display());
    Ok(())
}
