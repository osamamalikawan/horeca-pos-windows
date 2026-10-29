//! Windows printing helpers: list the installed printers, and build the WebView2 print settings
//! (paper size, chosen printer, copies) used for printing without a dialog.

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PrinterInfo {
    pub name: String,
    pub is_default: bool,
}

/// Names of the printers installed on this computer (local and network), with the Windows default marked.
#[cfg(target_os = "windows")]
pub fn list_installed_printers() -> Result<Vec<PrinterInfo>, String> {
    use windows::Win32::Graphics::Printing::{
        EnumPrintersW, GetDefaultPrinterW, PRINTER_ENUM_CONNECTIONS, PRINTER_ENUM_LOCAL,
        PRINTER_INFO_4W,
    };
    use windows::core::{PCWSTR, PWSTR};

    let mut names: Vec<String> = Vec::new();
    let mut default_name = String::new();

    unsafe {
        let flags = PRINTER_ENUM_LOCAL | PRINTER_ENUM_CONNECTIONS;
        let (mut needed, mut returned) = (0u32, 0u32);
        // The first call only reports how much space is needed. It "fails" on purpose.
        let _ = EnumPrintersW(flags, PCWSTR::null(), 4, None, &mut needed, &mut returned);

        if needed > 0 {
            // u64 elements keep the buffer 8-byte aligned, which PRINTER_INFO_4W requires.
            let mut buffer = vec![0u64; (needed as usize).div_ceil(8)];
            let bytes =
                std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, buffer.len() * 8);
            EnumPrintersW(
                flags,
                PCWSTR::null(),
                4,
                Some(bytes),
                &mut needed,
                &mut returned,
            )
            .map_err(|error| error.to_string())?;

            let infos = std::slice::from_raw_parts(
                buffer.as_ptr() as *const PRINTER_INFO_4W,
                returned as usize,
            );
            for info in infos {
                if !info.pPrinterName.is_null() {
                    names.push(info.pPrinterName.to_string().unwrap_or_default());
                }
            }
        }

        let mut length = 0u32;
        let _ = GetDefaultPrinterW(None, &mut length);
        if length > 0 {
            let mut wide = vec![0u16; length as usize];
            if GetDefaultPrinterW(Some(PWSTR(wide.as_mut_ptr())), &mut length).as_bool() {
                let end = (length as usize).saturating_sub(1).min(wide.len());
                default_name = String::from_utf16_lossy(&wide[..end]);
            }
        }
    }

    names.retain(|name| !name.is_empty());
    names.sort_by_key(|name| name.to_lowercase());
    names.dedup();
    Ok(names
        .into_iter()
        .map(|name| PrinterInfo {
            is_default: name == default_name,
            name,
        })
        .collect())
}

#[cfg(not(target_os = "windows"))]
pub fn list_installed_printers() -> Result<Vec<PrinterInfo>, String> {
    Err("Listing printers is supported only in the Windows desktop app.".into())
}

/// Print settings for one job. An empty or missing printer name means the Windows default printer.
#[cfg(target_os = "windows")]
pub fn build_print_settings(
    environment: &webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2Environment6,
    paper_width_mm: f64,
    paper_height_mm: f64,
    printer_name: Option<&str>,
    copies: i32,
) -> windows::core::Result<webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2PrintSettings>
{
    use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2PrintSettings2;
    use windows::core::{Interface, PCWSTR};

    unsafe {
        let settings = environment.CreatePrintSettings()?;
        settings.SetPageWidth(paper_width_mm / 25.4)?;
        settings.SetPageHeight(paper_height_mm / 25.4)?;
        settings.SetMarginTop(0.0)?;
        settings.SetMarginBottom(0.0)?;
        settings.SetMarginLeft(0.0)?;
        settings.SetMarginRight(0.0)?;
        settings.SetShouldPrintBackgrounds(true)?;
        settings.SetShouldPrintHeaderAndFooter(false)?;

        let printer = printer_name.map(str::trim).filter(|name| !name.is_empty());
        if printer.is_some() || copies > 1 {
            let settings2 = settings.cast::<ICoreWebView2PrintSettings2>()?;
            if let Some(name) = printer {
                let wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
                settings2.SetPrinterName(PCWSTR(wide.as_ptr()))?;
            }
            if copies > 1 {
                settings2.SetCopies(copies)?;
            }
        }
        Ok(settings)
    }
}
