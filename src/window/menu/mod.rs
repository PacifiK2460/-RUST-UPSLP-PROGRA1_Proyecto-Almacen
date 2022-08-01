use std::{io::stdout, string};

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
    ayuda: Vec<(String,String)>
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
            ayuda: vec![
                ("↑".to_string(), "Moverse hacia arriba".to_string()),
                ("↓".to_string(), "Moverse hacia abajo".to_string()),
                ("Enter".to_string(), "Seleccionar opción".to_string()),
                ("Escape".to_string(), "Salir del menú".to_string()),
            ]
        }
    }

    pub fn draw(&mut self) {
        // Ocultamos el cursor
        match execute!(stdout(),crossterm::cursor::Hide){
            Ok(_) => (),
            Err(e) => {
                panic!("Error al ocultar el cursor: {}", e);
            },
        };
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
            let size = crossterm::terminal::size().unwrap();
            
            match execute!(stdout(), crossterm::cursor::MoveTo(4, size.1 as u16 - self.ayuda.len() as u16)){
                Ok(_) => (),
                Err(e) => {
                    panic!("Error al mover el cursor: {}", e);
                },
            };
            
            for (tecla, desc) in self.ayuda.iter() {
                print!("{} {}\n", 
                    tecla.clone()
                        .with(Color::Rgb{r: 185, g: 251,b: 192}),
                    desc.clone()
                        .dim()
                );
            }
        }
        
        // Imprimimos las opciones
        self.focus();
        
        // Mostramos el cursor
        match execute!(stdout(),crossterm::cursor::Show){
            Ok(_) => (),
            Err(e) => {
                panic!("Error al mostrar el cursor: {}", e);
            },
        };
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
                                    //Ejecutamos la opcion seleccionada
                                    (self.opciones[self.selected as usize].accion)();
                                }
                                KeyCode::Left => {
                                    // Salimos del loop
                                    break;
                                },
                                KeyCode::Null => panic!("No se puede usar la tecla NULL"),
                                _ => {},
                            }
                        },
                        _ => {}
                    }
                },
                Err(e) => {
                    panic!("Unhandled error: {}", e);
                }
            }
        }
    }

    fn display(&self){
        // Nos movemos al inicio de la pantalla
        let x = 4;
        match execute!(stdout(), crossterm::cursor::MoveTo(x, 3)){
            Ok(_) => (),
            Err(e) => {
                panic!("Error al mover el cursor: {}", e);
            },
        };
        
        // Imprimimos las opciones y sus descripciones
        for (actual, opcion) in self.opciones.iter().enumerate() {
            let opcionTitulo = "▏".to_string().push_str(&opcion.titulo);
            let opcionDesc = "▏".to_string().push_str(&opcion.titulo);

            // Imprimimos las opciones
            if actual == self.selected as usize {
                print!("{}\n{}", 
                    menuvline.push_str(&opcion.titulo)
                        .with(Color::Rgb{r: 255, g: 255,b: 255}),
                );
            } else {

            }

        }

    }
}