struct Persona {
    nombre: String,
    edad: u8,
}

fn main() {
    let persona = Persona {
        nombre: "Juan".to_string(),
        edad: 25,
    };

    println!("Nombre: {}", persona.nombre);
    println!("Edad: {}", persona.edad);
}