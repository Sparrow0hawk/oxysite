use anyhow;
use pulldown_cmark;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

mod templates;

pub fn rebuild_site(content_dir: &str, build_dir: &str) -> Result<Vec<String>, anyhow::Error> {
    let _ = build_dir_control(build_dir);
    let mut content_files: Vec<String> = WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|file| file.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .collect();

    let mut html_files = Vec::with_capacity(content_files.len());

    for file in &content_files {
        let mut html = templates::HEADER.to_owned();
        let markdown_file = fs::read_to_string(&file)?;
        let parser =
            pulldown_cmark::Parser::new_ext(&markdown_file, pulldown_cmark::Options::all());
        let mut body = String::new();

        pulldown_cmark::html::push_html(&mut body, parser);

        html.push_str(templates::render_body(&body).as_str());
        html.push_str(templates::FOOTER);

        let html_file = file.replace(content_dir, build_dir).replace(".md", ".html");

        html_files.push(html_file);
    }

    println!("{:?}", html_files);

    Ok(html_files)
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
