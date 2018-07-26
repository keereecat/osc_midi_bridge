mod midi;

extern crate rlua;

use rlua::prelude::*;

/// Main entry point to library.
pub fn run() -> Result<(), rlua::Error> {
    //FIXME: Is there a way to get this from cargo.toml?
    println!("OSC/MIDI Bridge ver. 0.1.0");
    println!("Initializing Lua... ");
    let lua = Lua::new();

    lua.eval::<()>(
        r#"
        print("Lua initialized.")
        "#,
        None,
    )?;

    Ok(())
}
