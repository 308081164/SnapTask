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

// ==================== macOS 实现 ====================

#[cfg(target_os = "macos")]
fn capture_screen_macos() -> Result<Vec<u8>> {
    let output = Command::new("screencapture")
        .arg("-x")
        .arg("-C")  // 不播放声音
        .output()
        .context("Failed to execute screencapture")?;
    if !output.status.success() {
        anyhow::bail!("screencapture failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(output.stdout)
}

#[cfg(target_os = "macos")]
fn capture_area_macos(x: i32, y: i32, width: i32, height: i32) -> Result<Vec<u8>> {
    let rect = format!("{},{},{},{}", x, y, width, height);
    let output = Command::new("screencapture")
        .arg("-R")
        .arg(&rect)
        .arg("-x")
        .output()
        .context("Failed to execute screencapture for area")?;
    if !output.status.success() {
        anyhow::bail!("screencapture area failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(output.stdout)
}

#[cfg(target_os = "macos")]
fn capture_window_macos() -> Result<Vec<u8>> {
    let output = Command::new("screencapture")
        .arg("-l")
        .arg("$(osascript -e 'tell app \"System Events\" to get id of window 1 of (first process whose frontmost is true)')")
        .arg("-x")
        .output()
        .context("Failed to execute screencapture for window")?;
    if !output.status.success() {
        anyhow::bail!("screencapture window failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(output.stdout)
}

// ==================== Windows 实现 ====================

#[cfg(target_os = "windows")]
fn capture_screen_windows() -> Result<Vec<u8>> {
    // 使用 PowerShell 截屏，使用 CREATE_NO_WINDOW 标志避免闪现窗口
    let ps_script = r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$screen = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$bitmap = New-Object System.Drawing.Bitmap($screen.Width, $screen.Height)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen($screen.Location, [System.Drawing.Point]::Empty, $screen.Size)
$ms = New-Object System.IO.MemoryStream
$bitmap.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
$ms.Close()
[Convert]::ToBase64String($ms.ToArray())
"#;
    let output = Command::new("powershell")
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .args(["-NoProfile", "-Command", ps_script])
        .output()
        .context("Failed to execute PowerShell for screenshot")?;
    if !output.status.success() {
        anyhow::bail!("PowerShell screenshot failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    let b64_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let png_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &b64_str)
        .map_err(|e| anyhow::anyhow!("Failed to decode screenshot base64: {}", e))?;
    Ok(png_bytes)
}

#[cfg(target_os = "windows")]
fn capture_area_windows(x: i32, y: i32, width: i32, height: i32) -> Result<Vec<u8>> {
    let ps_script = format!(
        r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$bitmap = New-Object System.Drawing.Bitmap({}, {})
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen({}, {}, 0, 0, (New-Object System.Drawing.Size({}, {})))
$ms = New-Object System.IO.MemoryStream
$bitmap.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
$ms.Close()
[Convert]::ToBase64String($ms.ToArray())
"#,
        width, height, x, y, width, height
    );
    let output = Command::new("powershell")
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .args(["-NoProfile", "-Command", &ps_script])
        .output()
        .context("Failed to execute PowerShell for area screenshot")?;
    if !output.status.success() {
        anyhow::bail!("PowerShell area screenshot failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    let b64_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let png_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &b64_str)
        .map_err(|e| anyhow::anyhow!("Failed to decode area screenshot base64: {}", e))?;
    Ok(png_bytes)
}

#[cfg(target_os = "windows")]
fn capture_window_windows() -> Result<Vec<u8>> {
    // Windows 窗口截图使用 PowerShell 获取前台窗口
    let ps_script = r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class WinAPI {
    [DllImport("user32.dll")]
    public static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")]
    public static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);
    [StructLayout(LayoutKind.Sequential)]
    public struct RECT { public int Left, Top, Right, Bottom; }
}
"@
$hwnd = [WinAPI]::GetForegroundWindow()
$rect = New-Object WinAPI+RECT
[WinAPI]::GetWindowRect($hwnd, [ref]$rect)
$w = $rect.Right - $rect.Left
$h = $rect.Bottom - $rect.Top
$bitmap = New-Object System.Drawing.Bitmap($w, $h)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen($rect.Left, $rect.Top, 0, 0, (New-Object System.Drawing.Size($w, $h)))
$ms = New-Object System.IO.MemoryStream
$bitmap.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
$ms.Close()
[Convert]::ToBase64String($ms.ToArray())
"#;
    let output = Command::new("powershell")
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .args(["-NoProfile", "-Command", ps_script])
        .output()
        .context("Failed to execute PowerShell for window screenshot")?;
    if !output.status.success() {
        anyhow::bail!("PowerShell window screenshot failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    let b64_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let png_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &b64_str)
        .map_err(|e| anyhow::anyhow!("Failed to decode window screenshot base64: {}", e))?;
    Ok(png_bytes)
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
