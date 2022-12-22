extern crate my_internet_ip;
extern crate reqwest;

use std::string::String;

const VERSION_HANDLER: &str = "https://vizuara-optimized-offline.web.app/application/version.html";
const DATA_VERSION_HANDLER: &str = "https://vizuara-optimized-offline.web.app/application/data_version.html";

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
