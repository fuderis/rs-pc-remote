use crate::prelude::*;
use enigo::{ Axis, Button, Coordinate, Direction, Enigo, Mouse as EnigoMouse, Settings, };

/// The mouse emulator
#[derive(Debug)]
pub struct Mouse {
    enigo: Enigo,
}

impl Mouse {
    /// Creates a new mouse emulator
    pub fn new() -> Result<Self> {
        Ok(Self {
            enigo: Enigo::new( &Settings::default() )?,
        })
    }

    /// Returns current mouse coordinates
    pub fn coords(&mut self) -> Result<(i32, i32)> {
        self.enigo.location().map_err(From::from)
    }

    /// Returns screen resolution (width, height)
    pub fn display_size(&mut self) -> Result<(i32, i32)> {
        self.enigo.main_display().map_err(From::from)
    }

    /// Move mouse vertically
    pub fn move_to(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo.move_mouse(x, y, Coordinate::Abs).map_err(From::from)
    }

    /// Move mouse horizontally (relative)
    pub fn move_relative(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo.move_mouse(x, y, Coordinate::Rel).map_err(From::from)
    }

    /// Move mouse to center
    pub fn move_to_center(&mut self) -> Result<()> {
        let (w, h) = self.display_size()?;
        let (x, y) = (w / 2, h / 2);
        
        self.enigo.move_mouse(x, y, Coordinate::Abs).map_err(From::from)
    }

    /// Press the left mouse button
    pub fn press_left(&mut self) -> Result<()> {
        self.enigo.button(Button::Left, Direction::Press).map_err(From::from)
    }

    /// Release the left mouse button (uses after 'hold_left' method)
    pub fn release_left(&mut self) -> Result<()> {
        self.enigo.button(Button::Left, Direction::Release).map_err(From::from)
    }

    /// Press the right mouse button
    pub fn press_right(&mut self) -> Result<()> {
        self.enigo.button(Button::Right, Direction::Press).map_err(From::from)
    }

    /// Release the right mouse button (uses after 'hold_right' method)
    pub fn release_right(&mut self) -> Result<()> {
        self.enigo.button(Button::Right, Direction::Release).map_err(From::from)
    }

    /// Scroll horizontally
    pub fn scroll_x(&mut self, delta: i32) -> Result<()> {
        self.enigo.scroll(delta, Axis::Horizontal).map_err(From::from)
    }

    /// Scroll vertically
    pub fn scroll_y(&mut self, delta: i32) -> Result<()> {
        self.enigo.scroll(delta, Axis::Vertical).map_err(From::from)
    }
}
