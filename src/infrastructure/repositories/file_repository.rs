use std::env::home_dir;

pub struct FileRepository;

impl FileRepository {
    pub fn get_base_path() -> &'static str {
        let dir = match home_dir() {
            Some(path) => path.to_str().unwrap().to_string(),
            None => "/home".into()
        };

        format!("{dir}/.r8s").leak()
    }

    pub fn get_fjall_path() -> &'static str {
        format!(
            "{}/fjall",
            Self::get_base_path(),
        ).leak()
    }
}