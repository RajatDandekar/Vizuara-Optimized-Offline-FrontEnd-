/*Author: Htet Aung Hlaing
Modified Data: 20th Dec, 2022
<summary>
Description: 
This script will handle logging and delogging of user_prefs.key file possibly using encryption and decryption to
further secure the key file from getting modified

will store whether if user have downloaded certain files onto their file systems

Error Handling: 
If an error is raised while reading the file, we would assume that something has gone wrong, and we would 
delete everything inside the data folder, (a function to be implemented)
</summary>*/

/*#region Preparation */

/*#region importing necessities*/
/*#region importing necessary libraries */
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::sync::RwLock;
/*#endregion */

/*#region importing necessary modules*/
use crate::file_manager;
extern crate lazy_static;

use lazy_static::{lazy_static, __Deref};
/*#endregion */

/*#endregion */

/*#region dependent variables */
fn keyfile_pathbuf() -> PathBuf{
    /*<summary>
    //Getting Key File Path from this function, treat this as a global varibale that could be accessible from every part of this script
    </summary>*/

    file_manager::get_user_preferences_keyfile_path()
}


lazy_static!{
    static ref USER_PREFERENCE_DATA: RwLock<UserPreferences> = RwLock::new( UserPreferences{
        data_version        : Some("0.0.0".into()),
        downloaded_chapters : Some(Vec::new())
        /*classes_file_name : None,
        chapters_file_name: None,
        file_type_file_name: None,
        files_file_name: None*/
    }
    );
}  

/*#endregion */

/*#endregion */

/*#region Functionalities */
    pub fn is_first_launch() -> bool{
        //Function to check if this is the first launch of this application
        //First we will check if the vizuara folder exists or not

        if file_manager::does_vizuara_folder_exist() {
            if file_manager::does_user_preferences_keyfile_exist(){
                let get_user_preference_state = get_user_preference_data();
                if get_user_preference_state.is_ok(){
                    //which means data is likely to be correct
                    let get_data_version_state = get_data_version();
                    if get_data_version_state.is_ok(){
                        false
                    }else{

                        println!("File Exists but data seems to have been corrupted as we cannot find app-version which is the most basic value of all! Consider deleting
                        the data in the entire directory!");
                        true
                        //Some Error has occurred and value that should exist within the application does not exist anymore! All data inside the folder should be deleted
                    }
                }else{

                    //User Preference Manager Key File seems to be corrupted! Delete it asap!
                    file_manager::delete_vizuara_directory_and_recreate();
                    true
                }
            }else{
                //file_manager::create_file_with_encryption(keyfile_pathbuf(), serde_json::to_string(&*USER_PREFERENCE_DATA.read().unwrap()).unwrap());
                true
            }
        }else{
            file_manager::create_directory(file_manager::get_vizuara_data_path());
            //file_manager::create_file_with_encryption(keyfile_pathbuf(), serde_json::to_string(&*USER_PREFERENCE_DATA.read().unwrap()).unwrap());
            true
        }
    }

    //gotten user_preference_data successfully!
    pub fn get_user_preference_data() -> std::result::Result<(),()>{

        //println!("Getting User Preference Data");

        let read_file_result : String = file_manager::read_file(keyfile_pathbuf());

        let json_parsing_result : Result<UserPreferences> = serde_json::from_str(&read_file_result);
        
            if json_parsing_result.is_ok() {
            //executed successfully!
            let mut global_user_preference_pointer = USER_PREFERENCE_DATA.write().unwrap();
            *global_user_preference_pointer = json_parsing_result.unwrap();
            Ok(())
            }else{
                Err(())
            }

        //let version_data: String = up.version.as_ref().unwrap().to_owned();
        //set_user_pref(Lazy::new(||UserPreferences{version:Some(String::from(version_data))}));
    }

    pub fn get_data_version() -> std::result::Result<String,()>{
        
        let up: &UserPreferences = &*USER_PREFERENCE_DATA.read().unwrap();
        
        let app_version: &Option<String> = &up.data_version;
        if app_version.is_some(){
            Ok(up.data_version.as_ref().unwrap().to_owned().into())
        }else{
            Err(())
        }
    }

    pub fn set_new_data_version(new_data_version: String) -> std::result::Result<(),()>{
        let mut global_user_preference_pointer = USER_PREFERENCE_DATA.write().unwrap();
        (*global_user_preference_pointer).data_version = Some(new_data_version);
        drop(global_user_preference_pointer);

        save_from_variable_to_keyfile()
    }

    pub fn add_new_downloaded_chapter(new_chapter_id: String, new_chapter_version: String) -> std::result::Result<(),()>{

        let mut downloaded_chapter_data: Vec<String> = get_downloaded_chapters_data();
        
        if new_chapter_version.len() > 0 {
            downloaded_chapter_data.insert(0, new_chapter_id+"."+&new_chapter_version);
        }
        else{
            downloaded_chapter_data.insert(0, new_chapter_id);
        }

        let mut global_user_preference_pointer = USER_PREFERENCE_DATA.write().unwrap();
        (*global_user_preference_pointer).downloaded_chapters = Some(downloaded_chapter_data);
        drop(global_user_preference_pointer);

        save_from_variable_to_keyfile()
    }

    pub fn remove_downloaded_chapter(new_chapter_id: String) -> std::result::Result<(),()>{

        let mut downloaded_chapter_data: Vec<String> = get_downloaded_chapters_data();
        
        let is_chapter_downloaded: std::result::Result<usize, ()> = check_if_chapter_is_downloaded(new_chapter_id);
        if is_chapter_downloaded.is_ok() {

            let index_to_remove = is_chapter_downloaded.unwrap();

            downloaded_chapter_data.remove(index_to_remove);

            let mut global_user_preference_pointer = USER_PREFERENCE_DATA.write().unwrap();
            (*global_user_preference_pointer).downloaded_chapters = Some(downloaded_chapter_data);
            drop(global_user_preference_pointer);
    
            save_from_variable_to_keyfile()

        }else{

            Ok(())

        }
    }

    pub fn check_if_chapter_is_downloaded(chapter_id: String) -> std::result::Result<usize, ()> {
        let up: &UserPreferences = &*USER_PREFERENCE_DATA.read().unwrap();
        
        let downloaded_chapters_option: &Option<Vec<String>> = &up.downloaded_chapters;
        if downloaded_chapters_option.is_some(){
            let downloaded_chapters: Vec<String> = downloaded_chapters_option.as_ref().unwrap().to_owned();
            
            let chapt : String = "".into();

            fn get_chapterid_from_element(chapters_in_question: String) -> String{
                if(chapters_in_question.contains('.')) {
                    let mut loop_counter = 0;
                    let mut string_to_return: String = "".into();

                    for codes in chapters_in_question.split(".") {
                        if(loop_counter == 0){
                            string_to_return = codes.to_owned().into();
                        }
                        loop_counter += 1;
                    }
                    string_to_return
                }else{
                    chapters_in_question
                }
            }


            let index = downloaded_chapters.iter().position(|element|  
                

                get_chapterid_from_element(element.to_owned()) == chapter_id
            
            );
            if(index.is_some()){
                Ok(index.unwrap())
            }else{
                Err(())
            }
        }else{
            Err(())
        }
    }

    pub fn check_chapter_version(chapter_id: String) -> String{
        let mut downloaded_chapter_data: Vec<String> = get_downloaded_chapters_data();
        
        let is_chapter_downloaded: std::result::Result<usize, ()> = check_if_chapter_is_downloaded(chapter_id);
        if is_chapter_downloaded.is_ok() {

            let index_to_check = is_chapter_downloaded.unwrap();

            let chapters_in_question = downloaded_chapter_data.get(index_to_check).unwrap().to_owned();
            if chapters_in_question.contains('.') {
                let mut loop_counter = 0;
                let mut string_to_return : String = "".into();

                for codes in chapters_in_question.split(".") {
                    if(loop_counter == 1){
                        string_to_return = codes.to_owned().into();
                    }
                    loop_counter += 1;
                }
                string_to_return
            }else{
                "".into()
            }
        }else {
            "".into()
        }    
    }

    fn get_downloaded_chapters_data() -> Vec<String>{
        let up: &UserPreferences = &*USER_PREFERENCE_DATA.read().unwrap();
        if (&up).downloaded_chapters.to_owned().is_some() {
            (&up).downloaded_chapters.to_owned().unwrap()
        }else{
            Vec::new()
        }
    }

    pub fn save_from_variable_to_keyfile() -> std::result::Result<(),()>{
        //println!("Being Saved!");
        //let upd_temp = &*USER_PREFERENCE_DATA.read().expect("something went wrong");
        //println!("After getting");

        file_manager::create_file(keyfile_pathbuf(), serde_json::to_string(&*USER_PREFERENCE_DATA.read().unwrap()).unwrap());
        Ok(())
    }
/*#endregion */

/*#region structs */
#[derive(Serialize, Deserialize)]
pub struct UserPreferences {
                                    //<- Version of the application, determines whether the application needs to be updated or not
    data_version: Option<String>,   //<- Data Version of the application, determines whether the application needs to update by downloading data within application
    
    downloaded_chapters: Option<Vec<String>>
    /*
    classes_file_name: Option<String>,
    chapters_file_name: Option<String>,
    file_type_file_name: Option<String>,
    files_file_name:Option<String>
    */
}

/*#endregion */