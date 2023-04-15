use wasm_react::{Component, h, c};

pub mod tetris_game;

pub struct App;

impl Component for App {
    fn render(&self) -> wasm_react::VNode{
        h!(div).build(c![
            
        ])
    }

}