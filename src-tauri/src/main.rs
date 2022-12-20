#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/*#region importing necessities */
use std::process;
use tauri::WindowBuilder;
use tauri::window::Window;
use tauri::Manager;
use std::path::PathBuf;
use zip_extensions::*;
/*#endregion */

mod user_preference_manager;
mod file_manager;
mod connection_manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}


#[tauri::command]
fn quitapplication(window: tauri::Window) {
    window.app_handle().exit(0x0100);
}
#[tauri::command]
async fn resizewindow(window: tauri::Window) {
  let is_maximized = window.is_maximized().unwrap();
  if is_maximized {
    window.unmaximize().unwrap();
  }else{
    window.maximize().unwrap();
  }
}
#[tauri::command]
fn minimizewindow(window: tauri::Window) {
    window.minimize().unwrap();
}
#[tauri::command]
fn visitwebsite(window: tauri::Window, url_to_open: String) {
  if webbrowser::open(&*url_to_open).is_ok() {
    // ...
  }
}

#[tauri::command]
fn extractcustom(window: tauri::Window, archive_file_path: String, destination_file_path: String) {

  let archivepath = PathBuf::from(&archive_file_path);
  let destinationpath = PathBuf::from(&destination_file_path);

  zip_extract(&archivepath, &destinationpath).unwrap();

  println!("File is at {}, and destination is {}", archive_file_path, destination_file_path);
}


#[tauri::command]
fn loadingscreenloaded(window: tauri::Window) {
  tauri::async_runtime::spawn(async move {
    window.emit("LoadingDescription", Payload { message: "Tauri is awesome!".into() }).unwrap();
    println!("{}",file_manager::get_vizuara_data_path().display());
    initialize_application(window).await;
  });
}

async fn initialize_application(window: tauri::Window) -> (){
/*
//<summary>
//The function that will prepare the application for further operation
//</summary>
// */

  let is_first_launch: bool = user_preference_manager::is_first_launch();

  /*#region display and welcome the user */
  if is_first_launch {
    window.emit("LoadingDescription", Payload { message: "Welcome to Vizuara: Teacher's Portal!".into() }).unwrap();

  }else{
    window.emit("LoadingDescription", Payload { message: "Welcome back to Vizuara: Teacher's Portal!".into() }).unwrap();
    println!("{}",user_preference_manager::get_application_version());
  }
  std::thread::sleep(std::time::Duration::from_secs(2));

  if connection_manager::check_connection(){
    println!("Has Connection");
    /*<summary>
    //communicate with the server and check if the current version is up-to-date or not
    //check version first -> if version matched -> launch main application anyways
    //if !match -> update the application by downloading necessary files, version will be stored inside user preference manager
    //
    // */
  }else{
    println!("Does not have connection");
    /*<summary>
    //does not have any connection
    //if first launch -> we will force user to reconnect again
    //if no -> launch main application anyways
    //</summary>
    // */
  }
  /*#endregion */
}

fn main() {
    tauri::Builder::default().setup(|app|{
        let splashscreen_window = app.get_window("splashscreen").unwrap();
        let main_window = app.get_window("main").unwrap();

        // we perform the initialization code on a new task so the app doesn't freeze
      /*tauri::async_runtime::spawn(async move {
        // initialize your app here instead of sleeping :)
        println!("Initializing...");
        splashscreen_window.emit("Loading", Payload { message: "Tauri is awesome!".into() }).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("Done initializing.");

        // After it's done, close the splashscreen and display the main window
        //splashscreen_window.close().unwrap();
        //main_window.show().unwrap();
      });*/

      let id = app.listen_global("InitializationCompleted", move |event| {
        splashscreen_window.close().unwrap();
        main_window.show().unwrap();
      });
        Ok(())
        })
        .invoke_handler(tauri::generate_handler![quitapplication, resizewindow, minimizewindow, visitwebsite, extractcustom, loadingscreenloaded])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
