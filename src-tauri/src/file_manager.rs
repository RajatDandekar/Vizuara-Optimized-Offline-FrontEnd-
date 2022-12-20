/*Author: Htet Aung Hlaing
Creation Date: 20th Dec, 2022
Modified Date: 20th Dec, 2022
<summary>

Description: 
This script will handle file related operation regarding to the application including creating, deleting, modifying the data

Error Handling: 
If an error is raised while reading the file, we would assume that something has gone wrong, and we would delete everything 
inside the data folder, (a function to be implemented)

</summary>*/

/*#region Preparation */

/*#region importing necessary libraries*/
use std::option;
use std::path::PathBuf;
use std::path::Path;
use std::fs;
extern crate dirs;
/*#endregion*/

/*#region defining constants */
const VIZUARA_FOLDER_NAME: &str = "VizData";
const KEY_FILE_NAME: &str = "user_prefs.key";
/*#endregion */

/*#endregion */

/*#region Functions */

/*#region get the file paths adn checking */
/*<summary>
Getting the %APP_DATA% file path so that we can store our data or database or whatsoever
</summary>*/
pub fn get_local_data_path() -> PathBuf{
    return dirs::data_local_dir().unwrap();
}
/*<summary>
Getting the Vizuara folder from the %APP_DATA% directory
</summary>*/
pub fn get_vizuara_data_path() -> PathBuf{
    let data_local_path: PathBuf = get_local_data_path();
    let vizuara_path = Path::new(&data_local_path).join(&VIZUARA_FOLDER_NAME);
    PathBuf::from(vizuara_path)
}
/*
//<summary>
//Check if the vizuara folder exists or not
//</summary>
// */
pub fn does_vizuara_folder_exist() -> bool{
    check_if_path_exists(get_vizuara_data_path())
}
/*<summary>
Getting the key file that would store user_preferences
</summary>*/
pub fn get_user_preferences_keyfile_path() -> PathBuf{
    let vizuara_path: PathBuf = get_vizuara_data_path();
    let keyfile_path = Path::new(&vizuara_path).join(&KEY_FILE_NAME);
    PathBuf::from(keyfile_path)
}
/*
//<summary>
//Check if the vizuara folder exists or not
//</summary>
// */
pub fn does_user_preferences_keyfile_exist() -> bool{
    check_if_path_exists(get_user_preferences_keyfile_path())
}

/*#endregion FilePathGetter*/

/*#region Get File, Check if Exists, Create Folder, Create File, Append etc */

pub fn check_if_path_exists(path_buf: PathBuf) -> bool{
    let path = Path::new(&path_buf);
    path.exists()
}

pub fn create_directory(directory_pathbuf: PathBuf){
    let directory_path = Path::new(&directory_pathbuf);
    if fs::create_dir(directory_path).is_ok(){
        println!("Directory created successfully")
    }else{
        println!("Directory creation failed");
    }
}

pub fn create_file(file_pathbuf: PathBuf, content: String){
    let file_path = Path::new(&file_pathbuf);
    if fs::write(file_path, content).is_ok(){
        println!("File Created Successfully")
    }else{
        println!("File Creation Failed");
    }
}

pub fn read_file(file_pathbuf: PathBuf) -> String{
    let file_path: &Path = Path::new(&file_pathbuf);
    fs::read_to_string(file_path).unwrap()
}
/*#endregion */

/*#endregion */