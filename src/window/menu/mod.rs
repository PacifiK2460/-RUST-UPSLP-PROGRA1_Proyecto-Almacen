pub struct Opcion {
    titulo: String,
    descripcion: String,
    accion: fn(),
}

pub struct menu {
    opciones: Vec<Opcion>,
    selected: i32,
}

impl menu {
    pub fn new(opciones: Vec<Opcion>) -> menu {
        menu {
            opciones,
            selected: 0,
        }
    }
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