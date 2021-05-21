use js::*;
use lit_html::*;

static mut COUNTER: u32 = 0;

/**
 * Build counter component.
 */
fn counter() -> Template {
    let data = TemplateData::new();
    data.set("count", unsafe { COUNTER });
    data.set("increment", || {
        unsafe { COUNTER += 1 };
        rerender();
    });
    html!(
        r#"
        <p>Counter value: ${_.count}</p>
        <button class="mt-auto bg-blue-300 hover:bg-opacity-75 transition-colors duration-200 rounded-xl font-semibold py-2 px-4 inline-flex" @click="${_.increment}">Increment +</button>"#,
        &data
    )
}

/**
 * Mount the screen
 */
fn app() -> Template {
    let data = TemplateData::new();
    data.set("counter", &counter());
    html!(
        r#"<div class="container mx-auto">
            <p class="text-2xl mb-2">Counter with Lit HTML Rust!</p>
            <div>${_.counter}</div>
        </div>
        "#,
        &data
    )
}

fn rerender() {
    render(&app(), DOM_BODY);
}

#[no_mangle]
pub fn main() {
    rerender();
}
