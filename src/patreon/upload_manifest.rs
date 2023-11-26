// use std::fs;
// use azure_storage::prelude::*;
// use azure_storage_blobs::prelude::*;
// use std::path::Path;
// use crate::VERSION;
//
// pub async fn upload_manifest(manifest_path: &Path) {
//     let account = String::from("collegekingsstgaccountus");
//     let access_key = String::from("nBlJ34Xe4wlRgpL/T/T4M4FOXDNHz2aDmX6uT7wfU8IsQI6lZRau4yArfm8JeaJ/zvZKEPffKhqd+AStu1aL6Q==");
//     let blob_name = format!("{}/{}", VERSION.replace('.', "-"), manifest_path.file_name().unwrap().to_str().unwrap());
//
//     let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
//
//     let blob_client = ClientBuilder::new(account, storage_credentials).blob_client("data", blob_name);
//
//     let data = fs::read(manifest_path).unwrap();
//
//     let res = blob_client.put_block_blob(data).content_type("text/plain").await.unwrap();
//     println!("put_blob {res:?}")
// }
