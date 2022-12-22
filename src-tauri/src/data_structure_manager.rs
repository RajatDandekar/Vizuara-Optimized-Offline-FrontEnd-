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

use serde;
use serde_json;
/*#endregion */

/*
<summary>
    download from the server and process it. And then run it
</summary>
*/
pub async fn save_data_struct_from_server() -> Result<(),()>{
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

pub async fn read_and_check_data_struct_file_integrity() -> Result<(),()>{
    if !file_manager::does_data_struct_keyfile_exists() {
        Err(())
    }else{
        //We will try to parse the data into the global variable
        //if failed Err()
        //If parsed successfully -> Ok()
        Ok(())
    }
}
/*
<summary>
    read from data struct file
</summary>
*/

