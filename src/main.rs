mod templates;
use oxysite::rebuild_site;

const CONTENT_DIR: &str = "content";
const PUBLIC_DIR: &str = "public";

fn main() -> Result<(), anyhow::Error> {
    rebuild_site(CONTENT_DIR, PUBLIC_DIR);

    // rebuild_site(&str, &str)
    // delete public dir
    // get markdown files
    // for md file
    // read file
    // parse content and pass it into read_body func in templates
    // save templated file into output dir
    Ok(())
}
