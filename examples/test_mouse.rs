use pc_remote::{ Result, Mouse };

fn main() -> Result<()> {
    let mut mouse = Mouse::new()?;

    mouse.move_to_center()?;
    
    Ok(())
}
