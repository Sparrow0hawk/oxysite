use anyhow;
use log::{error, info};
use pulldown_cmark;
use std::fs;
use std::path::Path;
use std::process;
use walkdir::WalkDir;

mod templates;

pub struct Site {
    pub content_dir: String,
    pub build_dir: String,
}

impl Site {
    pub fn new(content_dir: String, build_dir: String) -> Result<Site, &'static str> {
        Ok(Site {
            content_dir,
            build_dir,
        })
    }
}

pub fn rebuild_site(site: Site) -> Result<Vec<String>, anyhow::Error> {
    let config: Site = site;

    let _ = build_dir_control(&config.build_dir);
    let mut content_files: Vec<String> = WalkDir::new(&config.content_dir)
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|file| file.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .collect();

    let mut html_files = Vec::with_capacity(content_files.len());

    for file in &content_files {
        let file_contents = fs::read_to_string(file)?;

        let html = build_md(&file_contents).unwrap_or_else(|e| {
            error!("An error occured building html from markdown: {}", e);
            process::exit(1)
        });

        let html_name = write_html(&file, html, &config).unwrap_or_else(|e| {
            error!("An error occured writing html files to disk: {}", e);
            process::exit(1);
        });

        html_files.push(html_name);
    }

    println!("{:?}", html_files);

    Ok(html_files)
}

fn build_dir_control(build_dir: &str) -> () {
    env_logger::init();
    if Path::exists(Path::new(build_dir)) {
        info!("Build directory exists at {}", build_dir);
    } else {
        fs::create_dir(build_dir).unwrap_or_else(|e| {
            error!("An error occured creating this directory: {}", e);
            process::exit(1)
        });
        info!("Creating build directory at {}", build_dir);
    }
}

fn build_md(file: &str) -> Result<String, anyhow::Error> {
    // build html files from markdown files
    // uses pulldown_cmark crate to do conversion
    // creates a
    let mut html = templates::HEADER.to_owned();
    let parser = pulldown_cmark::Parser::new_ext(&file, pulldown_cmark::Options::all());
    let mut body = String::new();

    pulldown_cmark::html::push_html(&mut body, parser);

    html.push_str(templates::render_body(&body).as_str());
    html.push_str(templates::FOOTER);

    Ok(html)
}

fn write_html(file: &str, html: String, site: &Site) -> Result<String, std::io::Error> {
    let html_file = file
        .replace(&site.content_dir, &site.build_dir)
        .replace(".md", ".html");
    let folder = Path::new(&html_file).parent().unwrap();
    let _ = fs::create_dir_all(folder);
    fs::write(&html_file, html)?;

    Ok(html_file)
}

#[cfg(test)]

mod test {
    use super::*;
    use assert_str::assert_str_trim_eq;

    #[test]
    fn test_write_html() {
        let file = "test_content/home.md";
        let test_site = Site {
            content_dir: String::from("test_content"),
            build_dir: String::from("public"),
        };

        let test_html = r#"<!DOCTYPE html>
        <html lang="en">
        
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width", initial-scale=1">
            </head>
            <h1>Hello world!</h1>
            </body>
            </html>
        "#
        .to_string();

        let _ = fs::remove_dir_all("public");
        let html_file = write_html(file, test_html, &test_site).unwrap();

        assert_eq!("public/home.html", html_file)
    }

    #[test]
    fn test_build_md() {
        let md = "# Blog page\n- item 1\n- item 2\n- item 3";

        let output = build_md(&md).unwrap();

        assert_str_trim_eq!(
            // assert returned string matches string
            output,
            r#"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <meta name="viewport" content="width=device-width", initial-scale=1">
                </head>
                <body>
                <nav>
                    <a href="/">Home</a>
                </nav>
                <br />
                <h1>Blog page</h1>
                <ul>
                <li>item 1</li>
                <li>item 2</li>
                <li>item 3</li>
                </ul>
                </body>
                </html>
            "#
            .to_string()
        )
    }

    #[test]
    fn rebuild_basic_test() {
        let test_site = Site {
            content_dir: String::from("test_content"),
            build_dir: String::from("public"),
        };

        let _ = fs::remove_dir_all("public");

        assert_eq!(
            // this should check fiel system for files
            rebuild_site(test_site).unwrap(),
            vec!["public/blog.html", "public/home.html"]
        )
    }
}
