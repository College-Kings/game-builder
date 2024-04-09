pub struct AppInfo {
    pub name: String,
    pub app_id: i32,
    pub content_path: String,
    pub additional_dlc: Vec<AppInfo>,
}

impl AppInfo {
    pub fn new(name: impl Into<String>, app_id: i32, content_path: impl Into<String>) -> Self {
        AppInfo {
            name: name.into(),
            app_id,
            content_path: content_path.into(),
            additional_dlc: Vec::new(),
        }
    }
}
