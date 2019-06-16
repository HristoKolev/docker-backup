use crate::global::prelude::*;

pub fn config_command() -> Result {

    let app_config = app_config();

    let json = serde_json::to_string_pretty(app_config)?;

    println!("{}", json);

    Ok(())
}
