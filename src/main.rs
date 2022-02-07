#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context, Result};
use manifest::Manifest;
use rayon::prelude::*;
use regex::Regex;
use usvg::TextRendering;

use crate::{
    component::SvgComponent, ignore_condition::IgnoreGroup, output_variant::OutputVariant,
};

mod component;
mod ignore_condition;
mod manifest;
mod output_variant;
mod script;

fn main() -> Result<()> {
    println!(include_str!("../LICENSE"));
    println!("SVG Label Generator\n");
    println!("https://github.com/BrokenLamp/label-generator");

    let working_dir = get_working_dir()?;
    let manifest: Manifest = get_manifest(&working_dir)?;
    println!("\n🔧 Config:");
    println!("  >> Root: {}", manifest.root);
    println!("  >> SKU: {}", manifest.sku);
    println!("  >> Ignore: {}", manifest.ignore.len());

    let ignore_groups: Vec<IgnoreGroup> = manifest
        .ignore
        .par_iter()
        .flat_map(|s| IgnoreGroup::from_str(s))
        .collect::<Vec<_>>();
    for group in &ignore_groups {
        println!("     > {}", group);
    }

    let mut root_file_data =
        std::fs::read_to_string(format!("{}/{}", &working_dir, &manifest.root))?;

    let mut components: HashMap<String, SvgComponent> = HashMap::new();

    println!("\n📦 Components:");

    let re = Regex::new(r"<!-- component:(.*) -->").unwrap();
    for cap in re.captures_iter(&root_file_data) {
        let component_name = &cap[1];

        let path_name = format!("{}/{}", &working_dir, component_name);
        let path = std::path::Path::new(&path_name);

        let component = SvgComponent::try_from(path)?;
        match &component {
            SvgComponent::Single(_data) => {
                println!("  >> {}", component_name);
            }
            SvgComponent::Exponential(variants) => {
                if !manifest.sku.contains(&format!("{{{}}}", component_name)) {
                    println!(
                        "  >> \x1b[93m\x1b[4m{}\x1b[0m 🔴 Not in SKU",
                        component_name
                    );
                } else {
                    println!("  >> \x1b[92m{}\x1b[0m", component_name);
                }
                for variant in variants {
                    println!("     > \x1b[94m{}\x1b[0m", variant.name);
                }
            }
        }
        components.insert(component_name.to_string(), component);
    }

    let mut output_variants: Vec<OutputVariant> = vec![OutputVariant {
        component_variants: HashMap::new(),
    }];

    for (component_name, component) in components.iter() {
        match component {
            SvgComponent::Exponential(component_variants) => {
                output_variants = output_variants
                    .iter()
                    .flat_map(|x| x.clone().add_variants(component_name, &component_variants))
                    .collect::<Vec<_>>();
            }
            SvgComponent::Single(x) => {
                root_file_data = root_file_data
                    .replace(format!("<!-- component:{} -->", component_name).as_str(), x);
            }
        }
    }

    let path = std::path::PathBuf::from_str(&working_dir).unwrap();

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    fontdb.load_fonts_dir(path);
    let usvg_opt = usvg::Options {
        text_rendering: TextRendering::GeometricPrecision,
        font_family: "Arial".to_string(),
        keep_named_groups: true,
        fontdb,
        ..Default::default()
    };
    let xml_opt = usvg::XmlOptions {
        id_prefix: None,
        writer_opts: xmlwriter::Options {
            use_single_quote: false,
            indent: xmlwriter::Indent::Spaces(4),
            attributes_indent: xmlwriter::Indent::Spaces(4),
        },
    };

    println!("\n📝 Generated Files:");

    let output_files = output_variants
        .into_par_iter()
        .map(|x| (x.get_sku(&manifest.sku), x))
        .filter(|(sku, x)| {
            let should_ignore = !x.should_ignore(&ignore_groups);
            if should_ignore {
                println!("  >> \x1b[92m{}\x1b[0m", sku);
            } else {
                println!("  >> \x1b[94m{} : ignored\x1b[0m", sku);
            }
            should_ignore
        })
        .map(|(sku, x)| (sku, x.apply_to_svg(&root_file_data)))
        .map(|(sku, svg)| -> (String, anyhow::Result<String>) { (sku, script::run_scripts(svg)) })
        .flat_map(|(sku, svg)| {
            let svg = match svg {
                Ok(s) => s,
                Err(e) => {
                    println!("{}", e);
                    return None;
                }
            };
            Some((sku, svg))
        })
        .map(|(sku, svg)| {
            let tree = usvg::Tree::from_str(&svg, &usvg_opt.to_ref()).unwrap();
            (sku, tree.to_string(&xml_opt))
        })
        .collect::<Vec<_>>();

    println!("\n💾 Saving Labels");
    println!("  >> {} files", output_files.len());
    let _ = std::fs::create_dir(format!("{}/out", &working_dir));
    output_files.into_par_iter().for_each(|(sku, svg)| {
        let final_path = format!("{}/out/{}.svg", &working_dir, sku);
        std::fs::write(final_path, &svg).unwrap();
    });

    println!("\n✅ Done\n");

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
