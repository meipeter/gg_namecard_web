use cfg_if::cfg_if;

use std::panic;
use web_sys::window;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate wasm_bindgen;
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern {
            #[wasm_bindgen(js_namespace = console)]
            fn error(msg: String);

            type Error;

            #[wasm_bindgen(constructor)]
            fn new() -> Error;

            #[wasm_bindgen(structural, method, getter)]
            fn stack(error: &Error) -> String;
        }

        fn hook_impl(info: &panic::PanicInfo) {
            let mut msg = info.to_string();


            // Add the error stack to our message.
            //
            // This ensures that even if the `console` implementation doesn't
            // include stacks for `console.error`, the stack is still available
            // for the user. Additionally, Firefox's console tries to clean up
            // stack traces, and ruins Rust symbols in the process
            // (https://bugzilla.mozilla.org/show_bug.cgi?id=1519569) but since
            // it only touches the logged message's associated stack, and not
            // the message's contents, by including the stack in the message
            // contents we make sure it is available to the user.
            msg.push_str("\n\nStack:\n\n");
            let e = Error::new();
            let stack = e.stack();
            msg.push_str(&stack);

            // Safari's devtools, on the other hand, _do_ mess with logged
            // messages' contents, so we attempt to break their heuristics for
            // doing that by appending some whitespace.
            // https://github.com/rustwasm/console_error_panic_hook/issues/7
            msg.push_str("\n\n");

            // Finally, log the panic with `console.error`!
            let window = window().ok_or_else(|| JsValue::from_str("No window")).unwrap();
                // 获取 document 对象
                let document = window.document().ok_or_else(|| JsValue::from_str("No document")).unwrap();

                // 创建一个新的 div 元素
                let div = document.create_element("div").unwrap();


            let html = format!(
                r#"
                <div role="alert" class="alert alert-error">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                  <span>{}</span>
                </div>
                "#,
                msg.replace("\n", "<br>")
            );
            // 设置 div 的内容
            div.set_inner_html(html.as_str());

            // 将 div 添加到 body 中
            while let Some(child) = document.body().unwrap().first_child() {
                    document.body().unwrap().remove_child(&child).expect("Failed to remove child");
                };
            document.body().unwrap().append_child(&div).unwrap();
            error(msg);
        }
    } else {
        use std::io::{self, Write};

        fn hook_impl(info: &panic::PanicInfo) {
            let _ = writeln!(io::stderr(), "{}", info);
        }
    }
}

/// A panic hook for use with
/// [`std::panic::set_hook`](https://doc.rust-lang.org/nightly/std/panic/fn.set_hook.html)
/// that logs panics into
/// [`console.error`](https://developer.mozilla.org/en-US/docs/Web/API/Console/error).
///
/// On non-wasm targets, prints the panic to `stderr`.
pub fn hook(info: &panic::PanicInfo) {
    hook_impl(info);
}

/// Set the `console.error` panic hook the first time this is called. Subsequent
/// invocations do nothing.
#[inline]
pub fn set_once() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook));
    });
}
