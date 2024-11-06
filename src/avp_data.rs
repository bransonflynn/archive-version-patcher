pub enum Language {
    English,
    German,
}
impl Language {
    pub fn patch_success(&self) -> String {
        match self {
            Language::English => return String::from("temp_patch_success"),
            Language::German => todo!(),
        }
    }

    pub fn patch_fail(&self) -> String {
        match self {
            Language::English => return String::from("temp_patch_fail"),
            Language::German => todo!(),
        }
    }

    pub fn patch_not_needed(&self) -> String {
        match self {
            Language::English => return String::from("temp_patch_not_needed"),
            Language::German => todo!(),
        }
    }

    pub fn error_invalid_version(&self) -> String {
        match self {
            Language::English => String::from("ERROR: Invalid version!"),
            Language::German => todo!(),
        }
    }
}

pub enum Message {
    Error,
    Notif,
}

pub enum Error {
    PatchFail,
    PatchNotNeeded,
    InvalidVersion,
}

pub enum Notif {
    PatchSuccess,
    ProcessStarted,
    ProcessFinished,
}
