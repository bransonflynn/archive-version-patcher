pub enum Language {
    English,
    German,
    Spanish,
}
impl Language {
    pub fn patch_success(&self) -> String {
        return String::from("temp_patch_success");
    }

    pub fn patch_fail(&self) -> String {
        return String::from("temp_patch_fail");
    }

    pub fn patch_not_needed(&self) -> String {
        return String::from("temp_patch_not_needed");
    }

    pub fn error_invalid_version(&self) -> String {
        match self {
            Language::English => String::from("ERROR: Invalid version"),
            Language::German => todo!(),
            Language::Spanish => todo!(),
        }
    }
}
