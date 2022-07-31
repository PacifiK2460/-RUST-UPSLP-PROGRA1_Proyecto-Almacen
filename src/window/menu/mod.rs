use crossterm::{*, terminal::*, cursor::*, style::{Stylize, Color}};


pub struct Opcion {
    titulo: String,
    descripcion: String,
    accion: fn(),
}

pub struct menu {
    titulo: String,
    opciones: Vec<Opcion>,
    selected: i32,
}

impl Opcion {
    pub fn new(titulo: String, descripcion: String, accion: fn()) -> Opcion {
        Opcion {
            titulo,
            descripcion,
            accion,
        }
    }
}

impl menu {
    pub fn new(titulo: String, opciones: Vec<Opcion>) -> menu {
        menu {
            titulo,
            opciones,
            selected: 0,
        }
    }

    pub fn draw(&self) {
        loop {
            // Limpia la pantalla
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All);
            // Imprimimos el titulo
            {
                crossterm::cursor::MoveTo(4, 2);
                print!("{}", 
                    self.titulo.clone()
                        .with(Color::Rgb{r: 255, g: 255,b: 255})
                        .on(Color::Rgb{r: 16,g:  158,b: 94})
                );
            }

            // Imprimimos las opciones
            {

            }

            // Imprimirmos la ayuda

            // Ejecutamos la funcion seleccionada
            {

            }
        }
    }
}
