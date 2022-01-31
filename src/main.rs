use manifest::Manifest;
use std::path;

mod manifest;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let manifest_string = std::fs::read_to_string(format!("{}/manifest.toml", &args[1])).unwrap();
    let manifest: Manifest = toml::from_str(&manifest_string).unwrap();
    let root = std::fs::read_to_string(format!("{}/{}", &args[1], &manifest.root)).unwrap();
    let mut file_data = Vec::new();
    for folder in manifest.folders {
        let mut inner_file_data = Vec::new();
        let path = format!("{}/{}", &args[1], folder);
        let x = std::fs::read_dir(path).unwrap();
        for i in x.flatten() {
            let data = std::fs::read_to_string(i.path()).unwrap();
            inner_file_data.push((
                i.path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split("-")
                    .collect::<Vec<_>>()[0]
                    .to_owned(),
                data,
            ));
        }
        file_data.push(inner_file_data);
    }
    fn get(a: &[Vec<(String, String)>]) -> Vec<(String, String)> {
        if a.len() == 1 {
            return a[0].clone();
        }
        let next = get(&a[1..]);
        let mut result = Vec::new();
        for i in a[0].iter() {
            for j in next.iter() {
                result.push((format!("{}-{}", i.0, j.0), format!("{}{}", i.1, j.1)));
            }
        }
        result
    }
    let x = get(&file_data);
    let _ = std::fs::create_dir(format!("{}/out", &args[1]));
    for (sku, svg) in x.iter() {
        let final_svg = root.replace("<!--body-->", svg);
        let final_name = format!("{}-{}.svg", &manifest.prefix, sku);
        let final_path = format!("{}/out/{}", &args[1], final_name);
        std::fs::write(final_path, final_svg).unwrap();
    }
}
