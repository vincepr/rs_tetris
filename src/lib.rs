use js_sys::{Reflect, Function};
use tetris_game::{Tetris, Direction};
use wasm_bindgen::{JsValue, UnwrapThrowExt, prelude::Closure, JsCast};
use wasm_react::{Component, h, c, hooks::{use_state, use_effect, Deps, use_callback}, export_components, props::Style};
use web_sys::{window, KeyboardEvent};

pub mod tetris_game;

/*
*   UI (react with a rust wasm binding) all in here atm:
*/

pub struct App{
    width: u32,
    height: u32,
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(
            App {
                width: Reflect::get(&value, &"width".into())?
                    .as_f64().unwrap_or(10.0) as u32,
                height: Reflect::get(&value, &"height".into())?
                    .as_f64().unwrap_or(20.0) as u32,
            }
        )
    }
}

impl Component for App {
    fn render(&self) -> wasm_react::VNode{
        let tetris = use_state(|| Tetris::new(self.width, self.height));
        let speed = use_state(|| 500);

        // the timer the game loop runs on:
        use_effect({
            let tetris = tetris.clone();
            let speed = *speed.value();
                move || {
                    let tick_closure = Closure::new({
                        let mut tetris = tetris.clone();
                        move || {
                            tetris.set(|mut tetris|{
                                tetris.tick();
                                tetris
                            })
                        }
                    });
                let handle = window()
                    .unwrap_throw()
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                        speed,
                    )
                    .unwrap_throw();
                move || {
                    drop(tick_closure);
                    window()
                        .unwrap_throw()
                        .clear_interval_with_handle(handle)
                }
            }}, Deps::some(*speed.value())
        );

        // event handler for user input:
        let mut handle_key_down = use_callback(
            {
                let mut tetris = tetris.clone();
                let mut speed = speed.clone();

                move |ev: KeyboardEvent| {
                    let code = ev.code();

                    let direction = match &*code {
                        "ArrowLeft" => Some(Direction::Left),
                        "ArrowRight" => Some(Direction::Right),
                        _ => None,
                    };
                    if let Some(direction) = direction {
                        tetris.set(|mut tetris|{
                            tetris.move_current_shape(direction);
                            tetris
                        });
                    }

                    if code == "ArrowUp" {
                        tetris.set(|mut tetris|{
                            tetris.move_rotate();
                            tetris
                        })
                    } else if code == "ArrowDown" {
                        speed.set(|_|80)
                    }
                }
            }, 
            Deps::none()
        );

        let handle_key_up = use_callback(
            {
                let mut speed = speed.clone();
                move |ev: KeyboardEvent|{
                    let code = ev.code();
                    if code == "ArrowDown"{
                        speed.set(|_|500);
                    }
                }
            },
            Deps::none()
        );

        // the div that make up the game-canvas-pixels:
        h!(div)
            .tabindex(0)
            .on_keydown(&handle_key_down)
            .on_keyup(&handle_key_up)
            .style(&Style::new()
                .display("inline-grid")
                .grid_template(format!("repeat({}, 1em) / repeat({}, 1em)",self.height, self.width))
                .outline("none")
                .border("1px solid grey"),
            )
            .build(c![
                ..tetris.value().get_pixels().map(|xy| {
                    let typ = tetris.value().get(xy);

                    h!(div)
                        .style(&Style::new().text_indent("-.2em").margin_top("-.2em"))
                        .build(c![typ.unwrap_or_default()])
                })
            ])
    }
}

export_components! {App}