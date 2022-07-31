mod window;

use std::io::{Write, stdout};

use crossterm::{*, terminal::*, cursor::*, style::{Stylize, Color}};


fn main(){

    match execute!(stdout(), EnterAlternateScreen) {
        Ok(_) => {},
        Err(e) => {
            panic!("No se pudo iniciar el programa: {}", e);
        }
    }

    let main = window::menu::menu::new(
        "Menu Principal".to_string(),
        vec![
            window::menu::Opcion::new(
                "Consulta de almacén".to_string(),
                "Enlista los productos disponibles".to_string(),
                todo!(),
            ),

            window::menu::Opcion::new(
                "Actualizar almacén".to_string(),
                "Sumar / Modificar inventario, agregar productos".to_string(),
                todo!(),
            ),

            window::menu::Opcion::new(
                "Registrar Pedido".to_string(),
                "Registrar información del pedido y del cliente".to_string(),
                todo!(),
            ),

            window::menu::Opcion::new(
                "Consultar Pedido".to_string(),
                "Enlista los pedidos por Activos, Entregados, Cancelados e ID".to_string(),
                todo!(),
            ),

            window::menu::Opcion::new(
                "Registrar entrega".to_string(),
                "Registra la entrega de un pedido y actualiza el inventario".to_string(),
                todo!(),
            ),

            window::menu::Opcion::new(
                "Modificar estado de pedido".to_string(),
                "Modifica el estado del pedido del cliente".to_string(),
                todo!(),
            ),

            window::menu::Opcion::new(
                "Salir del sistema".to_string(),
                "Cierra la sesión actualCierra la sesión actual".to_string(),
                todo!(),
            )
        ]
    );

    main.draw();

    match execute!(stdout(), LeaveAlternateScreen) {
        Ok(_) => {},
        Err(e) => {
            panic!("No se pudo cerrar el programa: {}", e);
        }
    };

}
