use anyhow::Result;
use qrcode::{QrCode, EcLevel};
use image::Luma;
use std::path::Path;

pub fn generate(text: &str, output: Option<String>, level: &str, size: u32) -> Result<()> {
    let ec_level = match level.to_uppercase().as_str() {
        "L" => EcLevel::L,
        "M" => EcLevel::M,
        "Q" => EcLevel::Q,
        "H" => EcLevel::H,
        _ => anyhow::bail!("Invalid error correction level: {}. Use L, M, Q, or H.", level),
    };

    let code = QrCode::with_error_correction_level(text, ec_level)?;

    if let Some(output_path) = output {
        let path = Path::new(&output_path);
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        match extension.as_deref() {
            Some("png") => {
                let image = code.render::<Luma<u8>>()
                    .max_dimensions(size, size)
                    .build();
                image.save(path).map_err(|e| anyhow::anyhow!("Failed to save PNG: {}", e))?;
                println!("QR code saved to {}", output_path);
            }
            Some("svg") => {
                let svg_string = code.render::<qrcode::render::svg::Color>()
                    .max_dimensions(size, size)
                    .build();
                std::fs::write(path, svg_string).map_err(|e| anyhow::anyhow!("Failed to save SVG: {}", e))?;
                println!("QR code saved to {}", output_path);
            }
            _ => anyhow::bail!("Unsupported output format. Use .png or .svg"),
        }
    } else {
        // Fallback to terminal output
        // Note: qr2term currently doesn't allow specifying EcLevel easily in its simple print_qr API,
        // but we'll use it for the best terminal experience.
        qr2term::print_qr(text).map_err(|e| anyhow::anyhow!("Failed to generate QR code: {}", e))?;
    }

    Ok(())
}
