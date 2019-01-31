// native_counter/native/src/lib.rs

#[macro_use]
extern crate neon;

use neon::prelude::*;

fn count_words(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let text = cx.argument::<JsString>(0)?.value();
    let word = cx.argument::<JsString>(1)?.value();
    Ok(cx.number(text.split(" ").filter(|s| s == &word).count() as f64))
}

register_module!(mut m, { 
    m.export_function("count_words", count_words)?;
    Ok(())
});
