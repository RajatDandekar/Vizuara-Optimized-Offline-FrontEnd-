#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use] 
extern crate magic_crypt;

/*#region importing necessities */
use tauri::Manager;
use std::path::PathBuf;
use zip_extensions::*;
use lazy_static::lazy_static;
use std::sync::RwLock;

use const_values::Event_Constants;
use const_values::Event_Messages;
/*#endregion */

mod user_preference_manager;
mod file_manager;
mod connection_manager;
mod const_values;
mod dialog_displayer;
mod data_structure_manager;

/*#region const variables */
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

lazy_static!{

  //this variable will control whether the in-app-application will be displayed in offline mode
  //so if this variable true, for example, user wouldn't be able to download new cheapters, etc
  static ref offline_mode: RwLock<bool> = RwLock::new( 
      false
  );
}  

fn set_offline_mode(is_offline_mode_local: bool){
  let mut global_is_offline_mode = offline_mode.write().unwrap();
  *global_is_offline_mode = is_offline_mode_local;
}

fn is_offline_mode() -> bool {
  *offline_mode.read().unwrap()
}
/*#endregion */

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

async fn exit_after_10_seconds(win: tauri::Window, message_content: String){
  for number in (1..10).rev() {
    emit_event(win.to_owned(), &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), const_values::Event_Messages.CLOSING_IN_WITH_MESSAGES(message_content.to_owned(), number).to_string());
    sleep_for_a_second().await;
  }
  quitapplication(win);
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
  fn requires_update(value1: String, value2: String) -> bool{
    value1 != value2
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
    if requires_update(app_version, server_app_version) {

      /*#region Application is outdated! Ask user to update! */
      //Quit the application
      exit_after_10_seconds(window.to_owned(), 
      Event_Messages.UPDATE_APPLICATION().into()).await;
      /*#endregion */

    }/*#endregion */
    
    /*#region Application is updated! Contine with the process of checking*/
    else{
       
      let server_data_version_state = connection_manager::check_data_version_and_connection_state().await;

      if server_data_version_state.is_ok(){
        let data_version: String = user_preference_manager::get_data_version().unwrap();
        let server_data_version: String = server_data_version_state.ok().unwrap();

        /*#region require in app update */
        if requires_update(data_version, server_data_version) {
          //need to download the data structure files
          //we will either create or overwrite the existing data struct file

          if !file_manager::does_data_struct_keyfile_exists() {
            //
            if is_first_launch {
              update_and_start(window.to_owned()).await;
            }
            
            /*#region data struct file does not exist when it should exist */
            else{

              //not first launch but does not have the struct key file
              //Something has happened which shouldn't have happened
              //delete the entire folder
              //and quit the application
              file_manager::delete_vizuara_directory_and_recreate();

              exit_after_10_seconds(window.to_owned(), 
              Event_Messages.UNEXPECTED_ERROR_CORRUPTED_DATA_STRUCT_KEY().into()).await;
            }
            /*#endregion */

          }
          /*#region Just casually update the application */
          else{
            update_and_start(window.to_owned()).await;
          }
        }/*#endregion */
        
        /*#region require no updates */
        else{
          //Just Launch the application
          prepare_to_launch(window).await;
        }
        /*#endregion */
      }
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

      exit_after_10_seconds(window.to_owned(), 
                        Event_Messages.FIRST_LAUNCH_CLOSING_MESSAGE().into()).await;
    /*#endregion */
      
    }else{

      /*#region Launch Application in offline mode */
      //Launch Application in offline mode since connection is not available and data seem to be valid
      emit_event(window.to_owned(), 
      &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), 
      Event_Messages.WELCOME_BACK().into());
      
      sleep_for_two_seconds().await;

      emit_event(window.to_owned(), 
      &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), 
      Event_Messages.ONLINE_MODE_NOT_AVAILABLE_ENTERING_OFFLINE_MODE().into());

      sleep_for_two_seconds().await;

      set_offline_mode(true);

      prepare_to_launch(window);
      /*#endregion */
    }
  }
  /*#endregion */
 
}

async fn update_and_start(window: tauri:: Window){

  //First time user, allow them to download the data struct key file from the server
  //println!("Trying to save data struct");
  if !data_structure_manager::save_data_struct_from_server().await.is_ok(){

    exit_after_10_seconds(window.to_owned(), 
    Event_Messages.UNEXPECTED_ERROR_SAVING_DATA_STRUCT_FAILED().into()).await;

  }else{              
    prepare_to_launch(window).await;
  }

}

async fn prepare_to_launch(window: tauri::Window) {

        //data_struct file has been successfully downloaded onto the user's machine! Good job!
        //Now we will prepare to launch the application!

        //STEPS TO TAKE
        //1. READ THE STRUCT FILE
        //2. LOAD ALL THE NECESSARY DATA ONTO A GLOBAL VECTOR (PROBABLY OR NOT) ALTERNATIVELY, WILL JUST CONFIRM IF THE FILE IS NOT CORRUPTED
        //3. DO ALL THE OPERATION NECESSARY WITH THE VECTOR

        println!("Trying to check data struct integrity");
        let data_struct_integrity_result: Result<(),()> = data_structure_manager::read_and_check_and_save_data_struct_file_integrity().await;
                
        /*#region data struct integrity exists! */
        if data_struct_integrity_result.is_ok() {
          emit_event(window.to_owned(), 
          &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), 
          Event_Messages.LAUNCHING_APPLICATION().into());

          sleep_for_a_second().await;


          emit_event(window.to_owned(), 
          &Event_Constants.GET_INITIALIZATION_COMPLETED(), 
          Event_Messages.LAUNCHING_APPLICATION().into());

          //Launch the application!
          //println!("Launch the application!")
        }
      /*#endregion */

      /*#region data struct integrity lost */
      else{
        //corrupted! delete and exit and ask to restart!
        file_manager::delete_vizuara_directory_and_recreate();

        exit_after_10_seconds(window.to_owned(), 
        Event_Messages.UNEXPECTED_ERROR_CORRUPTED_DATA_STRUCT_KEY().into()).await;

        //GET_INITIALIZATION_COMPLETED
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
