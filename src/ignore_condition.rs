use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub struct IgnoreGroup {
    pub conditions: Vec<IgnoreCondition>,
}

impl IgnoreGroup {
    pub fn matches(&self, variants: &[(&str, &str)]) -> bool {
        self.conditions
            .iter()
            .all(|condition| condition.matches(variants))
    }
}

impl FromStr for IgnoreGroup {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let conditions = s
            .split(",")
            .flat_map(|m| IgnoreCondition::from_str(m))
            .collect::<Vec<_>>();

        Ok(IgnoreGroup { conditions })
    }
}

impl Display for IgnoreGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, condition) in self.conditions.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", condition)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct IgnoreCondition {
    pub component: String,
    pub variant: String,
}

impl IgnoreCondition {
    pub fn matches(&self, variants: &[(&str, &str)]) -> bool {
        variants
            .iter()
            .any(|v| v.0 == self.component && v.1 == self.variant)
    }
}

impl FromStr for IgnoreCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        let component = split.next().ok_or(())?.to_string();
        let variant = split.next().ok_or(())?.to_string();

        Ok(IgnoreCondition { component, variant })
    }
}

impl Display for IgnoreCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\x1b[92m{}\x1b[0m:\x1b[94m{}\x1b[0m",
            self.component, self.variant
        )
    }
}
