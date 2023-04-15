use std::{collections::HashSet, ops::Add};

use rand::Rng;

/*
Viable Tetris shapes include Tetrominos like
    I, J, L, O, S, T, Z
from https://de.wikipedia.org/wiki/Tetris
*/

/// Represents the X and Y choords of one "gamePixel" (4 of with make one Tetris shape usually) 
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct XY(pub i32, pub i32);

/// A Shape is just a container for The XY Pixels mostly.
/// The Different Tetris Blocks like Square-Shape, L-Shape...
/// But also fragments of those (in the sticky blob of left over pixels at the bottom)
#[derive(Debug)]
pub struct Shape {
    /// Pixels that are "filled out"
    pixels: HashSet<XY>,
    /// Anchor we rotate on  
    anchor: XY,
    /// info about color etc:
    typ: &'static str,
}

impl Shape {
    // getter ( short syntax bound to the self lifetime with +'_)
    pub fn get_pixels(&self) -> impl Iterator<Item = XY> +'_{
        self.pixels.iter().copied()
    }

    // getter
    pub fn get_typ(&self) -> &'static str{
        self.typ
    }

    pub fn has_xy(&self, xy: XY) -> bool {
        self.pixels.contains(&xy)
    }

    // compares 2 HashSets for collision (so therefore we know the pixels do not overlap)
    pub fn collides_with(&self, other: &Shape)->bool{
        self.pixels.intersection(&other.pixels).count() > 0
    }

    /// returns a new clockwise rotated copy of a shape.
    pub fn rotated_shape(&self) -> Self{
        let XY(x_off, y_off) = self.anchor;
        let new_pixels = self.get_pixels().map(|XY(x,y)|{
            // for a clockwise rotation:
            // first we subtract our anchor (basically an offset)
            // then new_x = -old-y and new_y = old_x
            // and last step is adding back the offset
            XY(-y+y_off+x_off, x-x_off+y_off)
        }).collect();
        Self { pixels: new_pixels, anchor: self.anchor, typ:self.typ}
    }

    pub fn remove_line(&mut self, y: i32){
        self.pixels = self
            .pixels
            .iter()
            .copied()
            .filter(|xy| xy.1 != y)
            .map(|xy| if xy.1 >= y {xy} else {
                XY(xy.0, xy.1+1)
            })
            .collect();
    }
    
    // constructors:
    pub fn new_i() -> Self {
        Self {
            pixels: [XY(-1, 0), XY(0, 0), XY(1, 0), XY(2, 0)]
                .into_iter()
                .collect(),
            anchor: XY(0, 0),
            typ: "🟦",
        }
    }
    pub fn new_j() -> Self {
        Self {
            pixels: [XY(-1, 0), XY(-1, 1), XY(0, 1), XY(1, 1)]
                .into_iter()
                .collect(),
            anchor: XY(0, 1),
            typ: "🟫",
        }
    }
    pub fn new_l() -> Self {
        Self {
            pixels: [XY(-1, 1), XY(0, 1), XY(1, 1), XY(1, 0)]
                .into_iter()
                .collect(),
            anchor: XY(0, 1),
            typ: "🟧",
        }
    }
    pub fn new_o() -> Self {
        Self {
            pixels: [XY(0, 0), XY(1, 0), XY(0, 1), XY(1, 1)]
                .into_iter()
                .collect(),
            anchor: XY(0, 1),

            typ: "🟨",
        }
    }
    pub fn new_s() -> Self {
        Self {
            pixels: [XY(-1, 1), XY(0, 0), XY(0, 1), XY(1, 0)]
                .into_iter()
                .collect(),
            anchor: XY(0, 1),
            typ: "🟩",
        }
    }
    pub fn new_t() -> Self {
        Self {
            pixels: [XY(0, 0), XY(0, 1), XY(-1, 1), XY(1, 1)]
                .into_iter()
                .collect(),
            anchor: XY(0, 1),

            typ: "🟪",
        }
    }
    pub fn new_z() -> Self {
        Self {
            pixels: [XY(-1, 0), XY(0, 0), XY(0, 1), XY(1, 1)]
                .into_iter()
                .collect(),
            anchor: XY(0, 1),

            typ: "🟥",
        }
    }

    /// generates a random new Shape
    pub fn new() -> Self {
        let rng = rand::thread_rng().gen_range(0..7); //TODO: when all figures implemented check if all get drawn
        match rng {
            0 => Self::new_i(),
            1 => Self::new_j(),
            2 => Self::new_l(),
            3 => Self::new_o(),
            4 => Self::new_s(),
            5 => Self::new_t(),
            6 => Self::new_z(),
            _ => unreachable!(), // as long as we only use default shapes this should never trigger
        }
    }
}


/// translate/modify the the Shape by a XY-Position. +(XY=(3,1) -> 3 to the right 1 down)
/// we accomplish this by overloading the Add method therefore + syntax becomes available
impl Add<XY> for &Shape{
    type Output = Shape;
    fn add(self, rhs: XY) -> Self::Output{
        Shape {
            pixels: self.pixels.iter()
                .map( |xy| xy + rhs)
                .collect(),
            anchor: &self.anchor + rhs,
            typ: &self.typ
        }
    }
}

/// translate/modify the the XY-Positions by another XY-Position. +(XY=(3,1) -> 3 to the right 1 down)
/// we accomplish this by overloading the Add method therefore + syntax becomes available
impl Add<XY> for &XY{
    type Output = XY;
    fn add(self, rhs: XY) -> Self::Output{
        XY(self.0+rhs.0,   self.1+rhs.1)
    }
}