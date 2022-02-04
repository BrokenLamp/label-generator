use std::collections::HashMap;

use crate::component::SvgComponentVariant;

#[derive(Debug, Clone)]
pub struct OutputVariant<'a> {
    pub component_variants: HashMap<&'a str, &'a SvgComponentVariant>,
}

impl<'a> OutputVariant<'a> {
    pub fn add_variants(
        self,
        component_name: &'a str,
        component_variants: &'a Vec<SvgComponentVariant>,
    ) -> Vec<OutputVariant<'a>> {
        let mut variants = Vec::new();

        for variant in component_variants {
            let mut new_components = self.component_variants.clone();
            new_components.insert(component_name, variant);
            variants.push(OutputVariant {
                component_variants: new_components,
            });
        }

        variants
    }

    pub fn apply_to_svg(self, sku: &str, svg: &str) -> (String, String) {
        let mut svg = svg.to_string();
        let mut sku = sku.to_string();

        for (component_name, component_variant) in self.component_variants {
            let component_name = component_name.to_string();

            sku = sku.replace(
                format!("{{{}}}", component_name).as_str(),
                component_variant.name.as_str(),
            );
            svg = svg.replace(
                &format!("<!-- component:{} -->", component_name),
                &component_variant.data,
            );
        }

        (sku, svg)
    }
}
