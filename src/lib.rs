use js_sys::{Function, Reflect};
use tetris_game::{Direction, Tetris};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
    c, export_components, h,
    hooks::{use_callback, use_effect, use_js_ref, use_state, Deps},
    props::Style,
    Component,
};
use web_sys::{window, Element, HtmlElement, KeyboardEvent};

pub mod tetris_game;

/*
*   Frontend for the teris game.
*   Uses react-library for rust to generate some javascript that runs react
*   and hits the wasm generated instance of the teris game with user input or timer-ticks...
*/

pub struct App {
    width: u32,
    height: u32,
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(App {
            width: Reflect::get(&value, &"width".into())?
                .as_f64()
                .unwrap_or(10.0) as u32,
            height: Reflect::get(&value, &"height".into())?
                .as_f64()
                .unwrap_or(20.0) as u32,
        })
    }
}

impl Component for App {
    fn render(&self) -> wasm_react::VNode {
        let tetris = use_state(|| Tetris::new(self.width, self.height));
        let speed = use_state(|| 500);

        
        // autofocus the div handling key_down events once mounted:
        let container = use_js_ref::<Element>(None);
        use_effect(
            {
                let container = container.clone();
                move || {
                    container
                        .current()
                        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                        .map(|el| el.focus().ok());

                    || ()
                }
            },
            Deps::none(),
        );

        // the timer the game loop runs on:
        //  - for each tick the game moves down once.
        use_effect(
            {
                let tetris = tetris.clone();
                let speed = *speed.value();
                move || {
                    let tick_closure = Closure::new({
                        let mut tetris = tetris.clone();
                        move || {
                            tetris.set(|mut tetris| {
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
                        window().unwrap_throw().clear_interval_with_handle(handle)
                    }
                }
            },
            Deps::some(*speed.value()),
        );

        // event handler for user input:
        // - up :rotate
        // - left/right -press :try to move left and right
        // - down -press :move down once
        let handle_key_down = use_callback(
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
                        tetris.set(|mut tetris| {
                            tetris.move_current_shape(direction);
                            tetris
                        });
                    }

                    if code == "ArrowUp" {
                        tetris.set(|mut tetris| {
                            tetris.move_rotate();
                            tetris
                        })
                    } else if code == "ArrowDown" {
                        tetris.set(|mut tetris| {
                            tetris.tick();
                            tetris
                        });
                        speed.set(|_| 55)
                    }
                }
            },
            Deps::none(),
        );

        // event handler for user input:
        // - down -keepholding :speeds up the tick-rate while button is pressed
        let handle_key_up = use_callback(
            {
                let mut speed = speed.clone();
                move |ev: KeyboardEvent| {
                    let code = ev.code();
                    if code == "ArrowDown" {
                        speed.set(|_| 500);
                    }
                }
            },
            Deps::none(),
        );

        // div for the 'whole page' to just listen for on_keydown everywhere, gets autofocus with use_effect
        h!(div)
            .ref_container(&container)
            .tabindex(0)
            .on_keydown(&handle_key_down)
            .on_keyup(&handle_key_up)
            .style(
                &Style::new()
                    .width("100%")
                    .height("100%")
                    .outline("none")
            )
            .build(c![

                

                // the div that holds the game-canvas-pixels:
                h!(div)
                    .style(
                        &Style::new()
                            .display("inline-grid")
                            .grid_template(format!(
                                "repeat({}, 1em) / repeat({}, 1em)",
                                self.height, self.width
                            ))
                            .outline("none")
                            .border("3px solid grey")
                            .margin_top("2rem")
                            .margin_left("2rem")
                    )
                    // divs making up the canvas-pixels:
                    .build(c![..tetris.value().get_pixels().map(|xy| {
                        let typ = tetris.value().get_typ(xy);

                        h!(div)
                            .style(&Style::new().text_indent("-.1em").margin_top("-.1em"))
                            .build(c![typ.unwrap_or_default()])
                    })])
                ,
                
                // the preview-block for upcoming block:
                h!(div)
                    .style(
                        &Style::new()
                            .display("inline-grid")
                            .grid_template(format!(
                                "repeat({}, 1em) / repeat({}, 1em)",
                                4, 4
                            ))
                            .outline("none")
                            .border("3px solid grey")
                            .margin_left("0.5rem")
                    )
                    .build(c![..tetris.value().get_4x4pixels().map(|xy| {
                        let typ = tetris.value().get_4x4type(xy);

                        h!(div)
                            .style(&Style::new().text_indent("-.1em").margin_top("-.1em"))
                            .build(c![typ.unwrap_or_default()])
                    })])
                ,

                // score:
                h!(div)
                    .style(&Style::new().margin_left("2.1rem").color("lightcyan"))
                    .build(c![tetris.value().get_score()]),
            ])
    }
}

export_components! {App}
