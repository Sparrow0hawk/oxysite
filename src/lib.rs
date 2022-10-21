use log::info;
use std::fs::create_dir;
use std::path::Path;
use walkdir::WalkDir;

pub fn rebuild_site(content_dir: &str, build_dir: &str) -> Vec<String> {
    let _ = build_dir_control(build_dir);
    let mut content_files: Vec<String> = WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|file| file.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .collect();

    println!("{:?}", content_files);

    content_files
}

fn build_dir_control(build_dir: &str) -> () {
    if Path::exists(Path::new(build_dir)) {
        create_dir(build_dir);
        info!("Creating build directory at {}", build_dir);
    } else {
        info!("Build directory exists at {}", build_dir);
    }
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn rebuild_basic_test() {
        assert_eq!(
            rebuild_site("test_content", "test_build"),
            vec!["test_content/blog.md", "test_content/home.md"]
        )
    }
}
