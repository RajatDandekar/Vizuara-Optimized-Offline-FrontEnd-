extern crate dirs;
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

use magic_crypt::MagicCrypt256;
use magic_crypt::MagicCryptTrait;
/*#endregion*/

/*#region defining constants */
const VIZUARA_CRYPTO_KEY: &str = "vizuara_encryption_key_cQpUL0F4nvvkjz3irzFy";

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

pub fn create_file_with_encryption(file_pathbuf: PathBuf, content: String){
    //content will be encrypted using some kind of encryption method before saving
    let cryptographic_instance = new_magic_crypt!(VIZUARA_CRYPTO_KEY, 64);
    create_file(file_pathbuf, cryptographic_instance.encrypt_str_to_base64(content))
}

pub fn read_file(file_pathbuf: PathBuf) -> String{
    let file_path: &Path = Path::new(&file_pathbuf);
    fs::read_to_string(file_path).unwrap()
}

pub fn read_file_with_decryption(file_pathbuf: PathBuf) -> Result<String, String>{

    let original_content: String = read_file(file_pathbuf);

    let cryptographic_instance = new_magic_crypt!(VIZUARA_CRYPTO_KEY, 64);
    let decryption_result = cryptographic_instance.decrypt_base64_to_string(&original_content);
    if decryption_result.is_ok() {
        //decryption is okay
        println!("Encrypted String {}", original_content);
        println!("Decrypted String {}", decryption_result.as_ref().unwrap());
        Ok(decryption_result.unwrap())
    }else{
        Err("Failed to read data".into())
        //decryption is not okay
    }
}

/*#region directory deletion */
//delete files in the directory
pub fn delete_folder(directory_pathbuf:PathBuf) -> Result<(),()>{
    let directory_path: &Path = Path::new(&directory_pathbuf);
    println!("{}", directory_path.display());

    let remove_dir_result = fs::remove_dir_all(directory_path);
    if remove_dir_result.is_ok(){
        println!("directory deleted successfully!");
        Ok(())
    }else{
        println!("Error: cannot delete directory -> {}", remove_dir_result.err().unwrap());
        Err(())
    }
}

pub fn delete_folder_and_recreate(directory_pathbuf: PathBuf) -> Result<(),()>{
    if delete_folder(directory_pathbuf.to_path_buf()).is_ok() {
        create_directory(directory_pathbuf);
        Ok(())
    }else{
        Err(())
    }
}

pub fn delete_vizuara_directory_and_recreate() -> Result<(),()>{
    delete_folder_and_recreate(get_vizuara_data_path())
}

/*#endregion */

/*#endregion */

/*#endregion */