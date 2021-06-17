#[macro_use]
extern crate napi_derive;

use napi::{Env, JsObject, Property, Result};

mod client;

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
    let client = env.define_class(
        "Client",
        client::constructor,
        &[
            Property::new(&env, "get")?.with_method(client::get),
            Property::new(&env, "uuid")?.with_method(client::uuid),
        ],
    )?;
    exports.set_named_property("Client", client)?;
    Ok(())
}
