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

use lazy_static::lazy_static;
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
        data_version: Some("0.0.0".into()),

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
                file_manager::create_file_with_encryption(keyfile_pathbuf(), serde_json::to_string(&*USER_PREFERENCE_DATA.read().unwrap()).unwrap());
                true
            }
        }else{
            file_manager::create_directory(file_manager::get_vizuara_data_path());
            file_manager::create_file_with_encryption(keyfile_pathbuf(), serde_json::to_string(&*USER_PREFERENCE_DATA.read().unwrap()).unwrap());
            true
        }
    }

    //gotten user_preference_data successfully!
    pub fn get_user_preference_data() -> std::result::Result<(),()>{

        println!("Getting User Preference Data");

        let read_file_result : std::result::Result<String, String> = file_manager::read_file_with_decryption(keyfile_pathbuf());

        if read_file_result.is_ok() {
        let json_parsing_result : Result<UserPreferences> = serde_json::from_str(&read_file_result.unwrap());
        
        if json_parsing_result.is_ok() {
            //executed successfully!
            let mut global_user_preference_pointer = USER_PREFERENCE_DATA.write().unwrap();
            *global_user_preference_pointer = json_parsing_result.unwrap();
            Ok(())
            }else{
                Err(())
            }
        }else{
            println!("{:?}", read_file_result.err().unwrap());
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
/*#endregion */

/*#region structs */
#[derive(Serialize, Deserialize)]
pub struct UserPreferences {
                                    //<- Version of the application, determines whether the application needs to be updated or not
    data_version: Option<String>,   //<- Data Version of the application, determines whether the application needs to update by downloading data within application
    
    /*
    classes_file_name: Option<String>,
    chapters_file_name: Option<String>,
    file_type_file_name: Option<String>,
    files_file_name:Option<String>
    */
}

/*#endregion */