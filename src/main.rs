use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context, Result};
use manifest::Manifest;
use regex::Regex;
use usvg::TextRendering;

use crate::{
    component::SvgComponent, ignore_condition::IgnoreGroup, output_variant::OutputVariant,
};

mod component;
mod ignore_condition;
mod manifest;
mod output_variant;

fn main() -> Result<()> {
    println!(include_str!("../LICENSE"));
    println!("SVG Label Generator\n");
    println!("https://github.com/BrokenLamp/label-generator\n");

    let working_dir = get_working_dir()?;
    let manifest: Manifest = get_manifest(&working_dir)?;
    println!("🔧 Config:");
    println!("  >> Root: {}", manifest.root);
    println!("  >> SKU: {}", manifest.sku);
    println!("  >> Ignore: {:?} conditions", manifest.ignore.len());

    let ignore_groups: Vec<IgnoreGroup> = manifest
        .ignore
        .iter()
        .flat_map(|s| IgnoreGroup::from_str(s))
        .collect::<Vec<_>>();
    println!("{:#?}", ignore_groups);

    let mut root_file_data =
        std::fs::read_to_string(format!("{}/{}", &working_dir, &manifest.root))?;

    let mut components: HashMap<String, SvgComponent> = HashMap::new();

    println!("📦 Components:");

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
                println!("  >> \x1b[92m{}\x1b[0m", component_name);
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
        println!("🔧 Component: {}", component_name);
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

    let output_files = output_variants
        .into_iter()
        .map(|x| x.apply_to_svg(&manifest.sku, &root_file_data))
        .map(|(sku, svg)| {
            let tree = usvg::Tree::from_str(&svg, &usvg_opt.to_ref()).unwrap();
            (sku, tree.to_string(&xml_opt))
        })
        .collect::<Vec<_>>();

    println!("💾 Generated Files:");
    let _ = std::fs::create_dir(format!("{}/out", &working_dir));
    for (sku, svg) in output_files.into_iter() {
        let final_path = format!("{}/out/{}.svg", &working_dir, sku);
        println!("  >> {}", sku);
        std::fs::write(final_path, &svg)?;
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
