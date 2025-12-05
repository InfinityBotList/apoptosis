use mluau::prelude::*;

pub fn json_stringify(lua: &Lua, value: LuaValue) -> LuaResult<String> {
    let json_value: serde_json::Value = lua.from_value(value)?;
    serde_json::to_string(&json_value).map_err(LuaError::external)
}

pub fn json_parse(lua: &Lua, json_str: String) -> LuaResult<LuaValue> {
    let json_value: serde_json::Value =
        serde_json::from_str(&json_str).map_err(LuaError::external)?;
    lua.to_value(&json_value)
}

pub fn json_stringify_pretty(lua: &Lua, value: LuaValue) -> LuaResult<String> {
    let json_value: serde_json::Value = lua.from_value(value)?;
    serde_json::to_string_pretty(&json_value).map_err(LuaError::external)
}

pub fn json_tab(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;
    table.set("stringify", lua.create_function(json_stringify)?)?;
    table.set("parse", lua.create_function(json_parse)?)?;
    table.set("stringifypretty", lua.create_function(json_stringify_pretty)?)?;
    table.set_readonly(true);
    Ok(table)
}