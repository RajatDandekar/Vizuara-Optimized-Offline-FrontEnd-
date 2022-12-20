/*Author: Htet Aung Hlaing
Modified Data: 20th Dec, 2022
<summary>
Description: 
This script will handle logging and delogging of user_prefs.key file possibly using encryption and decryption to
further secure the key file from getting modified

Error Handling: 
If an error is raised while reading the file, we would assume that something has gone wrong, and we would 
delete everything inside the data folder, (a function to be implemented)
</summary>*/

/*#region Preparation */

use std::default;
/*#region importing necessities*/
/*#region importing necessary libraries */
use std::fs;
use std::path::PathBuf;
use std::path::Path;

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
    static ref USER_PREFERENCE_DATA: RwLock<UserPreferences> = RwLock::new(UserPreferences{version:Some(String::from("0.0.0"))});
}  

/*#endregion */

/*#endregion */

/*#region Functionalities */
    pub fn is_first_launch() -> bool{
        //Function to check if this is the first launch of this application
        //First we will check if the vizuara folder exists or not

        let mut is_first_launch: bool = false;

        if file_manager::does_vizuara_folder_exist() {
            if file_manager::does_user_preferences_keyfile_exist(){
                is_first_launch = false;
                get_user_preference_data();
            }else{
                file_manager::create_file(keyfile_pathbuf(), serde_json::to_string(&*USER_PREFERENCE_DATA.read().unwrap()).unwrap());
                is_first_launch =  true;
            }
        }else{
            file_manager::create_directory(file_manager::get_vizuara_data_path());
            file_manager::create_file(keyfile_pathbuf(), serde_json::to_string(&*USER_PREFERENCE_DATA.read().unwrap()).unwrap());
            is_first_launch = true;
        }

        is_first_launch
    }

    pub fn get_user_preference_data(){
        
        let mut global_user_preference_pointer = USER_PREFERENCE_DATA.write().unwrap();

        *global_user_preference_pointer = serde_json::from_str(&file_manager::read_file(keyfile_pathbuf())).unwrap();
        //let version_data: String = up.version.as_ref().unwrap().to_owned();
        //set_user_pref(Lazy::new(||UserPreferences{version:Some(String::from(version_data))}));
    }

    pub fn get_application_version() -> String{
        
        let up: &UserPreferences = &*USER_PREFERENCE_DATA.read().unwrap();
        let version: String = up.version.as_ref().unwrap().to_owned();
        println!("Application {}", version);


        "0.0.0".into()
    }
/*#endregion */

/*#region structs */
#[derive(Serialize, Deserialize)]
pub struct UserPreferences {
    version: Option<String>,
}

/*#endregion */