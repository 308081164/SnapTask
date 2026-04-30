use anyhow::{Context, Result};
use image::ImageFormat;
use std::io::Cursor;
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
        Err(anyhow::anyhow!("Window screenshot is not supported on this platform"))
    }
}

/// 压缩图片，quality 范围 1-100
pub fn compress_image(data: &[u8], quality: u8) -> Result<Vec<u8>> {
    let img = image::load_from_memory(data)
        .context("Failed to decode image for compression")?;

    // 转换为 RGB
    let rgb_img = img.to_rgb8();

    // 根据质量计算缩放比例
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
    let new_w = (w as f64 * scale) as u32;
    let new_h = (h as f64 * scale) as u32;

    let resized = image::imageops::resize(
        &rgb_img,
        new_w.max(1),
        new_h.max(1),
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
        new_w.max(1),
        new_h.max(1),
        image::ExtendedColorType::Rgb8,
    ).context("Failed to encode compressed PNG")?;

    Ok(buf.into_inner())
}

// ========== macOS 实现 ==========

#[cfg(target_os = "macos")]
fn capture_screen_macos() -> Result<Vec<u8>> {
    let tmp_path = format!("/tmp/snaptask_screen_{}.png", uuid::Uuid::now_v7());
    let output = Command::new("screencapture")
        .arg("-x")
        .arg(&tmp_path)
        .output()
        .context("Failed to execute screencapture command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        std::fs::remove_file(&tmp_path).ok();
        anyhow::bail!("screencapture failed: {}", stderr);
    }

    let data = std::fs::read(&tmp_path)
        .context("Failed to read screenshot file")?;
    std::fs::remove_file(&tmp_path).ok();

    Ok(data)
}

#[cfg(target_os = "macos")]
fn capture_area_macos(x: i32, y: i32, width: i32, height: i32) -> Result<Vec<u8>> {
    let tmp_path = format!("/tmp/snaptask_area_{}.png", uuid::Uuid::now_v7());
    let rect = format!("{{},{},{},{}}", x, y, width, height);

    let output = Command::new("screencapture")
        .arg("-x")
        .arg("-R")
        .arg(&rect)
        .arg(&tmp_path)
        .output()
        .context("Failed to execute screencapture command for area")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        std::fs::remove_file(&tmp_path).ok();
        anyhow::bail!("screencapture area failed: {}", stderr);
    }

    let data = std::fs::read(&tmp_path)
        .context("Failed to read area screenshot file")?;
    std::fs::remove_file(&tmp_path).ok();

    Ok(data)
}

#[cfg(target_os = "macos")]
fn capture_window_macos() -> Result<Vec<u8>> {
    let tmp_path = format!("/tmp/snaptask_window_{}.png", uuid::Uuid::now_v7());

    let output = Command::new("screencapture")
        .arg("-x")
        .arg("-o")  // 不包含窗口阴影
        .arg("-w")  // 交互式选择窗口（使用当前活动窗口）
        .arg(&tmp_path)
        .output()
        .context("Failed to execute screencapture command for window")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        std::fs::remove_file(&tmp_path).ok();
        anyhow::bail!("screencapture window failed: {}", stderr);
    }

    let data = std::fs::read(&tmp_path)
        .context("Failed to read window screenshot file")?;
    std::fs::remove_file(&tmp_path).ok();

    Ok(data)
}

// ========== Windows 实现 ==========

#[cfg(target_os = "windows")]
fn capture_screen_windows() -> Result<Vec<u8>> {
    let tmp_path = format!("{}\\snaptask_screen_{}.png", std::env::TEMP_DIR, uuid::Uuid::now_v7());
    let tmp_path = tmp_path.replace("/", "\\");

    let ps_script = r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$screen = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$bitmap = New-Object System.Drawing.Bitmap($screen.Width, $screen.Height)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen($screen.Location, [System.Drawing.Point]::Empty, $screen.Size)
$bitmap.Save($args[0], [System.Drawing.Imaging.ImageFormat]::Png)
$graphics.Dispose()
$bitmap.Dispose()
"#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script, &tmp_path])
        .output()
        .context("Failed to execute PowerShell for screenshot")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        std::fs::remove_file(&tmp_path).ok();
        anyhow::bail!("PowerShell screenshot failed: {}", stderr);
    }

    let data = std::fs::read(&tmp_path)
        .context("Failed to read screenshot file")?;
    std::fs::remove_file(&tmp_path).ok();

    Ok(data)
}

#[cfg(target_os = "windows")]
fn capture_area_windows(x: i32, y: i32, width: i32, height: i32) -> Result<Vec<u8>> {
    let tmp_path = format!("{}\\snaptask_area_{}.png", std::env::TEMP_DIR, uuid::Uuid::now_v7());
    let tmp_path = tmp_path.replace("/", "\\");

    let ps_script = format!(
        r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$bitmap = New-Object System.Drawing.Bitmap({}, {})
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen({}, {}, 0, 0, (New-Object System.Drawing.Size({}, {})))
$bitmap.Save('{}', [System.Drawing.Imaging.ImageFormat]::Png)
$graphics.Dispose()
$bitmap.Dispose()
"#,
        width, height, x, y, width, height, tmp_path
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_script])
        .output()
        .context("Failed to execute PowerShell for area screenshot")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        std::fs::remove_file(&tmp_path).ok();
        anyhow::bail!("PowerShell area screenshot failed: {}", stderr);
    }

    let data = std::fs::read(&tmp_path)
        .context("Failed to read area screenshot file")?;
    std::fs::remove_file(&tmp_path).ok();

    Ok(data)
}

#[cfg(target_os = "windows")]
fn capture_window_windows() -> Result<Vec<u8>> {
    let tmp_path = format!("{}\\snaptask_window_{}.png", std::env::TEMP_DIR, uuid::Uuid::now_v7());
    let tmp_path = tmp_path.replace("/", "\\");

    let ps_script = r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$hwnd = [System.Diagnostics.Process]::GetCurrentProcess().MainWindowHandle
# 获取前台窗口
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class Win32 {
    [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hwnd, out RECT rect);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
}
"@
$foreground = [Win32]::GetForegroundWindow()
$rect = New-Object Win32+RECT
[Win32]::GetWindowRect($foreground, [ref]$rect)
$width = $rect.Right - $rect.Left
$height = $rect.Bottom - $rect.Top
$bitmap = New-Object System.Drawing.Bitmap($width, $height)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen($rect.Left, $rect.Top, 0, 0, (New-Object System.Drawing.Size($width, $height)))
$bitmap.Save($args[0], [System.Drawing.Imaging.ImageFormat]::Png)
$graphics.Dispose()
$bitmap.Dispose()
"#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script, &tmp_path])
        .output()
        .context("Failed to execute PowerShell for window screenshot")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        std::fs::remove_file(&tmp_path).ok();
        anyhow::bail!("PowerShell window screenshot failed: {}", stderr);
    }

    let data = std::fs::read(&tmp_path)
        .context("Failed to read window screenshot file")?;
    std::fs::remove_file(&tmp_path).ok();

    Ok(data)
}
