use anyhow::{Context, Result};
use cli_core::ui::Theme;
use image::{ImageFormat, ImageReader};
use std::path::{Path, PathBuf};

pub fn process(
    input: &str,
    output: Option<String>,
    format_str: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<()> {
    let input_path = Path::new(input);
    if !input_path.exists() {
        anyhow::bail!("Input file does not exist: {}", input);
    }

    println!("Reading image from {}...", input);
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

    let output_path = if let Some(out) = output {
        PathBuf::from(out)
    } else {
        let default_ext = format_str.as_deref().unwrap_or("png");
        input_path.with_extension(default_ext)
    };

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
        ImageFormat::from_path(&output_path).unwrap_or(ImageFormat::Png)
    };

    println!("Saving image to {}...", output_path.display());
    result_img
        .save_with_format(&output_path, out_format)
        .context("Failed to save image")?;

    println!("{}", Theme::success(&format!("Image successfully saved to {}", output_path.display())));

    Ok(())
}
