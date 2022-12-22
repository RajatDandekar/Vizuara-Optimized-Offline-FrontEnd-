#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use] 
extern crate magic_crypt;

/*#region importing necessities */
use std::process;
use tauri::WindowBuilder;
use tauri::window::Window;
use tauri::Manager;
use std::path::PathBuf;
use zip_extensions::*;

use const_values::Event_Constants;
use const_values::Event_Messages;
/*#endregion */

mod user_preference_manager;
mod file_manager;
mod connection_manager;
mod const_values;
mod dialog_displayer;
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

/*#region common commands */
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
/*#endregion */

/*#region custom functions */
fn emit_event(target_window: tauri::Window, event_name: &str,message_to_deliver: String){
  target_window.emit(event_name, Payload { message: message_to_deliver}).unwrap();
}

async fn sleep_for_a_second(){
  std::thread::sleep(std::time::Duration::from_secs(1));
}

async fn sleep_for_two_seconds(){
  std::thread::sleep(std::time::Duration::from_secs(2));
}
/*#endregion */
#[tauri::command]
fn extractcustom(window: tauri::Window, archive_file_path: String, destination_file_path: String) {

  let archivepath = PathBuf::from(&archive_file_path);
  let destinationpath = PathBuf::from(&destination_file_path);

  zip_extract(&archivepath, &destinationpath).unwrap();

  println!("File is at {}, and destination is {}", archive_file_path, destination_file_path);
}


/*#region preparation */
#[tauri::command]
fn loadingscreenloaded(window: tauri::Window) {
  tauri::async_runtime::spawn(async move {
    emit_event(window.to_owned(), &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), Event_Messages.VIZUARA_INITIALIZING().into());
    //window.emit(&const_values::Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), Payload { message: "Tauri is awesome!".into() }).unwrap();
    println!("{}",file_manager::get_vizuara_data_path().display());
    initialize_application(window).await;
  });
}

async fn initialize_application(window: tauri::Window) -> (){

  /*#region Function Summary */
  /*
    <summary>
    this function initializes the application and prepare for usage within the application
    </summary>
  */
  /*#endregion */

  /*#region Local Functions */
  fn check_if_equal(value1: String, value2: String) -> bool{
    value1 == value2
  }

  async fn ExitAfter10Seconds(win: tauri::Window, message_content: String){
    for number in (1..10).rev() {
      emit_event(win.to_owned(), &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), const_values::Event_Messages.CLOSING_IN_WITH_MESSAGES(message_content.to_owned(), number).to_string());
      sleep_for_a_second().await;
    }
    quitapplication(win);
  }
  /*#endregion */

  //Rewriting the version text in the frontend
  emit_event(window.to_owned(), &Event_Constants.GET_APPLICATION_VERSION_EVENT(), const_values::APPLICATION_VERSION.into());

  //the variable that controls whether we should force the user to update their application by redirecting to the website
  let server_app_version_state = connection_manager::check_application_version_and_connection_state().await;
  //variable that detects whether it is the user's first time launching the application
  let is_first_launch: bool = user_preference_manager::is_first_launch();

  /*#region if connection is available */
  if server_app_version_state.is_ok(){

    /*#region set initial text */
    if is_first_launch {
      emit_event(window.to_owned(), &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), Event_Messages.WELCOME().into());
    }else{
      emit_event(window.to_owned(), &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), Event_Messages.WELCOME_BACK().into());
      //std::thread::sleep(std::time::Duration::from_secs(2));
      //window.emit("LoadingDescription", Payload { message: ["v", &server_app_version_state.unwrap()[..]].join("") }).unwrap();
    }
    /*#endregion */

    sleep_for_two_seconds().await;

    let app_version: String = const_values::APPLICATION_VERSION.into();
    let server_app_version: String = server_app_version_state.ok().unwrap();

    /*#region Application is Outdated */
    if !check_if_equal(app_version, server_app_version) {

      /*#region Application is outdated! Ask user to update! */
      //Quit the application
      ExitAfter10Seconds(window.to_owned(), 
      Event_Messages.UPDATE_APPLICATION().into()).await;
      /*#endregion */

    }/*#endregion */
    
    /*#region Application is updated! Contine with the process of checking*/
    else{
       
    }
    /*#endregion */

  }
  /*#endregion */

  /*#region if connection is not available */
  else{
    if is_first_launch {

      /*#region Do NOT EVEN bother to launch the application */
      //Force the user to quit
       emit_event(window.to_owned(), 
                  &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), 
                  Event_Messages.WELCOME().into());

      std::thread::sleep(std::time::Duration::from_secs(2));

      ExitAfter10Seconds(window.to_owned(), 
                        Event_Messages.FIRST_LAUNCH_CLOSING_MESSAGE().into()).await;
    /*#endregion */
      
    }else{

      /*#region Launch Application in offline mode */
      //Launch Application in offline mode since connection is not available and data seem to be valid
      emit_event(window.to_owned(), 
      &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), 
      Event_Messages.WELCOME_BACK().into());
      
      std::thread::sleep(std::time::Duration::from_secs(2));

      emit_event(window.to_owned(), 
      &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), 
      Event_Messages.ONLINE_MODE_NOT_AVAILABLE_ENTERING_OFFLINE_MODE().into());
      /*#endregion */
    }
  }
  /*#endregion */
 
}
/*#endregion */

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
