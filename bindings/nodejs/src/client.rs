use beans::Client;
use napi::{CallContext, JsObject, JsUndefined, JsUnknown, Result};

#[js_function(0)]
pub fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    let mut this: JsObject = ctx.this_unchecked();
    let client = Client::new();

    ctx.env.wrap(&mut this, client)?;
    ctx.env.get_undefined()
}

#[js_function(0)]
pub fn get(ctx: CallContext) -> Result<JsUnknown> {
    let this: JsObject = ctx.this_unchecked();
    let client: &Client = ctx.env.unwrap(&this)?;

    let res = client.get().unwrap();
    ctx.env.to_js_value(&res)
}

#[js_function(0)]
pub fn uuid(ctx: CallContext) -> Result<JsUnknown> {
    let this: JsObject = ctx.this_unchecked();
    let client: &Client = ctx.env.unwrap(&this)?;

    let res = client.uuid().unwrap();
    ctx.env.to_js_value(&res)
}
