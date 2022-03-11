use flour::BCCAD;
use std::fs::File;
fn main() {
    let mut f = File::open("../bread/build/libs/agb_tap.bccad").unwrap();
    let bccad = BCCAD::from_bccad(&mut f);
    match bccad {
        Ok(c) => match c.to_json() {
            Ok(d) => println!("{}", d),
            Err(e) => eprintln!("Error in loading BCCAD: {}", e),
        },
        Err(e) => eprintln!("Error in loading BCCAD: {}", e),
    }
}
