use std::{fs::File, io::Write, path::Path};

use windows::Win32::{
    Foundation::HWND,
    Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC,
        DIB_RGB_COLORS, DeleteDC, DeleteObject, GetDC, GetDIBits, GetDeviceCaps, HBITMAP, HORZRES,
        ReleaseDC, SRCCOPY, SelectObject, VERTRES,
    },
};

use crate::error::{CaptureError, SaveError};

pub struct ScreenshotCapture;

pub struct Bitmap {
    pub hbitmap: HBITMAP,
    pub width: i32,
    pub height: i32,
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe {
            let _ = DeleteObject(self.hbitmap);
        }
    }
}

impl ScreenshotCapture {
    pub fn capture() -> Result<Bitmap, CaptureError> {
        unsafe {
            // Get screen DC
            let screen_dc = GetDC(HWND(0));
            if screen_dc.is_invalid() {
                return Err(CaptureError::GetDCFailed);
            }

            // Create compatible DC
            let mem_dc = CreateCompatibleDC(screen_dc);
            if mem_dc.is_invalid() {
                let _ = ReleaseDC(HWND(0), screen_dc);
                return Err(CaptureError::CreateCompatibleDCFailed);
            }

            // Get screen dimensions
            let width = GetDeviceCaps(screen_dc, HORZRES);
            let height = GetDeviceCaps(screen_dc, VERTRES);

            // Create compatible bitmap
            let hbitmap = CreateCompatibleBitmap(screen_dc, width, height);
            if hbitmap.is_invalid() {
                let _ = DeleteDC(mem_dc);
                let _ = ReleaseDC(HWND(0), screen_dc);
                return Err(CaptureError::CreateBitmapFailed);
            }

            // Select bitmap into memory DC
            let _ = SelectObject(mem_dc, hbitmap);

            // Copy screen content to bitmap
            let result = BitBlt(mem_dc, 0, 0, width, height, screen_dc, 0, 0, SRCCOPY);

            // Cleanup DCs
            let _ = DeleteDC(mem_dc);
            let _ = ReleaseDC(HWND(0), screen_dc);

            if result.is_err() {
                return Err(CaptureError::BitBltFailed);
            }

            Ok(Bitmap {
                hbitmap,
                width,
                height,
            })
        }
    }

    pub fn save(bitmap: &Bitmap, path: &Path) -> Result<(), SaveError> {
        unsafe {
            // Get screen DC for GetDIBits
            let screen_dc = GetDC(HWND(0));
            if screen_dc.is_invalid() {
                return Err(SaveError::GetDIBitsFailed);
            }

            // Prepare BITMAPINFO structure
            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: bitmap.width,
                    biHeight: bitmap.height,
                    biPlanes: 1,
                    biBitCount: 24, // 24-bit RGB
                    biCompression: BI_RGB.0 as u32,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [Default::default(); 1],
            };

            // Calculate row size (must be multiple of 4)
            let row_size = ((bitmap.width * 3 + 3) / 4) * 4;
            let image_size = (row_size * bitmap.height) as usize;
            let mut pixel_data: Vec<u8> = vec![0; image_size];

            // Get bitmap bits
            let result = GetDIBits(
                screen_dc,
                bitmap.hbitmap,
                0,
                bitmap.height as u32,
                Some(pixel_data.as_mut_ptr() as *mut _),
                &mut bmi,
                DIB_RGB_COLORS,
            );

            let _ = ReleaseDC(HWND(0), screen_dc);

            if result == 0 {
                return Err(SaveError::GetDIBitsFailed);
            }

            // Create BMP file
            let mut file = File::create(path).map_err(SaveError::FileWriteFailed)?;

            // Write BITMAPFILEHEADER
            let file_size = 14 + 40 + image_size; // header + info header + pixel data
            file.write_all(&[0x42, 0x4D]) // "BM" signature
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&(file_size as u32).to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&[0, 0, 0, 0]) // Reserved
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&54u32.to_le_bytes()) // Offset to pixel data
                .map_err(SaveError::FileWriteFailed)?;

            // Write BITMAPINFOHEADER
            file.write_all(&bmi.bmiHeader.biSize.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&bmi.bmiHeader.biWidth.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&bmi.bmiHeader.biHeight.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&bmi.bmiHeader.biPlanes.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&bmi.bmiHeader.biBitCount.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&bmi.bmiHeader.biCompression.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&(image_size as u32).to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&bmi.bmiHeader.biXPelsPerMeter.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&bmi.bmiHeader.biYPelsPerMeter.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&bmi.bmiHeader.biClrUsed.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;
            file.write_all(&bmi.bmiHeader.biClrImportant.to_le_bytes())
                .map_err(SaveError::FileWriteFailed)?;

            // Write pixel data (already in BGR format from GetDIBits)
            file.write_all(&pixel_data)
                .map_err(SaveError::FileWriteFailed)?;

            // Print saved file path to console
            println!("Saved: {}", path.display());

            Ok(())
        }
    }
}
