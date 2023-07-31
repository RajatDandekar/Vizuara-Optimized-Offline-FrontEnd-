#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use] 
extern crate magic_crypt;
extern crate winreg;
extern crate win_msgbox;
extern crate chrono;


/*#region importing necessities */

use file_manager::get_path_in_vizuara_folder;
use futures_util::__private::async_await;
use reqwest::Url;
use futures_util::{StreamExt, FutureExt};
use tauri::Manager;
use user_preference_manager::remove_downloaded_chapter;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use chrono::NaiveDate;
use win_msgbox::Okay;
use windows_sys::{w};


use std::fmt::format;
use std::os::windows::process::CommandExt;
use std::path::{PathBuf, Path};
use std::vec;
use std::io::{Cursor, Write, BufReader};
use std::sync::RwLock;
use std::fs::File;
use std::io::{self, BufRead};

use winreg::enums::*;
use winreg::RegKey;

use zip_extensions::*;
use lazy_static::lazy_static;

use std::process::Command;

use serde_json::{Value, Map};
use serde_json::Number;

use actix_files as actix_fs;
use actix_web::{App, HttpServer};

use rand::{distributions::Alphanumeric, Rng}; 

use const_values::Event_Constants;
use const_values::Event_Messages;

  mod user_preference_manager;
  mod file_manager;
  mod connection_manager;
  mod const_values;
  mod dialog_displayer;
  mod data_structure_manager;

/*#endregion */

/*#region const variables */

/*#region Payloads for Events */
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

#[derive(Clone, serde::Serialize)]
struct SelectionUpdateStruct {
  class_id: i32, //For Searching Purposes
  class_name: String, //For Displaying Purposes
  
  chapter_id: i32,
  chapter_name: String,

  file_type: String,

  file_id: i32,
  file_name: String
}
/*#endregion */

lazy_static!{

  //this variable will control whether the in-app-application will be displayed in offline mode
  //so if this variable true, for example, user wouldn't be able to download new cheapters, etc
  static ref offline_mode: RwLock<bool> = RwLock::new( 
      false
  );

  static ref currently_selected_class_id: RwLock<i32> = RwLock::new(
    0
  );

  static ref currently_selected_class_name: RwLock<String> = RwLock::new(
    "".into()
  );

  static ref currently_selected_chapter_id: RwLock<i32> = RwLock::new(
    0
  );

  static ref currently_selected_chapter_name: RwLock<String> = RwLock::new(
    "".into()
  );

  static ref currently_selected_file_type: RwLock<String> = RwLock::new(
    "".into()
  );

  static ref currently_selected_file_id: RwLock<i32> = RwLock::new(
    0
  );

  static ref a_file_is_being_downloaded: RwLock<i32> = RwLock::new(
    0
  );
}  


fn set_offline_mode(is_offline_mode_local: bool){
  let mut global_is_offline_mode = offline_mode.write().unwrap();
  *global_is_offline_mode = is_offline_mode_local;
  drop(global_is_offline_mode);
}

fn is_offline_mode() -> bool {
  *offline_mode.read().unwrap()
}

fn set_current_selected_class_id(new_value: i32){
  let mut currently_selected_class_id_local = currently_selected_class_id.write().unwrap();
  *currently_selected_class_id_local = new_value;
  drop(currently_selected_class_id_local);
}

fn get_current_selected_class_id() -> i32{
  *currently_selected_class_id.read().unwrap()
}

fn set_current_selected_class_name(new_value: String){
  let mut currently_selected_class_name_local = currently_selected_class_name.write().unwrap();
  *currently_selected_class_name_local = new_value;
  drop(currently_selected_class_name_local);
}

fn get_current_selected_class_name() -> String{
  (*currently_selected_class_name.read().unwrap()).to_string()
}

fn set_current_selected_chapter_id(new_value: i32){
  let mut currently_selected_chapter_id_local = currently_selected_chapter_id.write().unwrap();
  *currently_selected_chapter_id_local = new_value;
  drop(currently_selected_chapter_id_local);
}

fn get_current_selected_chapter_id() -> i32{
  *currently_selected_chapter_id.read().unwrap()
}

fn set_current_selected_chapter_name(new_value: String){
  let mut currently_selected_chapter_name_local = currently_selected_chapter_name.write().unwrap();
  *currently_selected_chapter_name_local = new_value;
  drop(currently_selected_chapter_name_local);
}

fn get_current_selected_chapter_name() -> String{
  (*currently_selected_chapter_name.read().unwrap()).to_string()
}

fn set_current_selected_file_type(new_value: String){
  let mut currently_selected_file_type_local = currently_selected_file_type.write().unwrap();
  *currently_selected_file_type_local = new_value;
  drop(currently_selected_file_type_local);
}

fn get_current_selected_file_type() -> String{
  (*currently_selected_file_type.read().unwrap()).to_string()
}

fn set_current_selected_file_id(new_value: i32){
  let mut currently_selected_file_id_local = currently_selected_file_id.write().unwrap();
  *currently_selected_file_id_local = new_value;
  drop(currently_selected_file_id_local);
}

fn get_current_selected_file_id() -> i32{
  (*currently_selected_file_id.read().unwrap())
}

fn set_a_file_is_being_downloaded(new_value: bool){
  let mut a_file_is_being_downloaded_local = a_file_is_being_downloaded.write().unwrap();
  if new_value == true {
    *a_file_is_being_downloaded_local = 1;
  }else{
    *a_file_is_being_downloaded_local = 0;
  }
  drop(a_file_is_being_downloaded_local);
}

fn get_a_file_is_being_downloaded() -> bool{
  if *a_file_is_being_downloaded.read().unwrap() == 1 {
    true
  }else{
    false
  }
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


/*#region button click */

/*#region class selection*/
fn emit_selection_update_event(window: tauri::Window, class_id: i32, class_name: String, chapter_id: i32, chapter_name: String, file_type: String, file_id: i32, file_name: String){
  
  window.emit(&Event_Constants.GET_SELECTION_UPDATED(), SelectionUpdateStruct{class_id : class_id, class_name: (&class_name).to_owned(), chapter_id: chapter_id, chapter_name: (&chapter_name).to_owned(), file_type: (&file_type).to_owned(), file_id: file_id, file_name: file_name});
  
  set_current_selected_class_id(class_id.to_owned());
  set_current_selected_class_name((&class_name).to_string());
  set_current_selected_chapter_id(chapter_id.to_owned());
  set_current_selected_chapter_name((&chapter_name).to_string());
  set_current_selected_file_type((&file_type).to_string());
  set_current_selected_file_id(file_id.to_owned());

}

fn reload_selection_screen(window: tauri:: Window){
  emit_selection_update_event(window, 
                              get_current_selected_class_id(), get_current_selected_class_name(),
                              get_current_selected_chapter_id(), get_current_selected_chapter_name(), 
                              get_current_selected_file_type(),
                              0, "".into())
}
#[tauri::command]
fn set_class_id_method(window: tauri::Window, class_id: i32) -> (){
  println!("Class Selected -> id = {:?}", (&class_id).to_string());
  
  let mut class_name = data_structure_manager::find_in_static_classes_struct_with_class_id(class_id.to_string()).unwrap().replace("\\", "");

  emit_selection_update_event(window, class_id, 
                                      class_name.replace("\"", "")
                                      , 0, "".into(),"".into(), 0, "".into());

  data_structure_manager::clear_static_chapters_struct();
  data_structure_manager::insert_chapters_to_variable(class_id.to_string());
}

#[tauri::command]
fn set_chapter_id_method(window: tauri::Window, chapter_id: i32) -> (){
  println!("Chapter Selected -> id = {:?}", (&chapter_id).to_string());

  let chapter_name = data_structure_manager::find_in_static_chapters_struct_with_chapter_id(chapter_id.to_string()).unwrap().get("name").unwrap().to_string();

  emit_selection_update_event(window, 
                              get_current_selected_class_id(), get_current_selected_class_name(),
                              chapter_id, chapter_name, "".into(),
                              0, "".into());
}

#[tauri::command]
fn set_file_type_method(window: tauri::Window, file_type: String) -> (){
  println!("File Type Selected -> id = {:?}", (&file_type).to_string());

  //let chapter_name = data_structure_manager::find_in_static_chapters_struct_with_chapter_id(chapter_id.to_string()).unwrap().get("name").unwrap().to_string();

  emit_selection_update_event(window, 
                              get_current_selected_class_id(), get_current_selected_class_name(),
                              get_current_selected_chapter_id(), get_current_selected_chapter_name(),
                              file_type,
                              0, "".into());
}

#[tauri::command]
fn set_file_method(window: tauri::Window, file: i32) -> (){

  //let chapter_name = data_structure_manager::find_in_static_chapters_struct_with_chapter_id(chapter_id.to_string()).unwrap().get("name").unwrap().to_string();

  println!("selected file id {:?}", file);

  let file_type_path = file_manager::get_path_in_vizuara_folder(&(get_current_selected_class_id().to_string() + "_" + &get_current_selected_chapter_id().to_string())).join(get_current_selected_file_type());
  let file_name = file_manager::read_file((&file_type_path).join(Path::new(&file.to_string())).join("filename.txt"));

  emit_selection_update_event(window, 
                              get_current_selected_class_id(), get_current_selected_class_name(),
                              get_current_selected_chapter_id(), get_current_selected_chapter_name(),
                              get_current_selected_file_type(),
                              file, file_name);
}

#[tauri::command]
fn get_chapters_within_current_classes(window: tauri::Window) -> Vec<Value>{
  let mut loop_counter = 1;
  
  let mut data_to_return: Vec<Value> = Vec::new();
  loop{
     let get_chapter_result = data_structure_manager::find_in_static_chapters_struct_with_chapter_index(loop_counter.to_string());
     
     if get_chapter_result.is_ok() {
        let current_chapter = get_chapter_result.unwrap();

        let mut insertation_chapter_detail : Map<String, Value> = Map::new();
                    
        insertation_chapter_detail.insert("name".into(), (&current_chapter).get("name").unwrap().to_owned());
        insertation_chapter_detail.insert("subname".into(), (&current_chapter).get("subname").unwrap().to_owned());
        insertation_chapter_detail.insert("link".into(), (&current_chapter).get("link").unwrap().to_owned());
        insertation_chapter_detail.insert("id".into(), (&current_chapter).get("id").unwrap().to_owned());
        insertation_chapter_detail.insert("thumbnailpath".into(), Value::String(file_manager::get_chapter_thumbnail(((&current_chapter).get("id").unwrap().to_owned()).to_string()).to_string_lossy().to_string()));

        let mut chapter_folder_id = get_current_selected_class_id().to_string();
        chapter_folder_id.push_str("_");
        chapter_folder_id.push_str(&(&current_chapter).get("id").unwrap().to_owned().to_string());
        chapter_folder_id = chapter_folder_id.replace("\"", "");

        println!("Chapter folder id {:?}", chapter_folder_id);
        let chapter_available: bool = user_preference_manager::check_if_chapter_is_downloaded((&chapter_folder_id).to_owned()).is_ok();
        
        insertation_chapter_detail.insert("available".into(), Value::Bool((&chapter_available).to_owned()));
        insertation_chapter_detail.insert("version".into(), (&current_chapter).get("version").unwrap().to_owned());
        if user_preference_manager::check_chapter_version((&chapter_folder_id).to_owned()).to_owned() != (&current_chapter).get("version").unwrap().to_owned() {
          if chapter_available {
            insertation_chapter_detail.insert("shouldupdate".into(), Value::Bool(true));
          }else{
            insertation_chapter_detail.insert("shouldupdate".into(), Value::Bool(false));
          }
        }else{
          insertation_chapter_detail.insert("shouldupdate".into(), Value::Bool(false));
        }

        data_to_return.push(Value::Object(insertation_chapter_detail));
     }else{
        break;
     }
     loop_counter+=1;
  }
  data_to_return
} 

#[tauri::command]
fn get_file_types(window: tauri::Window) -> Vec<Value>{
  
  let mut loop_counter = 1;
  
  let mut data_to_return: Vec<Value> = Vec::new();

  let file_type_list = file_manager::list_all_within_a_folder(file_manager::get_path_in_vizuara_folder(&(get_current_selected_class_id().to_string() + "_" + &get_current_selected_chapter_id().to_string()))).unwrap();
  
  for file_type in &file_type_list{
    println!("{:?}", &file_type);
    
    let mut insertation_file_type_detail : Map<String, Value> = Map::new();
    insertation_file_type_detail.insert("name".into(), Value::String((&file_type).to_owned().to_owned()));
    data_to_return.push(Value::Object(insertation_file_type_detail));
  }
  data_to_return
}

#[tauri::command]
fn get_files(window: tauri::Window) -> Vec<Value>{
  let mut loop_counter = 1;
  
  let mut data_to_return: Vec<Value> = Vec::new();

  let file_type_path = file_manager::get_path_in_vizuara_folder(&(get_current_selected_class_id().to_string() + "_" + &get_current_selected_chapter_id().to_string())).join(get_current_selected_file_type());

  let files_path = file_manager::list_all_within_a_folder((&file_type_path).to_owned()).unwrap();
  
  for file_name in &files_path{
    println!("listing available{:?}", &file_name);
    
    let mut insertation_file_detail : Map<String, Value> = Map::new();

    insertation_file_detail.insert("id".into(), Value::String((&file_name).to_owned().to_owned()));

    println!("Trying to read file -> {:?}", (&file_type_path).join(file_name).join("filename.txt").to_string_lossy().to_string());

    insertation_file_detail.insert("name".into(), Value::String(file_manager::read_file((&file_type_path).join(file_name).join("filename.txt"))));
    insertation_file_detail.insert("thumbnail_path".into(), Value::String((&file_type_path).join(file_name).join("thumbnail.png").to_string_lossy().to_string()));

    data_to_return.push(Value::Object(insertation_file_detail));
  }
  data_to_return
}

#[tauri::command]
fn get_display_file_path(window: tauri::Window) -> String{
  let file_type_path = file_manager::get_path_in_vizuara_folder(&(get_current_selected_class_id().to_string() + "_" + &get_current_selected_chapter_id().to_string())).join(get_current_selected_file_type());
  let file_name = (&file_type_path).join(Path::new(&get_current_selected_file_id().to_string())).join("index.html").to_string_lossy().to_string();
  println!("{:?}", file_name);
  file_name
}

#[tauri::command]
fn get_chapters_in_current_class(window: tauri::Window){
  println!("Getting current chapters -> {:?}", "Hello")
}

fn random_string_generator() -> String{
  let s: String = rand::thread_rng()
  .sample_iter(&Alphanumeric)
  .take(7)
  .map(char::from)
  .collect();
  s
}

#[tauri::command]
async fn download_data_and_extract(window: tauri::Window, data: String, folder: String, version: String) -> (){

  if !is_offline_mode(){
  if(get_a_file_is_being_downloaded() == true){
    return
  }
  set_a_file_is_being_downloaded(true);

  let random_string: String = random_string_generator();

  println!("{:?} {:?}", data, folder);
  println!("{:?}", file_manager::get_path_in_vizuara_folder(&((random_string).to_owned() + ".zip")));


  /*#region download file and extract */

  /*#region download */

  download_file(window.to_owned(), data, file_manager::get_path_in_vizuara_folder(&((random_string).to_owned() + ".zip"))).await;

  /*#endregion */

  sleep_for_a_second().await;

  /*#region unzip */
  if file_manager::does_folder_exist_in_vizuara_folder(&folder) {

    //delete folder and recreate
    file_manager::delete_folder_and_recreate(get_path_in_vizuara_folder(&folder));

  }else{

    //create the directory only
    file_manager::create_directory(get_path_in_vizuara_folder(&folder));
    
  }

  extract_zip_file(file_manager::get_path_in_vizuara_folder(&(random_string + ".zip")), file_manager::get_path_in_vizuara_folder(&folder)).await;

  user_preference_manager::add_new_downloaded_chapter((&folder).to_owned(), (&version).to_owned());

  set_a_file_is_being_downloaded(false);
  
  reload_selection_screen(window);
  //fileToWrite.write_all(&DownloadBody);
  /*#endregion */

  /*#endregion */

  }
}

#[tauri::command]
async fn delete_chapter(window: tauri::Window, folder: String) -> (){
  user_preference_manager::remove_downloaded_chapter((&folder).to_owned());
  reload_selection_screen(window);

  if file_manager::does_folder_exist_in_vizuara_folder(&folder) {
      file_manager::delete_folder(get_path_in_vizuara_folder(&folder));
  }
}


async fn download_file(window: tauri::Window, hyperlink: String, save_path: PathBuf){
  let mut fileToWrite = match std::fs::File::create(save_path){
    Err(why)=> {
      exit_after_10_seconds(window.to_owned(), 
      Event_Messages.MUST_RESTART_APPLICATION().into()).await;
      panic!("{:?}", why)},
    Ok(file) => file,
  };

  let resp = reqwest::get(&hyperlink).await.expect("request failed");
  let total_size = resp.content_length().unwrap();

  let mut downloaded: u64 = 0;
  let mut download_stream = resp.bytes_stream();

  if get_a_file_is_being_downloaded() {
    //in main app not in loading screen
    emit_event(window.to_owned(), &Event_Constants.FILE_START_DOWNLOAD(), "".into());
  }

  let mut chunk_counter = 0;

  while let Some(item) = download_stream.next().await {

    let chunk = item.or(Err(format!("Error while downloading file"))).unwrap();

    fileToWrite.write_all(&chunk)
        .or(Err(format!("Error while writing to file")));
    let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
    downloaded = new;

    chunk_counter += 1;
    if chunk_counter == 50 {
      println!("{:?}/{:?}", downloaded, total_size);
      if get_a_file_is_being_downloaded() {
        //in main app not in loading screen
        let download_percentage = ((((downloaded as f64/total_size as f64)) as f64) * (100 as f64)) as i32;
        emit_event(window.to_owned(), &Event_Constants.FILE_BEING_DOWNLOADED(), download_percentage.to_string());
      }
      chunk_counter = 0;
    }
  }

  if get_a_file_is_being_downloaded() {
    //in main app not in loading screen
    emit_event(window.to_owned(), &Event_Constants.FILE_END_DOWNLOAD(), "".into());
  }
  return
}

async fn extract_zip_file(zip_file_path: PathBuf, extract_destination: PathBuf){
  let file_to_read = std::fs::File::open(zip_file_path).unwrap();

  let mut archive = zip::ZipArchive::new(file_to_read).unwrap();

  for i in 0..archive.len() {
      let mut file = archive.by_index(i).unwrap();
      let outpath = match file.enclosed_name() {
        Some(path) => get_path_in_vizuara_folder(&extract_destination.to_owned().to_string_lossy()).join(path),
        None => continue,
    };;

      {
          let comment = file.comment();
          if !comment.is_empty() {
              println!("File {} comment: {}", i, comment);
          }
      }
      if (*file.name()).ends_with('/') {
          println!("File {} extracted to \"{}\"", i, outpath.display());
          std::fs::create_dir_all(&outpath).unwrap();
      } else {
          println!(
              "File {} extracted to \"{}\" ({} bytes)",
              i,
              outpath.display(),
              file.size()
          );
          if let Some(p) = outpath.parent() {
              if !p.exists() {
                  std::fs::create_dir_all(p).unwrap();
              }
          }
          let mut outfile = std::fs::File::create(&outpath).unwrap();
          std::io::copy(&mut file, &mut outfile).unwrap();
      }
  }
}
/*#endregion */

/*#endregion */

/*#region getting data*/
#[tauri::command]
fn getclassesdata(window: tauri::Window) -> Vec<Value>{
    let mut classes_vector: Vec<Value> = Vec::new();

    println!("Get_Classes Function being called");
    
    let mut loop_counter = 1;
    loop  {
      let class_get_result = data_structure_manager::get_class_basic_data((&loop_counter).to_string());
      if class_get_result.is_ok() {

        let tempMap = class_get_result.ok().unwrap();

        data_structure_manager::insert_to_static_classes_struct((&loop_counter).to_string(), 
                                                                (&tempMap).get("name").unwrap().to_string());


        classes_vector.push(Value::Object(tempMap));

      }else{
        break;
      }
      loop_counter+=1;
    }

    classes_vector
}

/*#endregion*/

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
        println!("server-data-version-> {:?}", server_data_version);

        /*#region require in app update */
        if requires_update(data_version, (&server_data_version).to_owned()) {
          //need to download the data structure files
          //we will either create or overwrite the existing data struct file
          
          //downloading server_launcher.exe
          emit_event(window.to_owned(), &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), "Downloading necessary data (5 MB)".into());
          download_file(window.to_owned(),"https://vizuaraserver.ap-south-1.linodeobjects.com/server_launcher.exe".into(), file_manager::get_path_in_vizuara_folder("server_launcher.exe")).await;
          
          //downloading thumbnails.zip
          emit_event(window.to_owned(), &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), "Downloading thumbnails".into());
          download_data_and_extract(window.to_owned(), "https://vizuaraserver.ap-south-1.linodeobjects.com/thumbnails.zip".into(), "thumbnails".into(), "".into()).await;

          if !file_manager::does_data_struct_keyfile_exists() {
            //
            if is_first_launch {
              update_and_start(window.to_owned(), server_data_version).await;
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
            update_and_start(window.to_owned(), server_data_version).await;
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

      prepare_to_launch(window).await;
      /*#endregion */
    }
  }
  /*#endregion */
 
}

async fn update_and_start(window: tauri:: Window, server_data_version: String){

  //First time user, allow them to download the data struct key file from the server
  //println!("Trying to save data struct");
  if !data_structure_manager::save_data_struct_from_server().await.is_ok(){

    exit_after_10_seconds(window.to_owned(), 
    Event_Messages.UNEXPECTED_ERROR_SAVING_DATA_STRUCT_FAILED().into()).await;

  }else{              

    if user_preference_manager::set_new_data_version(server_data_version).is_ok() {
      println!("Updating is not okay");
      prepare_to_launch(window).await;
    }else{
      println!("Updating is not okay");
    }
  }

}

///<summary>
/// Function dropped
///</summary>
fn check_serial_number()-> Result<(),()>{
      //get the command line result
      /*
    let output_result  = Command::new("wmic")
                                .arg("bios")
                                .arg("get")
                                .arg("serialnumber")
                                .output().expect("Something happened!?");
    let commandline_output_result = String::from_utf8_lossy(&output_result.stdout).to_string();
    
    //and read the line 1
    let mut serialnumber: String = "".into();
    let mut count = 0;
    for line in commandline_output_result.lines() {
        if count == 1 {
            serialnumber = line.into();
            println!("{}", line);
        }
        count += 1;
    }

    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join("Vizuara");
    let (key, _disp) = hklm.create_subkey(&path).unwrap();
    let value: Result<String, std::io::Error> = key.get_value("ID");

    if value.is_ok() {
      if value.unwrap() == serialnumber {
        Ok(())
      }else{
        Err(())
      }
    }else{
      Err(())
    }*/
    Ok(())
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

          if check_serial_number().is_ok() { 
            emit_event(window.to_owned(), 
            &Event_Constants.GET_LOADING_DESCRIPTION_EVENT(), 
            Event_Messages.LAUNCHING_APPLICATION().into());

            println!("Launching Server Application at {:?}", &file_manager::get_server_launcher_file_in_vizuara_folder().to_string_lossy().to_string());
            let mut com: Command = Command::new(file_manager::get_server_launcher_file_in_vizuara_folder().to_string_lossy().to_string().replace("\\", "/"));
            com.arg(file_manager::get_vizuara_data_path().to_string_lossy().to_string());
            com.creation_flags(0x08000000);

            println!("Command is {:?}", com);
            com.spawn();

            println!("Application is Launching");
            emit_event(window.app_handle().get_window("main").to_owned().unwrap(),&Event_Constants.GET_INITIALIZATION_COMPLETED(), "".into());
          }
          /*#region Serial Number not the same */
          else{
              file_manager::delete_vizuara_directory_and_recreate();

              exit_after_10_seconds(window.to_owned(), 
               Event_Messages.PRODUCT_KEY_ERROR().into()).await;
          }
          /*#endregion */

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


fn check_for_license() -> bool {
        
  let licence_data: File= File::open("licence.lic").unwrap();
  let lines: Vec<String> = io::BufReader::new(licence_data)
             .lines()
             .map(|line|line.expect("Could not read line"))
             .collect();
  
  let mcrypt = new_magic_crypt!("magickey", 256); //Creates an instance of the magic crypt library/crate.
  let installation_date  = mcrypt.decrypt_base64_to_string(lines[0].to_string()).unwrap(); //Decrypts the string so we can read it.

  let expiry_date  = mcrypt.decrypt_base64_to_string(lines[1].to_string()).unwrap(); //Decrypts the string so we can read it.

  let _install_date_as_date = NaiveDate::parse_from_str(&installation_date, "%Y-%m-%d").unwrap();
  let _expiry_date_as_date = NaiveDate::parse_from_str(&expiry_date, "%Y-%m-%d").unwrap();

 let remaining_days = (_expiry_date_as_date - _install_date_as_date).num_days();
 
 if remaining_days < 0
 {
     return  false;
 }
 else {
     return  true;
 }

}


fn does_licence_file_exist() -> bool
{
  let mut valid_licence_file = Path::new("licence.lic").exists();
  return  valid_licence_file;
}

fn main() 
{
  let mut is_valid_licence = true;
  
  is_valid_licence = does_licence_file_exist();

  if is_valid_licence == false
  {
    let _ = win_msgbox::information::<Okay>(w!("No licence file found"))
        .title(w!("Vizura"))
        .show();
      std::process::exit(is_valid_licence as i32);
  }
  else 
  {
    is_valid_licence =  check_for_license();

    if is_valid_licence == false
    {
      let _ = win_msgbox::information::<Okay>(w!("Licence has expired"))
      .title(w!("Vizura"))
      .show();
       std::process::exit(is_valid_licence as i32);

    }
  }

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

      let _id = app.once_global("InitializationCompleted", move |event| {
        splashscreen_window.close().unwrap();
        main_window.show().unwrap();
      });
        Ok(())
        })
        .invoke_handler(tauri::generate_handler![quitapplication, resizewindow, minimizewindow, visitwebsite, extractcustom, loadingscreenloaded, getclassesdata, 
                                                
                                                set_class_id_method,set_chapter_id_method,set_file_type_method, set_file_method,
                                                 
                                                get_chapters_within_current_classes, download_data_and_extract, delete_chapter,
                                                 get_file_types, get_files, get_display_file_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
