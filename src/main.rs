mod window;
use crossterm::event::{read, Event};

fn main(){
    let _main = window::menu::menu::new(
        vec![
            window::menu::Opcion::new(
                "Titulo".to_string(),
                "Descripcion".to_string(),
                || {
                    println!("Accion");
                }
            ),
            window::menu::Opcion::new(
                "Titulo".to_string(),
                "Descripcion".to_string(),
                || {
                    println!("Accion");
                }
            ),
        ]
    );


    loop {
        // `read()` blocks until an `Event` is available
        match read().unwrap() {
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
}


