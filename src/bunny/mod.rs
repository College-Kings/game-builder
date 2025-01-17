use std::env ;
use std::path::Path;

use bunny_cdn_wrapper::BunnyStorage;


pub async fn upload(src_path: &Path, dest_path: String) {    
    println!("Uploading {} build...", dest_path);
    println!("Source path: {}", src_path.display());

    let api_key = env::var("BUNNY_ACCESS_KEY").unwrap();

    let bunny_storage =
        BunnyStorage::new("collegekingsstorage", &api_key, "de").unwrap();

    let response = bunny_storage
        .upload(
            src_path,
            &format!(
                "__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/{dest_path}",
                
            ),
        )
        .await.unwrap();

    if !response.status().is_success() {
        println!("Failed to upload the file. Status: {:?}", response);
    }
}
