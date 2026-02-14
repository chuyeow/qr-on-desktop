use anyhow::Result;
use arboard::ImageData;
use image::{DynamicImage, GrayImage, RgbaImage};
use std::path::Path;

pub fn load_file_to_gray(path: &Path) -> Result<GrayImage> {
    Ok(image::open(path)?.to_luma8())
}

pub fn load_clipboard_to_gray() -> Result<GrayImage> {
    let mut clipboard = arboard::Clipboard::new()?;
    let ImageData {
        width,
        height,
        bytes,
    } = clipboard.get_image()?;
    let rgba = RgbaImage::from_raw(width as u32, height as u32, bytes.into_owned())
        .ok_or_else(|| anyhow::anyhow!("invalid clipboard image bytes"))?;
    Ok(DynamicImage::ImageRgba8(rgba).to_luma8())
}
