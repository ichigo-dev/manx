use std::io::Read;
use std::fs;
use std::path::Path;

const LOGO_PATH: &str = "logo.txt";

//------------------------------------------------------------------------------
//  Display logo.
//------------------------------------------------------------------------------
fn display_logo()
{
    let path = Path::new(LOGO_PATH);
    let mut file = match fs::File::open(path)
    {
        Ok(file) => file,
        Err(err) => panic!("couldn't open a logo file: {:?}", err),
    };

    let mut s = String::new();
    if let Err(err) = file.read_to_string(&mut s)
    {
        panic!("couldn't load a logo file: {:?}", err);
    }
    else
    {
        for c in s.chars()
        {
            match c
            {
                _ => print!("\x1b[34m{}\x1b[0m", c),
            }
        }
    }
    print!("\n");
}

fn main()
{
    display_logo();
}
