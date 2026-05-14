use anyhow::{Context, Result};
use cli_core::ui::Theme;
use image::{ImageFormat, ImageReader};
use std::fs;
use std::path::{Path, PathBuf};

#[allow(clippy::too_many_arguments)]
pub fn process(
    input: &str,
    output: Option<String>,
    format_str: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    blur: Option<f32>,
    rotate: Option<u32>,
    flip_h: bool,
    flip_v: bool,
) -> Result<()> {
    let input_path = Path::new(input);
    if !input_path.exists() {
        anyhow::bail!("Input path does not exist: {}", input);
    }

    if input_path.is_dir() {
        process_directory(
            input_path, output, format_str, width, height, blur, rotate, flip_h, flip_v,
        )
    } else {
        let output_path = resolve_output_path(input_path, output.as_deref(), format_str.as_deref());
        process_single_file(
            input_path,
            &output_path,
            format_str.as_deref(),
            width,
            height,
            blur,
            rotate,
            flip_h,
            flip_v,
        )?;
        println!(
            "{}",
            Theme::success(&format!(
                "Image successfully saved to {}",
                output_path.display()
            ))
        );
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn process_directory(
    input_dir: &Path,
    output_dir: Option<String>,
    format_str: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    blur: Option<f32>,
    rotate: Option<u32>,
    flip_h: bool,
    flip_v: bool,
) -> Result<()> {
    let out_dir = output_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| input_dir.to_path_buf());

    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).context("Failed to create output directory")?;
    }

    let mut processed_count = 0;
    let mut error_count = 0;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if !["png", "jpg", "jpeg", "webp", "gif", "bmp", "ico", "tiff"].contains(&ext.as_str())
            {
                continue;
            }

            let file_stem = path.file_stem().unwrap_or_default();
            let new_ext = format_str.as_deref().unwrap_or(ext.as_str());
            let mut out_file = out_dir.join(file_stem);
            out_file.set_extension(new_ext);

            // Avoid overwriting input file if output is the same as input
            if out_file == path {
                out_file = out_dir.join(format!(
                    "{}_mod.{}",
                    file_stem.to_string_lossy(),
                    new_ext
                ));
            }

            match process_single_file(
                &path,
                &out_file,
                format_str.as_deref(),
                width,
                height,
                blur,
                rotate,
                flip_h,
                flip_v,
            ) {
                Ok(_) => processed_count += 1,
                Err(e) => {
                    eprintln!(
                        "{}",
                        Theme::error(&format!("Failed to process {}: {}", path.display(), e))
                    );
                    error_count += 1;
                }
            }
        }
    }

    println!(
        "{}",
        Theme::success(&format!(
            "Batch processing complete: {} succeeded, {} failed. Output directory: {}",
            processed_count,
            error_count,
            out_dir.display()
        ))
    );

    Ok(())
}

fn resolve_output_path(
    input_path: &Path,
    output: Option<&str>,
    format_str: Option<&str>,
) -> PathBuf {
    if let Some(out) = output {
        PathBuf::from(out)
    } else {
        let default_ext = format_str.unwrap_or("png");
        input_path.with_extension(default_ext)
    }
}

#[allow(clippy::too_many_arguments)]
fn process_single_file(
    input_path: &Path,
    output_path: &Path,
    format_str: Option<&str>,
    width: Option<u32>,
    height: Option<u32>,
    blur: Option<f32>,
    rotate: Option<u32>,
    flip_h: bool,
    flip_v: bool,
) -> Result<()> {
    println!("Reading image from {}...", input_path.display());
    let img = ImageReader::open(input_path)
        .context("Failed to open image file")?
        .with_guessed_format()
        .context("Failed to guess image format")?
        .decode()
        .context("Failed to decode image")?;

    let mut result_img = img;

    if let (Some(w), Some(h)) = (width, height) {
        println!("Resizing image to {}x{}...", w, h);
        result_img = result_img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);
    } else if let Some(w) = width {
        let h = (result_img.height() as f32 * (w as f32 / result_img.width() as f32)) as u32;
        println!("Resizing image to {}x{}...", w, h);
        result_img = result_img.resize(w, h, image::imageops::FilterType::Lanczos3);
    } else if let Some(h) = height {
        let w = (result_img.width() as f32 * (h as f32 / result_img.height() as f32)) as u32;
        println!("Resizing image to {}x{}...", w, h);
        result_img = result_img.resize(w, h, image::imageops::FilterType::Lanczos3);
    }

    if let Some(sigma) = blur {
        println!("Blurring image with sigma {}...", sigma);
        result_img = result_img.blur(sigma);
    }

    if let Some(deg) = rotate {
        println!("Rotating image by {} degrees...", deg);
        result_img = match deg {
            90 => result_img.rotate90(),
            180 => result_img.rotate180(),
            270 => result_img.rotate270(),
            _ => {
                eprintln!("Warning: Unsupported rotation angle {}, ignoring.", deg);
                result_img
            }
        };
    }

    if flip_h {
        println!("Flipping image horizontally...");
        result_img = result_img.fliph();
    }

    if flip_v {
        println!("Flipping image vertically...");
        result_img = result_img.flipv();
    }

    let out_format = if let Some(fmt) = format_str {
        match fmt.to_lowercase().as_str() {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "webp" => ImageFormat::WebP,
            "gif" => ImageFormat::Gif,
            "bmp" => ImageFormat::Bmp,
            "ico" => ImageFormat::Ico,
            "tiff" => ImageFormat::Tiff,
            _ => anyhow::bail!("Unsupported output format: {}", fmt),
        }
    } else {
        ImageFormat::from_path(output_path).unwrap_or(ImageFormat::Png)
    };

    println!("Saving image to {}...", output_path.display());
    result_img
        .save_with_format(output_path, out_format)
        .context("Failed to save image")?;

    Ok(())
}
