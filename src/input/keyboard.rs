use crate::prelude::*;
use super::Key;
use enigo::{ Enigo, Direction, Keyboard as EnigoKeyboard, Settings };

/// The keyboard emulator
#[derive(Debug)]
pub struct Keyboard {
    enigo: Enigo,
}

impl Keyboard {
    /// Creates a new keyboard emulator
    pub fn new() -> Result<Self> {
        Ok(Self {
            enigo: Enigo::new( &Settings::default() )?,
        })
    }

    /// Press a keyboard keys
    pub fn press(&mut self, keys: &[Key]) -> Result<()> {
        for key in keys {
            self.enigo.key(key.clone().into(), Direction::Press)?;
        }

        Ok(())
    }

    /// Release a keyboard keys (uses after 'hold' method)
    pub fn release(&mut self, keys: &[Key]) -> Result<()> {
        for key in keys {
            self.enigo.key(key.clone().into(), Direction::Release)?;
        }

        Ok(())
    }
}
