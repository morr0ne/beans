use beans::Client;
use futures::prelude::*;
use napi::{CallContext, Error, JsObject, JsString, JsUndefined, Result, Status};

#[js_function(0)]
pub fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    let mut this: JsObject = ctx.this_unchecked();
    let client = Client::new();

    ctx.env.wrap(&mut this, client)?;
    ctx.env.get_undefined()
}

#[js_function(0)]
pub fn get(ctx: CallContext) -> Result<JsObject> {
    let this: JsObject = ctx.this_unchecked();
    let client = ctx.env.unwrap::<Client>(&this)?;

    let res = ctx.env.execute_tokio_future(
        client.get().map(|res| {
            res.map_err(|e| Error::new(Status::GenericFailure, "good question lol".to_string()))
        }),
        |&mut env, res| env.to_js_value(&res),
    );

    res
}

// #[js_function(1)]
// pub fn test_execute_tokio_readfile(ctx: CallContext) -> Result<JsObject> {
//     let js_filepath = ctx.get::<JsString>(0)?;
//     let path_str = js_filepath.into_utf8()?.into_owned()?;
//     ctx.env.execute_tokio_future(
//         tokio::fs::read(path_str).map(|v| {
//             v.map_err(|e| {
//                 Error::new(
//                     Status::GenericFailure,
//                     format!("failed to read file, {}", e),
//                 )
//             })
//         }),
//         |&mut env, data| env.create_buffer_with_data(data).map(|v| v.into_raw()),
//     )
// }

// #[js_function(0)]
// pub fn uuid(ctx: CallContext<'static>) -> Result<JsObject> {
//     let this: JsObject = ctx.this_unchecked();
//     let client: &Client = ctx.env.unwrap(&this)?;
//     ctx.env.execute_tokio_future(
//         async {
//             Ok(client.uuid().await.unwrap()) // TODO: handle errors
//         },
//         |&mut env, res| env.to_js_value(&res),
//     )
// }
