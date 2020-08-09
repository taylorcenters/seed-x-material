#![allow(clippy::must_use_candidate)]
#![allow(clippy::wildcard_imports)]

use enclose::enc;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    time_from_js: Option<String>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    JsReady(bool),
    Tick(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::JsReady(ready) => {
            if ready {
                log!("JS ready!");
                // We don't want to create recursive calls between Rust and JS,
                // because our closures (see `create_closures_for_js` below) close `App` clones
                // - app panics if JS tries to call `update` function,
                // while we are still in `update` function.
                // We break the recursive chain with the simple future.
                //
                // _Note:_ Create an issue in Seed's repo if this solution is not usable for you,
                // we can find another one or try to integrate some locks.
                orders.perform_cmd(async { enableClock() });
            } else {
                log!("JS is NOT ready!");
            }
        }
        Msg::Tick(time) => model.time_from_js = Some(time),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        h1![
            style![
                St::TextAlign => "center"
                St::MarginTop => unit!(40, vmin)
                St::FontSize => unit!(10, vmin)
                St::FontFamily => "monospace"
            ],
            model.time_from_js.clone().unwrap_or_default()
        ],
        div![
          C!["mdc-slider"],
          div![
            C!["mdc-slider__track"],
            div![
              C!["mdc-slider__track--active"], 
              div![
                C!["mdc-slider__track--active_fill"],
              ],
            ],
            div![
              C!["mdc-slider__track--inactive"],
            ],
          ],
          div![
            C!["mdc-slider__thumb"],
            attrs!{
              At::from("role") => "slider",
              At::from("tabindex") => "0",
              At::from("aria-valuemin") => "0",
              At::from("aria-valuemax") => "100",
              At::from("aria-valuenow") => "50",
            },
            div![
              C!["mdc-slider__thumb-knob"],
            ],
          ],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen]
// `wasm-bindgen` cannot transfer struct with public closures to JS (yet) so we have to send slice.
pub fn start() -> Box<[JsValue]> {
    let app = App::start("app", init, update, view);

    create_closures_for_js(&app)
}

fn create_closures_for_js(app: &App<Msg, Model, Node<Msg>>) -> Box<[JsValue]> {
    let js_ready = wrap_in_permanent_closure(enc!((app) move |ready| {
        app.update(Msg::JsReady(ready))
    }));

    let tick = wrap_in_permanent_closure(enc!((app) move |time| {
        app.update(Msg::Tick(time))
    }));

    vec![js_ready, tick].into_boxed_slice()
}

fn wrap_in_permanent_closure<T>(f: impl FnMut(T) + 'static) -> JsValue
where
    T: wasm_bindgen::convert::FromWasmAbi + 'static,
{
    // `Closure::new` isn't in `stable` Rust (yet) - it's a custom implementation from Seed.
    // If you need more flexibility, use `Closure::wrap`.
    let closure = Closure::new(f);
    let closure_as_js_value = closure.as_ref().clone();
    // `forget` leaks `Closure` - we should use it only when
    // we want to call given `Closure` more than once.
    closure.forget();
    closure_as_js_value
}

// ------ ------
//    Extern
// ------ ------

#[wasm_bindgen]
extern "C" {
    fn enableClock();
}
