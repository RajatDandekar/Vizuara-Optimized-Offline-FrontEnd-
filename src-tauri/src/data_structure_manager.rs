/*Author: Htet Aung Hlaing
Modified Data: 22th Dec, 2022
<summary>
Description: 
This script will handle data related to chapters, classes, and their files

Error Handling: 
If an error is raised while reading the file, we would assume that something has gone wrong, and we would 
delete everything inside the data folder possibly, (a function to be implemented) 

1. Unparsable JSON
2. User Preference Exists but Struct Key does not exist (DONE)

Action -> Delete the entire folder and create new one, and then restart the application
</summary>*/

/*#region importing necessary libraries */
use crate::connection_manager;
use crate::file_manager;
extern crate lazy_static;

use serde_json::{Result, Value, Map};
use std::sync::RwLock;

use lazy_static::lazy_static;
/*#endregion */

/*#region constant variable */
lazy_static!{
    static ref DATA_STRUCT_DATA: RwLock<Option<Value>> = RwLock::new( 
        None
    );

    //This variable will contain the classes struct
    //id -> class name
    //this variable won't be changed after initial initialization
    static ref CLASSES_STRUCT: RwLock<serde_json::Map<String, Value>> = RwLock::new(
        serde_json::Map::new()
    );

    static ref CHAPTERS_STRUCT: RwLock<serde_json::Map<String,Value>> = RwLock::new(
        serde_json::Map::new()
    );
}  

pub fn insert_to_static_classes_struct(class_id: String, class_name: String){
    let mut global_classes_struct = CLASSES_STRUCT.write().unwrap();
    (*global_classes_struct).insert(class_id, serde_json::Value::String(class_name));
    drop(global_classes_struct);
}

pub fn find_in_static_classes_struct_with_class_id(class_id: String) -> std::result::Result<String, ()>{
    let binding = (*CLASSES_STRUCT).read().unwrap();
    let class = binding.get(&class_id.to_owned().to_string());
    if (&class).is_some() {
        //no error
        Ok(class.unwrap().to_string())
    }else{
        Err(())
    }
}

pub fn clear_static_chapters_struct(){
    let mut global_chapters_struct = CHAPTERS_STRUCT.write().unwrap();
    (*global_chapters_struct).clear();
    drop(global_chapters_struct);
}

pub fn insert_to_static_chapters_struct(chapter_id: &str, chapter_details: Value){
    let mut global_chapters_struct = CHAPTERS_STRUCT.write().unwrap();
    (*global_chapters_struct).insert(chapter_id.to_string(), chapter_details);
    println!("adding {}", chapter_id.to_string());
    drop(global_chapters_struct);
}

pub fn find_in_static_chapters_struct_with_chapter_id(chapter_id: String)-> std::result::Result<Map<String, Value>,()>{
    let binding = (*CHAPTERS_STRUCT).read().unwrap();
    let chapter = binding.get(&chapter_id.to_owned().to_string());
    if (&chapter).is_some() {
        //no error
        Ok(chapter.unwrap().as_object().to_owned().unwrap().to_owned())
    }else{
        Err(())
    }
}
/* 
    <summary>
    These variables will be used to check the data integrity
    basically by checking if those keys exist within the data struct variable
    </summary>
*/
const DATA_STRUCT_VARIABLE_TO_CHECK_1: &str = "O1xqt41b";

const DATA_STRUCT_VARIABLE_TO_CHECK_2: &str = "UaYp2sG1";

/*#endregion */

/*
<summary>
    download from the server and process it. And then run it
</summary>
*/
pub async fn save_data_struct_from_server() -> std::result::Result<(),()>{
    let get_data_struct_state = connection_manager::get_data_struct().await;
    
    if get_data_struct_state.is_ok() {
        let data_struct = get_data_struct_state.ok().unwrap();

        //println!("Data Struct File {}", data_struct);
        file_manager::create_file(file_manager::get_data_struct_keyfile_path(), data_struct);
        Ok(())
    }else{
        //do nothing and possibly reset the application
        println!("Something has occurred");
        Err(())
    }
}

pub async fn read_and_check_and_save_data_struct_file_integrity() -> std::result::Result<(),()>{
    if !file_manager::does_data_struct_keyfile_exists() {
        Err(())
    }else{
        //We will try to parse the data into the global variable
        //if failed Err()
        //If parsed successfully -> Ok()
        
        let saving_result = read_and_save_to_global_data_struct_variable().await;
        
        /*#region saved to variable */
        if saving_result.is_ok() {
            if data_integrity_is_valid(){
                Ok(())
            }else{
                Err(())
            }
        }
        /*#endregion */

        /*#region Saving Failed some type of error has occured */
        else{
            Err(())
        }
    }
}

fn data_integrity_is_valid() -> bool{
    let data_struct: &Option<Value> = &*DATA_STRUCT_DATA.read().unwrap();
    println!("Data integrity function triggered!");
    if data_struct.is_some(){
            check_if_key_exists_in_data_struct(DATA_STRUCT_VARIABLE_TO_CHECK_1.into()) && 
            check_if_key_exists_in_data_struct(DATA_STRUCT_VARIABLE_TO_CHECK_2.into())
    }
    else{
        false
    }
}

fn check_if_key_exists_in_data_struct(key_name: String) -> bool {
    let data_struct_option: &Option<Value> = &*DATA_STRUCT_DATA.read().unwrap();
    let data_struct_option_result = &data_struct_option.as_ref();

    if data_struct_option_result.is_some() {
        let data_struct = data_struct_option_result.unwrap();
        data_struct.get(key_name).is_some()
    }else{
        false
    }
}

fn get_key(key_name: String) -> std::result::Result<Value,()>{
    if check_if_key_exists_in_data_struct((&key_name).to_owned()) {

        let data_struct_option: &Option<Value> = &*DATA_STRUCT_DATA.read().unwrap();
        let data_struct_option_result = &data_struct_option.as_ref();

        if data_struct_option_result.is_some() {
            let data_struct = data_struct_option_result.unwrap();
            Ok(data_struct.get(key_name).unwrap().to_owned())
        }else{
            Err(())
        }
    
    }else{
        Err(())
    }
}

pub fn get_class_basic_data(class_id: String) -> std::result::Result<Map<String, Value>,()> {
    //println!("Trying to get class key");
    let get_key_result = get_key(class_id);
    if get_key_result.is_ok() {
        let classes = get_key_result.ok().unwrap();
        println!("{:?}", classes);
        let mut classes_basic_data = Map::new();

        let class_id = classes.get("I").unwrap().to_owned();

        classes_basic_data.insert("id".into(), (&class_id).to_owned());
        classes_basic_data.insert("name".into(), classes.get("N").unwrap().to_owned());
        classes_basic_data.insert("thumbnailpath".into(), Value::String(file_manager::get_class_thumbnail(class_id.to_string()).into_os_string().into_string().unwrap()));

        Ok(classes_basic_data)
    }else{
        println!("Failed to get data");
        Err(())
    }
}

pub fn insert_chapters_to_variable(class_id: String) -> std::result::Result<Map<String,Value>, ()> {
    //we would assume that the class exists so we would just display the data
        //println!("Trying to get class key");
    let get_key_result = get_key(class_id);
    if get_key_result.is_ok() {
        let classes = get_key_result.ok().unwrap();
        println!("{:?}", (&classes).to_owned());

        let available_chapters: Map<String,Value> = classes.get("C").unwrap().to_owned().as_object().unwrap().to_owned();
        //println!("{:?}",available_chapters);

        let mut loop_counter = 1;
        loop{

            let get_individual_chapter_result: Option<&Value> = (&available_chapters).get(&loop_counter.to_string());
            if (&get_individual_chapter_result).is_none() {
                break;
            }else{
                let chapter_detail_option: Option<&Map<String,Value>> = get_individual_chapter_result.unwrap().as_object();
                if (&chapter_detail_option).is_none() {
                    break;
                }else{
                    let chapter_detail: Map<String,Value> = chapter_detail_option.unwrap().to_owned();

                    let mut insertation_chapter_detail : Map<String, Value> = Map::new();
                    
                    //println!("{:?}", (&chapter_detail).to_owned());

                    insertation_chapter_detail.insert("name".into(), (&chapter_detail).get("N").unwrap().to_owned());
                    insertation_chapter_detail.insert("subname".into(), (&chapter_detail).get("P").unwrap().to_owned());
                    insertation_chapter_detail.insert("link".into(), (&chapter_detail).get("L").unwrap().to_owned());
                    insertation_chapter_detail.insert("id".into(), (&chapter_detail).get("I").unwrap().to_owned());

                    insert_to_static_chapters_struct(&(&loop_counter).to_owned().to_string(), Value::Object(insertation_chapter_detail));
                    println!("{:?}",(&chapter_detail).get("I").unwrap().as_str().unwrap());
                }
            }
            loop_counter+=1;
        }
        Ok(Map::new())
    }else{
        println!("Failed to get data");
        Err(())
    }
}

async fn read_data_struct_file() -> std::result::Result<Value, ()>{

    println!("Reading data struct file from {:?}", file_manager::get_data_struct_keyfile_path());

    let read_file_result = file_manager::read_file(file_manager::get_data_struct_keyfile_path());
    println!("{}", &read_file_result);

    let json_parse_result = serde_json::from_str(read_file_result.as_str());
    if json_parse_result.is_ok(){
        Ok(json_parse_result.unwrap())
    }else{
        println!("{:?}", json_parse_result.err());
        Err(())
    }
}

async fn read_and_save_to_global_data_struct_variable() -> std::result::Result<(),()>{

    println!("reading and saving");

    let read_data_struct_file_result = read_data_struct_file().await;

    if read_data_struct_file_result.is_ok(){
        save_to_global_data_struct_variable(read_data_struct_file_result.unwrap());
        Ok(())
    }else{
        println!("data struct error");
        Err(())
    }
}

fn save_to_global_data_struct_variable(data_struct_new_value : Value){

    let mut global_data_struct_pointer = DATA_STRUCT_DATA.write().unwrap();
    *global_data_struct_pointer = Some(data_struct_new_value);
}
