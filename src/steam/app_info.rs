pub struct AppInfo {
    pub name: String,
    pub app_id: i32,
    pub content_path: String,
    pub additional_dlc: Vec<AppDLC>,
}

pub struct AppDLC {
    pub name: String,
    pub app_id: i32,
    pub content_path: String,
}
