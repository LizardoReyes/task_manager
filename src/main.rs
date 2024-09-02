use clap::{Arg, Command};
use rusqlite::{params, Connection, Result};
use serde::Serialize;
use std::num::ParseIntError;

#[derive(Serialize)]
struct Task {
    id: i32,
    description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Tu Nombre")
        .about("Una aplicación simple para administrar tareas")
        .subcommand(
            Command::new("add")
                .about("Añadir una nueva tarea")
                .arg(Arg::new("description").required(true).help("Descripción de la tarea")),
        )
        .subcommand(Command::new("list").about("Listar todas las tareas"))
        .subcommand(
            Command::new("delete")
                .about("Eliminar una tarea por ID")
                .arg(Arg::new("id").required(true).help("ID de la tarea")),
        )
        .get_matches();

    let conn = Connection::open("tasks.db")?;

    // Crear la tabla si no existe
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL
        )",
        [],
    )?;

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let description = sub_matches.get_one::<String>("description").unwrap();
            conn.execute("INSERT INTO tasks (description) VALUES (?1)",
                         params![description])?;
            println!("Tarea añadida: {}", description);
        }
        Some(("list", _)) => {
            let mut stmt = conn.prepare("SELECT id, description FROM tasks")?;
            let task_iter = stmt.query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                })
            })?;

            for task in task_iter {
                let task = task?;
                println!("ID: {}, Descripción: {}", task.id, task.description);
            }
        }
        Some(("delete", sub_matches)) => {
            let id_str = sub_matches.get_one::<String>("id").unwrap();
            let id: i32 = id_str.parse().map_err(|e: ParseIntError| -> Box<dyn std::error::Error> { Box::new(e) })?;

            conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
            println!("Tarea eliminada con ID: {}", id);
        }
        _ => {
            eprintln!("Comando no reconocido");
        }
    }

    Ok(())
}
