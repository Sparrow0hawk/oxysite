mod templates;
use oxysite::{rebuild_site, Site};

fn main() -> Result<(), anyhow::Error> {
    let content: String = String::from("test_content");
    let public: String = String::from("public");

    let config = Site {
        content_dir: content,
        build_dir: public,
    };

    rebuild_site(config);

    // rebuild_site(&str, &str)
    // delete public dir
    // get markdown files
    // for md file
    // read file
    // parse content and pass it into read_body func in templates
    // save templated file into output dir
    Ok(())
}
