use std::path::Path;

#[derive(Debug, Clone)]
pub enum SvgComponent {
    Single(String),
    Exponential(Vec<SvgComponentVariant>),
}

impl TryFrom<&Path> for SvgComponent {
    type Error = std::io::Error;
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Ok(if path.is_dir() {
            let variants = std::fs::read_dir(path)?
                .flatten()
                .flat_map(|file| -> Result<_, std::io::Error> {
                    let path = file.path();
                    let file_name = file.file_name().to_str().unwrap().to_string();
                    let file_data = std::fs::read_to_string(path)?;
                    let sku_name = file_name.split(".").collect::<Vec<_>>()[0]
                        .split("-")
                        .collect::<Vec<_>>()[0]
                        .to_owned();
                    Ok(SvgComponentVariant {
                        name: sku_name,
                        data: file_data,
                    })
                })
                .collect::<Vec<_>>();
            SvgComponent::Exponential(variants)
        } else {
            let path_with_svg = format!("{}.svg", path.to_str().unwrap().to_string());
            let file_data = std::fs::read_to_string(path_with_svg)?;
            SvgComponent::Single(file_data.to_string())
        })
    }
}

#[derive(Debug, Clone)]
pub struct SvgComponentVariant {
    pub name: String,
    pub data: String,
}
