// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use anyhow::Result;
use tauri_plugin_updater::UpdaterExt;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("app_lib::run -> start");
    // Initialize logger for production
    #[cfg(not(debug_assertions))]
    {
        std::env::set_var("RUST_LOG", "debug");
    }
    
    #[cfg(debug_assertions)]
    env_logger::init();

    println!("app_lib::run -> before Builder::default");
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
         .setup(|app| {
            let app_handle = app.handle().clone(); // <-- Clone the AppHandle
        
            println!("tauri setup callback");
            tauri::async_runtime::spawn(async move {
                if let Err(e) = update(app_handle).await {
                    eprintln!("Update failed: {}", e);
                }
            });
        
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("app_lib::run -> end");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
      let mut downloaded = 0;
  
      // alternatively we could also call update.download() and update.install() separately
      update
        .download_and_install(
          |chunk_length, content_length| {
            downloaded += chunk_length;
            println!("downloaded {downloaded} from {content_length:?}");
          },
          || {
            println!("download finished");
          },
        )
        .await?;
  
      println!("update installed");
      app.restart();
    }
  
    Ok(())
  }