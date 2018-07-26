mod midi;

extern crate rlua;

use rlua::prelude::*;
use std::error::Error;

/// Main entry point to library.
pub fn run() -> Result<(), Box<Error>> {
    //FIXME: Is there a way to get this from cargo.toml?
    println!("OSC/MIDI Bridge ver. 0.1.0");
    println!("Initializing Lua... ");
    let lua = Lua::new();

    match lua.eval::<()>(
        r#"
        print("Lua initialized.")
        "#,
        None,
    )?;

    Ok(())
}
