use pc_remote::{ Result, input::{ Keyboard, Key } };

fn main() -> Result<()> {
    let mut keyboard = Keyboard::new()?;

    let hotkey = [Key::Shift, Key::A];
    
    keyboard.press(&hotkey)?;
    keyboard.release(&hotkey)?;

    Ok(())
}
