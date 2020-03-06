extern crate check_if_email_exists;
extern crate neon;
extern crate tokio;

use check_if_email_exists::email_exists;
use neon::prelude::*;
use serde_json;
use tokio::runtime::Runtime;

fn check_email_exists(mut cx: FunctionContext) -> JsResult<JsString> {
    let to_email = cx.argument::<JsString>(0)?.value();

    let from_email = match cx.argument_opt(1) {
        Some(arg) => arg.downcast::<JsString>().or_throw(&mut cx)?.value(),
        // Default to 12 if no value is given
        None => "user@example.org".to_string(),
    };

    println!("to email :{} ,from email :{}", to_email,from_email);


    // let result = email_exists("check.this.email@qqdjb.com", "user@example.org").await;
    // let result = Runtime::new().unwrap().block_on(email_exists(
    //     "check.this.email@qqdjb.com",
    //     "user@example.org",
    // ));
    let result = Runtime::new()
        .unwrap()
        .block_on(email_exists(&to_email, &from_email));

    let ret = match serde_json::to_string_pretty(&result) {
        Ok(output) => output,
        Err(err) => err.to_string(),
    };
    Ok(cx.string(ret))
}
register_module!(mut cx, {
    cx.export_function("check_email_exists", check_email_exists)
});
