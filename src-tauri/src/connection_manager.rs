extern crate my_internet_ip;

pub fn check_connection()-> bool{
    match my_internet_ip::get(){
        Ok(ip) =>{
            return true;
        },
        Err(e) =>{
            return false;
        }
    }
}