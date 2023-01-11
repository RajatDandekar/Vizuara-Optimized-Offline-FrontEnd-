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
use std::path::PathBuf;
use std::path::Path;
use std::fs;

use magic_crypt::MagicCryptTrait;
/*#endregion*/

/*#region defining constants */
const VIZUARA_CRYPTO_KEY: &str = "vizuara_encryption_key_cQpUL0F4nvvkjz3irzFy";

const VIZUARA_FOLDER_NAME: &str = "VizData";            //<- The parent folder that will hold all the data for this application
const KEY_FILE_NAME: &str = "user_prefs.key";           //<- This file will contain all the data related to this user
const DATA_STRUCT_NAME: &str = "data_struct.key";       //<- This file will contain almost all of structure related data, url links, and all that
const SERVER_FILE_NAME: &str = "server_launcher.exe";   //<- Server Launcher Exe will launch

/*#region thumbnails */
const THUMBNAILS_FOLDER_NAME: &str = "thumbnails";

    /*#region thumbnails for classes and chapters */
        const CLASSES_THUMBNAILS_FOLDER_NAME: &str = "classes";
        const CHAPTERS_THUMBNAILS_FOLDER_NAME: &str = "chapters";
    /*#endregion */

    /*#region thumbnails for each data type */
    const LABORATORIES: &str = "lab";
    const VIDEOS : &str = "vid";
    /*#endregion */

/*#endregion */

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

//<summary>
//Get a path in Vizuara Folder
//</summary>
pub fn get_path_in_vizuara_folder(pathToUse: &str) -> PathBuf{
    let vizuara_folder_path: PathBuf = get_vizuara_data_path();
    let path_in_folder: PathBuf = Path::new(&vizuara_folder_path).join(&pathToUse);
    PathBuf::from(path_in_folder)
}

//<summary>
//Get server launcher file
//</summary>
pub fn get_server_launcher_file_in_vizuara_folder() -> PathBuf{
    get_path_in_vizuara_folder(SERVER_FILE_NAME)
}

//<summary>
//Check if a path in vizuara folder exists or not
//</summary>
pub fn does_folder_exist_in_vizuara_folder(pathToUse: &str) -> bool{
    check_if_path_exists(get_path_in_vizuara_folder(pathToUse))
}



//<summary>
//List all the folders within a folder
//</summary>
pub fn list_all_within_a_folder(pathToUse: PathBuf) -> Result<Vec<String>,()>{
    let mut allpaths: Vec<String> = Vec::new();
    for file in fs::read_dir(pathToUse).unwrap() {
        allpaths.insert(0, file.unwrap().file_name().into_string().unwrap());
    }
    Ok(allpaths)
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

/*<summary>
//This function will allow you to get data_struct key path
//</summary>
*/
pub fn get_data_struct_keyfile_path() -> PathBuf{
    let vizuara_path: PathBuf = get_vizuara_data_path();
    let struct_file_path = Path::new(&vizuara_path).join(&DATA_STRUCT_NAME);
    PathBuf::from(struct_file_path)
}

/*
//<summary>
//Check if the vizuara folder exists or not
//</summary>
// */
pub fn does_data_struct_keyfile_exists() -> bool{
    check_if_path_exists(get_data_struct_keyfile_path())
}
/*#endregion FilePathGetter*/

//<summary>
//get thumbnails folder
//</summary>
pub fn get_thumbnails_folder() -> PathBuf{
    let vizuara_path: PathBuf = get_vizuara_data_path();
    let thumbnails_directory_path = Path::new(&vizuara_path).join(&THUMBNAILS_FOLDER_NAME);
    PathBuf::from(thumbnails_directory_path)
}

//<summary>
//check if the thumbnails folder exists
//</summary>
pub fn does_thumbnails_folder_exist() -> bool{
    check_if_path_exists(get_thumbnails_folder())
}

//<summary>
//get thumbnails folder of classes
//</summary>
pub fn get_classes_thumbnails_folder() -> PathBuf{
    let thumbails_directory: PathBuf = get_thumbnails_folder();
    let classes_thumbnails_directory_path = Path::new(&thumbails_directory).join(&CLASSES_THUMBNAILS_FOLDER_NAME);
    PathBuf::from(classes_thumbnails_directory_path)
}

//<summary>
//get thumbnail of chapters
//</summary>
pub fn get_class_thumbnail(thumbnail_id: String) -> PathBuf{
    let classes_thumbnail_directory: PathBuf = get_classes_thumbnails_folder();

    println!("file path is {:?}",&thumbnail_id);
    let mut temp_thumbnail_id = (&thumbnail_id).to_owned();
    temp_thumbnail_id.push_str(".png".into());
    let str_thumbnail_id: &str = temp_thumbnail_id.as_str();
    let new_path:String = (&str_thumbnail_id.replace("\"", "")).replace("\\", "/");

    let class_thumbail_path = Path::new(&classes_thumbnail_directory).join(new_path);
    PathBuf::from(class_thumbail_path)
}

//<summary>
//get chapters folder of classes
//</summary>
pub fn get_chapters_thumbnails_folder() -> PathBuf{
    let thumbnails_directory: PathBuf = get_thumbnails_folder();
    let chapters_thumbnails_directory_path = Path::new(&thumbnails_directory).join(&CHAPTERS_THUMBNAILS_FOLDER_NAME);
    PathBuf::from(chapters_thumbnails_directory_path)
}

//<summary>
//get thumbnail of chapters
//</summary>
pub fn get_chapter_thumbnail(thumbnail_id: String) -> PathBuf{
    let chapter_thumbnails_directory: PathBuf = get_chapters_thumbnails_folder();
    let chapter_thumbnail_path = Path::new(&chapter_thumbnails_directory).join(thumbnail_id + ".jpg");
    PathBuf::from(chapter_thumbnail_path)
}

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
    println!("key file asking to be created!");
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