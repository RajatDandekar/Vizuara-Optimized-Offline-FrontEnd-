/*Author: Htet Aung Hlaing
Modified Data: 22th Dec, 2022
<summary>
Description: 
This script will handle the consts value that would be used inside main.rs

Error Handling: 
None
</summary>*/

/*
    <summary>
        Error Codes
        550 -> Corrupted Data Struct Key
        549 -> Could not save data struct
    </summary>
*/

pub const APPLICATION_VERSION: &str = r#"0.0.1"#;

pub struct Event_Constants;
impl Event_Constants{
    pub const fn GET_LOADING_DESCRIPTION_EVENT(&self) -> &str{
        "LoadingDescription"
    }

    pub const fn GET_APPLICATION_VERSION_EVENT(&self) -> &str{
        "ApplicationVersion"
    }
}

pub struct Event_Messages;
impl Event_Messages{
    pub const fn WELCOME(&self) -> &str{
        "Welcome to Vizuara: Teacher's Portal"
    }

    pub const fn WELCOME_BACK(&self) -> &str{
        "Welcome back to Vizuara: Teacher's Portal"
    }

    pub const fn VIZUARA_INITIALIZING(&self) -> &str{
        "Vizuara: Initializing"
    }

    pub fn CLOSING_IN_WITH_MESSAGES(&self, message_to_add: String, number: i32) -> String{
        message_to_add + Event_Messages.CLOSING_IN(number).as_str()
    }

    fn CLOSING_IN(&self, number: i32) -> String{
        " Closing in ".to_string() + number.to_string().as_str()
    }

    pub const fn FIRST_LAUNCH_CLOSING_MESSAGE(&self) -> &str{
        "Internet connection is required for first launches! "
    }

    pub const fn ONLINE_MODE_NOT_AVAILABLE_ENTERING_OFFLINE_MODE(&self) -> &str{
        "Failed to connect! Entering Offline Mode "
    }

    pub const fn UPDATE_APPLICATION(&self) -> &str{
        "An Update is available! Please update the application!"
    }

    pub const fn UNEXPECTED_ERROR_CORRUPTED_DATA_STRUCT_KEY(&self) -> &str{
        "Exception:550. DELETING ALL DATA TO ENSURE DATA INTEGRITY"
    }

    pub const fn UNEXPECTED_ERROR_SAVING_DATA_STRUCT_FAILED(&self) -> &str{
        "Exception:549. DELETING ALL DATA TO ENSURE DATA INTEGRITY"
    }
}