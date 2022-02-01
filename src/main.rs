use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use manifest::Manifest;
use regex::Regex;

mod manifest;

fn main() -> Result<()> {
    println!(include_str!("../LICENSE"));
    println!("SVG Label Generator\n");
    println!("https://github.com/BrokenLamp/label-generator\n");

    let working_dir = get_working_dir()?;
    let manifest: Manifest = get_manifest(&working_dir)?;
    println!("🔧 Config:");
    println!("  >> Root: {}", manifest.root);
    println!("  >> SKU: {}", manifest.sku);

    let root_file_data = std::fs::read_to_string(format!("{}/{}", &working_dir, &manifest.root))?;

    let mut source_files: HashMap<String, String> = HashMap::new();

    let mut output_files: Vec<(String, String)> =
        vec![(manifest.sku.clone(), root_file_data.clone())];

    println!("📦 Components:");

    let re = Regex::new(r"<!-- component:(.*) -->").unwrap();
    for cap in re.captures_iter(&root_file_data) {
        let component_name = &cap[1];

        let path_name = format!("{}/{}", &working_dir, component_name);
        let path = std::path::Path::new(&path_name);

        if path.is_dir() {
            println!("  >> {} 📂", component_name);
            if !manifest.sku.contains(component_name) {
                println!(
                    "\x1b[93m     {} warning: '{}' not in sku\x1b[0m",
                    "^".repeat(component_name.len()),
                    component_name
                );
                println!(
                    "\n\x1b[94m     Help: add '{{{}}}' to sku in manifest.toml\x1b[0m",
                    component_name
                );
                println!("\x1b[94m     Example:\x1b[0m");
                println!(
                    "\x1b[94;1m     sku = \"{}-{{{}}}\"\x1b[0m",
                    manifest.sku, component_name
                );
                println!();
            }
            let files = std::fs::read_dir(path).unwrap();
            let mut new_output_files = Vec::new();
            for file in files.flatten() {
                let mut files_to_append = Vec::new();
                let path = file.path();
                let path_str = path.to_str().unwrap().to_string();
                let file_name = file.file_name().to_str().unwrap().to_string();
                let file_data = match source_files.get(&path_str) {
                    Some(data) => data,
                    None => {
                        let file_data = std::fs::read_to_string(path)?;
                        source_files.insert(path_str.clone(), file_data);
                        source_files.get(&path_str).unwrap()
                    }
                };
                for (sku, data) in &output_files {
                    let sku_name = file_name.split(".").collect::<Vec<_>>()[0]
                        .split("-")
                        .collect::<Vec<_>>()[0];
                    let new_sku = sku.replace(&format!("{{{}}}", &component_name), &sku_name);
                    let new_data = data.replace(
                        &format!("<!-- component:{} -->", &component_name),
                        &file_data,
                    );
                    files_to_append.push((new_sku, new_data));
                }
                new_output_files.append(&mut files_to_append);
            }
            output_files = new_output_files;
        } else {
            println!("  >> {} 📝", component_name);
            let path_name = format!("{}/{}.svg", &working_dir, component_name);
            let path = std::path::Path::new(&path_name);
            let file_data = match source_files.get(path.to_str().unwrap()) {
                Some(data) => data,
                None => {
                    let file_data = std::fs::read_to_string(path)?;
                    source_files.insert(path.to_str().unwrap().to_string(), file_data);
                    source_files.get(path.to_str().unwrap()).unwrap()
                }
            };
            for data in output_files.iter_mut() {
                data.1 = data
                    .1
                    .replace(&format!("<!-- component:{} -->", component_name), file_data);
            }
        }
    }

    println!("💾 Generated Files:");
    let _ = std::fs::create_dir(format!("{}/out", &working_dir));
    for (sku, svg) in output_files.into_iter() {
        let final_path = format!("{}/out/{}.svg", &working_dir, sku);
        println!("  >> {}", sku);
        std::fs::write(final_path, svg)?;
    }

    println!("✅ Done\n");

    Ok(())
}

fn get_working_dir() -> Result<String> {
    let mut args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        Err(anyhow!("No file specified"))?;
    }
    Ok(args.swap_remove(1))
}

fn get_manifest(folder_path: &str) -> Result<Manifest> {
    let manifest_string = std::fs::read_to_string(format!("{}/manifest.toml", folder_path))
        .context("Failed to load manifest.toml, is there one in the folder?")?;
    Ok(toml::from_str(&manifest_string).context("Failed to parse manifest.toml")?)
}
