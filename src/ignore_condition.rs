use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct IgnoreGroup {
    pub conditions: Vec<IgnoreCondition>,
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

#[derive(Debug, Clone)]
pub struct IgnoreCondition {
    pub component: String,
    pub variant: String,
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
