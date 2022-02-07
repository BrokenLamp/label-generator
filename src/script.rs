use anyhow::Error;
use mlua::prelude::*;
use regex::Regex;

lazy_static! {
    static ref RE_SCRIPT: Regex = Regex::new(r"<script\b[^>]*>([\s\S]*?)</script>").unwrap();
    static ref RE_EXPR: Regex = Regex::new(r"\{\{(.*?)}}").unwrap();
}

pub fn run_scripts(svg: String) -> anyhow::Result<String> {
    let lua = Lua::new();
    for cap in RE_SCRIPT.captures_iter(&svg) {
        let script = &cap[1];
        lua.load(&script).exec()?;
    }
    let mut new_svg = svg.clone();
    for cap in RE_EXPR.captures_iter(&svg) {
        let expr = &cap[1];
        let value = lua.load(expr).eval::<LuaValue>()?;
        let value = match value {
            LuaNil => String::from("NULL"),
            LuaValue::Boolean(x) => x.to_string(),
            LuaValue::Integer(x) => x.to_string(),
            LuaValue::Number(x) => x.to_string(),
            LuaValue::String(x) => x.to_str().unwrap().to_string(),
            _ => Err(Error::msg("unsupported value"))?,
        };
        new_svg = new_svg.replace(&format!("{{{{{}}}}}", expr), &value.to_string());
    }
    Ok(new_svg)
}
