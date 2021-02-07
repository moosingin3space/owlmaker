use std::path::{Path, PathBuf};

static TEMPLATES: &str = "templates";
static PAGES: &str = "pages";
static ASSETS: &str = "assets";
static OUTPUT: &str = "out";

pub fn get_template_dir<P>(root: P) -> PathBuf
    where P: AsRef<Path>
{
    let mut b = PathBuf::from(root.as_ref());
    b.push(TEMPLATES);
    b
}

pub fn get_page_dir<P>(root: P) -> PathBuf
    where P: AsRef<Path>
{
    let mut b = PathBuf::from(root.as_ref());
    b.push(PAGES);
    b
}

pub fn get_asset_dir<P>(root: P) -> PathBuf
    where P: AsRef<Path>
{
    let mut b = PathBuf::from(root.as_ref());
    b.push(ASSETS);
    b
}

pub fn get_output_dir<P>(root: P) -> PathBuf
    where P: AsRef<Path>
{
    let mut b = PathBuf::from(root.as_ref());
    b.push(OUTPUT);
    b
}
