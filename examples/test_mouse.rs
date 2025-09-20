use pc_remote::{ Result, input::Mouse };

fn main() -> Result<()> {
    let mut mouse = Mouse::new()?;

    mouse.move_to_center()?;
    
    Ok(())
}
