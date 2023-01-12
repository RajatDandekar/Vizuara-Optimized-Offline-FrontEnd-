extern crate my_internet_ip;
extern crate reqwest;

use std::string::String;

const VERSION_HANDLER: &str = "https://vizuara-optimized-offline-2.web.app/application/version.html";
const DATA_VERSION_HANDLER: &str = "https://vizuara-optimized-offline-2.web.app/application/data_version.html";
const DATA_STRUCT_CONTROLLER: &str = "https://vizuara-optimized-offline-2.web.app/application/data_struct_controller.html";

pub async fn make_connection(connection_path: &str) -> Result<String, String>{

    //if connection okay, version will be returned
    //if connection failed, conneciton error

    let body = reqwest::get(connection_path).await;
    if body.is_ok() {
            /*.ok().unwrap()
    .text()
    .await;*/

        Ok(body.ok().unwrap().text().await.ok().unwrap().into())
    }else{
        println!("Error Occurred");
        Err("Error Occurred".into())
    }
}

/*#region Check Application verion while also checking connection state */
pub async fn check_application_version_and_connection_state() -> Result<String, String>{
    make_connection(VERSION_HANDLER).await
}

pub async fn check_data_version_and_connection_state() -> Result<String, String>{
    make_connection(DATA_VERSION_HANDLER).await
}

pub async fn get_data_struct_controller() -> Result<String, String>{
    make_connection(DATA_STRUCT_CONTROLLER).await
}

pub async fn get_data_struct() -> Result<String, String>{
    let get_data_struct_controller_result = get_data_struct_controller().await;
    if get_data_struct_controller_result.is_ok() {
        make_connection(get_data_struct_controller_result.ok().unwrap().as_str()).await
    }else{
        Err("failed to get data struct data".into())
    }
}

