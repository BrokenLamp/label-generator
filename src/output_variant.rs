use std::collections::HashMap;

use crate::{component::SvgComponentVariant, ignore_condition::IgnoreGroup};

#[derive(Debug, Clone)]
pub struct OutputVariant<'a> {
    pub component_variants: HashMap<&'a str, &'a SvgComponentVariant>,
}

impl<'a> OutputVariant<'a> {
    pub fn add_variants(
        self,
        component_name: &'a str,
        component_variants: &'a [SvgComponentVariant],
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

    pub fn should_ignore(&self, ignore_groups: &[IgnoreGroup]) -> bool {
        let v = self
            .component_variants
            .iter()
            .map(|e| (*e.0, e.1.name.as_ref()))
            .collect::<Vec<_>>();
        ignore_groups.iter().any(|group| group.matches(&v[..]))
    }

    pub fn get_sku(&self, template: &str) -> String {
        let mut sku = template.to_string();
        for (component_name, variant) in &self.component_variants {
            sku = sku.replace(&format!("{{{}}}", component_name), variant.name.as_ref());
        }
        sku
    }

    pub fn apply_to_svg(self, svg: &str) -> String {
        let mut svg = svg.to_string();

        for (component_name, component_variant) in self.component_variants {
            let component_name = component_name.to_string();

            svg = svg.replace(
                &format!("<!-- component:{} -->", component_name),
                &component_variant.data,
            );
        }

        svg
    }
}
