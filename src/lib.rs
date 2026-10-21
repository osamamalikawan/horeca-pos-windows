use tauri::WebviewWindow;

#[cfg(target_os = "windows")]
use std::time::Duration;
#[cfg(target_os = "windows")]
use webview2_com::{
    Microsoft::Web::WebView2::Win32::{
        COREWEBVIEW2_PRINT_STATUS_SUCCEEDED, ICoreWebView2_16, ICoreWebView2Environment6,
    },
    PrintCompletedHandler,
};
#[cfg(target_os = "windows")]
use windows::core::Interface;

#[tauri::command]
async fn print_document(
    window: WebviewWindow,
    paper_width_mm: f64,
    paper_height_mm: f64,
) -> Result<(), String> {
    if !paper_width_mm.is_finite()
        || !paper_height_mm.is_finite()
        || !(20.0..=500.0).contains(&paper_width_mm)
        || !(10.0..=2000.0).contains(&paper_height_mm)
    {
        return Err("Invalid print page dimensions.".into());
    }

    #[cfg(target_os = "windows")]
    {
        let (completion_tx, completion_rx) = std::sync::mpsc::channel();
        window
            .with_webview(move |webview| {
                let result = unsafe {
                    let core = webview.controller().CoreWebView2();
                    let core = core.and_then(|core| core.cast::<ICoreWebView2_16>());
                    let environment = webview.environment().cast::<ICoreWebView2Environment6>();
                    core.and_then(|core| {
                        environment.and_then(|environment| {
                            let settings = environment.CreatePrintSettings()?;
                            settings.SetPageWidth(paper_width_mm / 25.4)?;
                            settings.SetPageHeight(paper_height_mm / 25.4)?;
                            settings.SetMarginTop(0.0)?;
                            settings.SetMarginBottom(0.0)?;
                            settings.SetMarginLeft(0.0)?;
                            settings.SetMarginRight(0.0)?;
                            settings.SetShouldPrintBackgrounds(true)?;
                            settings.SetShouldPrintHeaderAndFooter(false)?;

                            let callback_tx = completion_tx.clone();
                            let handler =
                                PrintCompletedHandler::create(Box::new(move |result, status| {
                                    let result =
                                        result.map_err(|error| error.to_string()).and_then(|()| {
                                            if status == COREWEBVIEW2_PRINT_STATUS_SUCCEEDED {
                                                Ok(())
                                            } else {
                                                Err(format!(
                                                    "Printing failed with WebView2 status {}.",
                                                    status.0
                                                ))
                                            }
                                        });
                                    let _ = callback_tx.send(result);
                                    Ok(())
                                }));
                            core.Print(&settings, &handler)
                        })
                    })
                };

                if let Err(error) = result {
                    let _ = completion_tx.send(Err(error.to_string()));
                }
            })
            .map_err(|error| error.to_string())?;

        return tauri::async_runtime::spawn_blocking(move || {
            completion_rx.recv_timeout(Duration::from_secs(90))
        })
        .await
        .map_err(|error| error.to_string())?
        .map_err(|error| format!("Unable to complete printing: {error}"))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = window;
        Err("Silent printing is currently supported only in the Windows desktop app.".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![print_document])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
