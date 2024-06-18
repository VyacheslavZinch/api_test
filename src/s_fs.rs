pub mod server_fs {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct FileInfo {
        pub name: String,
        pub file_type: String,
    }
    pub struct FileSystem;
    impl FileSystem {
        pub async fn recursion_get_all_files(dir: &str) {
            use std::fs;

            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let _data = entry.metadata().unwrap();
                        let fdir_path = dir.to_string() + "/" + entry.file_name().to_str().unwrap();

                        if _data.is_dir() == true {
                            FileSystem::get_info_from_dir(&fdir_path).await;
                        } else {
                            println!("{:?}", entry.file_name())
                        }
                    }
                }
            }
        }

        pub async fn get_info_from_dir(dir: &str) -> Vec<FileInfo> {
            use std::fs;

            let mut result: Vec<FileInfo> = Vec::new();
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let _data = entry.metadata().unwrap();

                        if _data.is_dir() == true {
                            let element = FileInfo {
                                name: (entry.file_name().to_str().unwrap()).to_string(),
                                file_type: "dir".to_string(),
                            };
                            result.push(element)
                        } else {
                            let element = FileInfo {
                                name: (entry.file_name().to_str().unwrap()).to_string(),
                                file_type: "file".to_string(),
                            };
                            result.push(element)
                        }
                    }
                }
            }
            result
        }
    }
}
