pub enum Language {
    English,
    German,
}
impl Language {
    pub fn to_string(&self) -> String {
        match self {
            Language::English => return String::from("English"),
            Language::German => return String::from("German"),
        }
    }

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
    Default,
}
impl Message {
    pub fn warning_invalid_language(&self, lang: Language) -> String {
        // doesnt need a match statement since this is error handling
        return String::from("WARNING! Invalid language set in config/avp_config.toml");
    }
}
