use std::{
    env,
    fs,
    path::{Path, PathBuf},
    };
use structopt::{
    StructOpt,
    clap::AppSettings,
    };
use anyhow::{bail, Error};
use tera::{Context, Tera};
use walkdir::{DirEntry, WalkDir};

mod dirs;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Cli {
    /// Creates a new static site in the current directory.
    Init,

    /// Builds the static site.
    Build,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn walker_filter(entry: &DirEntry) -> bool {
    !is_hidden(entry)
}

fn compute_output_file(root: impl AsRef<Path>, relative: impl AsRef<Path>) -> PathBuf {
    let mut b = dirs::get_output_dir(root);
    b.push(relative.as_ref());
    b
}

fn make_parent_if_necessary(path: impl AsRef<Path>) -> Result<(), Error> {
    let parent = match path.as_ref().parent() {
        Some(p) => p,
        None => bail!("shouldn't be in root dir")
    };
    fs::create_dir_all(parent)?;
    Ok(())
}

fn add_templates_from_dir(tera: &mut Tera, prefix: impl AsRef<Path>, template_root: impl AsRef<Path>) -> Result<(), Error> {
    let walker = WalkDir::new(template_root.as_ref())
        .into_iter();
    for entry in walker.filter_entry(walker_filter) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let rel_path = entry.path().strip_prefix(prefix.as_ref())?;
            tera.add_template_file(entry.path(), Some(rel_path.to_str().unwrap()))?;
        }
    }
    Ok(())
}

fn init() -> Result<(), Error> {
    let cwd = env::current_dir()?;
    // make directories
    fs::create_dir(dirs::get_template_dir(&cwd))?;
    fs::create_dir(dirs::get_page_dir(&cwd))?;
    fs::create_dir(dirs::get_asset_dir(&cwd))?;

    let _f = fs::File::create({
        let mut b = PathBuf::from(&cwd);
        b.push("context.toml");
        b
    });

    Ok(())
}

fn build() -> Result<(), Error> {
    let cwd = env::current_dir()?;
    let mut tera = Tera::default();
    // walk through all the unrendered templates
    print!("Collecting templates...");
    add_templates_from_dir(&mut tera, &cwd, dirs::get_template_dir(&cwd))?;
    println!("Done!");

    // now add each page as a template
    print!("Collecting pages...");
    add_templates_from_dir(&mut tera, &cwd, dirs::get_page_dir(&cwd))?;
    println!("Done!");

    // load global context in (from toml file)
    let context_path = {
        let mut b = PathBuf::from(&cwd);
        b.push("context.toml");
        b
    };
    let ctx = if context_path.exists() {
        let data = fs::read_to_string(context_path)?;
        Context::from_serialize(toml::from_str(&data)?)?
    } else {
        Context::new()
    };

    // clean the output directory
    let out_dir = dirs::get_output_dir(&cwd);
    if out_dir.exists() {
        let entries = fs::read_dir(out_dir)?;
        for entry in entries {
            let entry = entry?;
            if entry.path().is_dir() {
                fs::remove_dir_all(entry.path())?;
            } else {
                fs::remove_file(entry.path())?;
            }
        }
    } else {
        fs::create_dir(out_dir)?;
    }

    // now render
    print!("Rendering pages...");
    let page_walker = WalkDir::new(dirs::get_page_dir(&cwd)).into_iter();
    for entry in page_walker.filter_entry(walker_filter) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let rel_path = entry.path().strip_prefix(dirs::get_page_dir(&cwd))?;
            let entry_name = entry.path().strip_prefix(&cwd)?.to_str().unwrap();
            let output_file = compute_output_file(&cwd, rel_path);
            let rendered_output = tera.render(&entry_name, &ctx)?;
            make_parent_if_necessary(&output_file)?;
            fs::write(output_file, rendered_output)?;
        }
    }
    println!("Done!");

    // copy assets
    print!("Copying assets...");
    let asset_walker = WalkDir::new(dirs::get_asset_dir(&cwd)).into_iter();
    for entry in asset_walker.filter_entry(walker_filter) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let new_path = {
                let rel_path = entry.path().strip_prefix(dirs::get_asset_dir(&cwd))?;
                let mut b = PathBuf::from(dirs::get_output_dir(&cwd));
                b.push(&rel_path);
                b
            };
            make_parent_if_necessary(&new_path)?;
            fs::copy(entry.path(), &new_path)?;
        }
    }
    println!("Done!");
    Ok(())
}

fn main() {
    let cli = Cli::from_args();
    let res = match cli {
        Cli::Init => init(),
        Cli::Build => build(),
    };
    if let Err(e) = res {
        eprintln!("error: {}", e);
    }
}
