use anyhow::{Context, Result};
use image::ImageEncoder;
use std::io::Cursor;

#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::process::Command;

/// 全屏截屏，返回 PNG 编码的 Vec<u8>
pub fn capture_screen() -> Result<Vec<u8>> {
    #[cfg(target_os = "macos")]
    {
        capture_screen_macos()
    }
    #[cfg(target_os = "windows")]
    {
        capture_screen_windows()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err(anyhow::anyhow!("Screenshot capture is not supported on this platform"))
    }
}

/// 区域截屏，返回 PNG 编码的 Vec<u8>
pub fn capture_area(_x: i32, _y: i32, _width: i32, _height: i32) -> Result<Vec<u8>> {
    #[cfg(target_os = "macos")]
    {
        capture_area_macos(_x, _y, _width, _height)
    }
    #[cfg(target_os = "windows")]
    {
        capture_area_windows(_x, _y, _width, _height)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err(anyhow::anyhow!("Area screenshot is not supported on this platform"))
    }
}

/// 当前窗口截屏，返回 PNG 编码的 Vec<u8>
pub fn capture_window() -> Result<Vec<u8>> {
    #[cfg(target_os = "macos")]
    {
        capture_window_macos()
    }
    #[cfg(target_os = "windows")]
    {
        capture_window_windows()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err(anyhow::anyhow!("Window screenshot capture is not supported on this platform"))
    }
}

/// 压缩图片，质量范围 1-100
pub fn compress_image(data: &[u8], quality: u8) -> Result<Vec<u8>> {
    let img = image::load_from_memory(data)
        .context("Failed to decode image for compression")?;
    let rgb_img = img.to_rgb8();
    let scale = if quality < 30 {
        0.5
    } else if quality < 60 {
        0.7
    } else if quality < 80 {
        0.85
    } else {
        1.0
    };
    let (w, h) = rgb_img.dimensions();
    let new_w = (w as f64 * scale).max(1.0) as u32;
    let new_h = (h as f64 * scale).max(1.0) as u32;
    let resized = image::imageops::resize(
        &rgb_img,
        new_w,
        new_h,
        image::imageops::FilterType::Lanczos3,
    );
    let mut buf = Cursor::new(Vec::new());
    let encoder = image::codecs::png::PngEncoder::new_with_quality(
        &mut buf,
        image::codecs::png::CompressionType::Fast,
        image::codecs::png::FilterType::Adaptive,
    );
    encoder.write_image(
        resized.as_raw(),
        new_w,
        new_h,
        image::ExtendedColorType::Rgb8,
    ).context("Failed to encode compressed PNG")?;
    Ok(buf.into_inner())
}
