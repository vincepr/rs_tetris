pub mod buf;
pub mod shapes;
use self::buf::RingBuffer;
use self::shapes::{Shape, XY};

/*
    Implements the Game logic.
        - excluding:
            - frontend
            - timer implementation
            - io/ -input (keypresses etc)
*/

/// The Game(State) itself
#[derive(Debug)]
pub struct Tetris {
    // game state
    score: i32,
    game_over: bool,
    // size of the playing field
    width: i32,
    height: i32,
    /// Player controlled shape and the next shape
    current_shape: Shape,
    next_shapes: RingBuffer<Shape>,
    /// Pixels that build up on the bottom of the game:
    sticky_bottom_shapes: Vec<Shape>,
}
#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl Tetris {
    // constructor, starts a new game of width X heigth Pixels/Blocks
    pub fn new(width: u32, height: u32) -> Self {
        let initial_next_shapes = vec![
            &Shape::new() + XY((width as i32) / 2, 0),
            &Shape::new() + XY((width as i32) / 2, 0),
            &Shape::new() + XY((width as i32) / 2, 0),
            &Shape::new() + XY((width as i32) / 2, 0),
        ];
        Self {
            score: 0 as i32,
            game_over: false,
            width: width as i32,
            height: height as i32,
            // current shape starts in middle of screen (half width):
            current_shape: &Shape::new() + XY((width as i32) / 2, 0),
            next_shapes: RingBuffer::new(initial_next_shapes),
            sticky_bottom_shapes: vec![],
        }
    }

    /// get_pixels, expose the game pixels to the outside (then draw the canvas in frontend)
    pub fn get_pixels(&self) -> impl Iterator<Item = XY> {
        // to not depend on the &self lifetime we rebind height and width:
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |y| (0..width).map(move |x| XY(x, y)))
    }

    // gets the next shape to draw the preview
    pub fn get_4x4pixels(&self) -> impl Iterator<Item=XY> {
        let xoff = self.width / 2 - 1;
        (-1..3).flat_map(move |y| (xoff..xoff+4).map(move |x| XY(x,y)))
    }
    // return picels of said 4 by 4 pixels
    pub fn get_4x4type(&self, xy: XY) -> Option<&'static str> {
        if self.next_shapes.peek().has_xy(xy){
            Some(self.next_shapes.peek().get_typ())
        } else {
            None
        }
    }

    pub fn get_score(&self) -> String {
        format!("score: {}", self.score)
    }

    /// get type of the shape on point xy
    pub fn get_typ(&self, xy: XY) -> Option<&'static str> {
        if self.current_shape.has_xy(xy) {
            // xy is in current shape:
            Some(self.current_shape.get_typ())
        } else {
            // check if sticky_bottom contains xy:
            self.sticky_bottom_shapes
                .iter()
                .find(|shape| shape.has_xy(xy))
                .map(|shape| shape.get_typ())
        }
    }

    // private helper functionality:

    // check if a shape is colliding with the game pixels
    fn is_colliding(&self, shape: &Shape) -> bool {
        self.sticky_bottom_shapes
            .iter()
            .any(|s| s.collides_with(shape))
    }

    // check if a shape is ouside of the game field
    fn is_out_of_bounds(&self, shape: &Shape) -> bool {
        !shape.get_pixels().all(|xy| {
            // we check if were in bounds of the gamefield:
            xy.0 >= 0 && xy.0 < self.width && xy.1 >= 0 && xy.1 < self.height
        })
    }

    // checks if there is a fully filled line (so we can remove, add points etc.)
    fn is_line_full(&self, y: i32) -> bool {
        let len = self
            .sticky_bottom_shapes
            .iter()
            .flat_map(|s| s.get_pixels())
            .filter(|xy| xy.1 == y)
            .collect::<std::collections::HashSet<_>>()
            .len() as i32;
        return len == self.width;
    }

    // removes a (full) line of rows from the game and makes the line above "fall down"
    fn remove_line(&mut self, y: i32) {
        for shape in self.sticky_bottom_shapes.iter_mut() {
            shape.remove_line(y);
        }
    }

    // Points per line cleared: 1line:40     2lines:100      3liens:300  4lines:1200
    fn remove_full_lines(&mut self) {
        let mut lines_cleared = 0;
        for y in 0..self.height {
            if self.is_line_full(y) {
                self.remove_line(y);
                lines_cleared +=1;
            }
        }
        match lines_cleared{
            4 => self.score += 1200,
            3 => self.score += 300,
            2 => self.score += 100,
            1 => self.score += 40,
            _ => {}
        }
    }

    /// Main Game Loop, gets Called from frontend.
    /// Every Tick the block moves down one field
    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        let new_pos = &self.current_shape + XY(0, 1); // move 1 pixel down
        if self.is_out_of_bounds(&new_pos) || self.is_colliding(&new_pos) {
            // current shape hit bottom
            // -> so we 1. add the current shape to the sticky_bottom_shapes:
            // ->    we 2. create a new current shape for the top:
            self.next_shape();

            self.remove_full_lines();

            // if no more space left -> GameOver:
            if self.is_colliding(&self.current_shape) {
                self.game_over = true;
            }
        } else {
            self.current_shape = new_pos;
        }
    }

    // helper for tick(), gets a new shape from the "RingBuffered" next_shapes queue
    // sets that shape to current shape and inserts a new shape to the queue
    fn next_shape(&mut self) {
        let random_shape = &Shape::new() + XY(self.width / 2, 0);
        let next_shape = self.next_shapes.pop_and_push(random_shape);

        let new_sticky = std::mem::replace(&mut self.current_shape, next_shape);
        self.sticky_bottom_shapes.push(new_sticky);
    }

    /// Player Interacting with left right input -> move shape left/right
    pub fn move_current_shape(&mut self, dir: Direction) {
        if self.game_over {
            return;
        }
        let move_dir = match dir {
            Direction::Left => XY(-1, 0),
            Direction::Right => XY(1, 0),
        };
        let new_pos = &self.current_shape + move_dir;
        // check if new position is not an illegal move:
        if !(self.is_out_of_bounds(&new_pos) || self.is_colliding(&new_pos)) {
            self.current_shape = new_pos;
        }
    }

    /// Player Interacting with up/down input -> rotate current shape
    pub fn move_rotate(&mut self) {
        if self.game_over {
            return;
        }
        // first we check if the rotated Position is a legal move:
        let mut new_shape = self.current_shape.rotated_shape();
        if self.do_if_is_legal_move(new_shape) {
            return
        }
        // If Basic Rotation fails, 'Wall Kicks' are attempted to push the shape 'away from the sides etc.'
        // https://tetris.wiki/Super_Rotation_System
        // Since this version only implements one rotation anyways we just simply try to push the shape left and right by an offset and call it a day
        new_shape = &self.current_shape.rotated_shape() +  XY(1, 0);
        if self.do_if_is_legal_move(new_shape){
           return
        }
        new_shape = &self.current_shape.rotated_shape() +  XY(2, 0);
        if self.do_if_is_legal_move(new_shape){
           return
        }
        new_shape = &self.current_shape.rotated_shape() +  XY(-1, 0);
        if self.do_if_is_legal_move(new_shape){
           return
        }
        new_shape = &self.current_shape.rotated_shape() +  XY(-2, 0);
        if self.do_if_is_legal_move(new_shape){
           return
        }
    }

    // check if new position is not an illegal move (collision with existing shapes or out of bounds)
    fn do_if_is_legal_move(&mut self, new_shape: Shape) -> bool{
        if !self.is_out_of_bounds(&new_shape) && !self.is_colliding(&new_shape) {
            self.current_shape = new_shape;
            return true
        }
        return false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut gamestate = Tetris::new(10, 30);
        println!("{:#?}", gamestate);
        gamestate.tick();
        gamestate.tick();
        gamestate.tick();

        println!("{:#?}", gamestate);
    }
}
