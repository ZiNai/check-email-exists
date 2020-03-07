extern crate check_if_email_exists;
extern crate neon;
extern crate neon_serde;
extern crate tokio;

use check_if_email_exists::email_exists;
use neon::prelude::*;
use serde_json;
use tokio::runtime::Runtime;

// core fn , async check email return String
fn check_email_exists(to_email: &str, from_email: &str) -> String {
    let result = Runtime::new()
        .unwrap()
        .block_on(email_exists(&to_email, &from_email));

    let ret = match serde_json::to_string_pretty(&result) {
        Ok(output) => output,
        Err(err) => err.to_string(),
    };
    ret
}

// js sync  function ,return js Json Obj
fn check_email_exists_sync(mut cx: FunctionContext) -> JsResult<JsValue> {
    let to_email = cx.argument::<JsString>(0)?.value();

    let from_email = match cx.argument_opt(1) {
        Some(arg) => {
            // Throw if the argument exist and it cannot be downcasted
            // to a number
            let from = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
            from
        }
        None => String::from("user@example.org"),
    };

    let result = Runtime::new()
        .unwrap()
        .block_on(email_exists(&to_email, &from_email));

    let js_value = neon_serde::to_value(&mut cx, &result)?;
    Ok(js_value)
}

// async
struct CheckEmailTask {
    to_email: String,
    from_email: String,
}

impl Task for CheckEmailTask {
    type Output = String;
    type Error = String;
    type JsEvent = JsString;

    fn perform(&self) -> Result<String, String> {
        Ok(check_email_exists(&self.to_email, &self.from_email))
    }

    fn complete(self, mut cx: TaskContext, result: Result<String, String>) -> JsResult<JsString> {
        Ok(cx.string(result.unwrap().to_string()))
    }
}

// js async  function ,return js Json String , can be parsed to Json Obj
fn check_email_exists_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let to_email = cx.argument::<JsString>(0)?.value();

    let cb;
    let mut from_email = String::from("user@example.org");

    //  according the args length impl option js function arguments 
    let args_length = cx.len();
    if args_length == 3 {
        from_email = cx.argument::<JsString>(1)?.value();
        cb = cx.argument::<JsFunction>(2)?;
    } else {
        cb = cx.argument::<JsFunction>(1)?;
    }

    let task = CheckEmailTask {
        to_email,
        from_email,
    };

    task.schedule(cb);

    Ok(cx.undefined())
}

register_module!(mut m, {
    m.export_function("checkEmailExistsSync", check_email_exists_sync)?;
    m.export_function("checkEmailExistsAsync", check_email_exists_async)?;
    Ok(())
});
