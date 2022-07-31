use crossterm::{*, terminal::*, cursor::*, style::{Stylize, Color}, event::{read, Event, KeyEvent, KeyCode}};
use crossterm::event::Event::Key;

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

            // Imprimirmos la ayuda
            {
                todo!();                
            }
            
            // Imprimimos las opciones
            self.focus();

            // Ejecutamos la funcion seleccionada
            {

            }
        }
    }

    fn focus(&mut self){
        loop {
            // Imprimimos las opciones
            self.display();
            // Capturamos el evento
            match read() {
                Ok(event) => {
                    match event {
                        Key(key) => {
                            match key.code {
                                KeyCode::Up => {
                                    if self.selected > 0 {
                                        self.selected -= 1;
                                    }
                                },
                                KeyCode::Down => {
                                    if self.selected < self.opciones.len() as i32 - 1 {
                                        self.selected += 1;
                                    }
                                },
                                KeyCode::Enter => {
                                    (self.opciones[self.selected as usize].accion)();
                                }
                                KeyCode::Left => todo!(),
                                KeyCode::Null => panic!("No se puede usar la tecla NULL"),
                                _ => {},
                            }
                        },
                        _ => {}
                    }
                },
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }
    }

    fn display(&self){
        todo!();
    }
}
