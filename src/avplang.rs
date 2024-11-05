pub enum Lang {
    English,
    German,
    Spanish,
}
impl Lang {}

pub struct Language {
    pub lang: Lang,
}
impl Language {
    pub fn patch_success(&self) -> String {
        return String::from("temp_patch_success");
    }

    pub fn patch_fail(&self) -> String {
        return String::from("temp_patch_fail");
    }

    pub fn error_invalid_version(&self) -> String {
        match &self.lang {
            Lang::English => String::from("ERROR: Invalid version"),
            Lang::German => todo!(),
            Lang::Spanish => todo!(),
        }
    }
}
