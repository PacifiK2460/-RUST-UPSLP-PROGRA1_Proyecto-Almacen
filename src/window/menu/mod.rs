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
    pub fn New(opciones: Vec<Opcion>) -> menu {
        menu {
            opciones,
            selected: 0,
        }
    }
}

impl Opcion {
    pub fn New(titulo: String, descripcion: String, accion: fn()) -> Opcion {
        Opcion {
            titulo,
            descripcion,
            accion,
        }
    }
}