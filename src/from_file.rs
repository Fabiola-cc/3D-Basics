use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Función para cargar un laberinto desde un archivo
pub fn load_maze(filename: &str) -> Result<Vec<Vec<char>>, io::Error> {
    // Abre el archivo
    let file = File::open(filename)?;
    // Crea un lector de búfer para el archivo
    let reader = BufReader::new(file);

    // Lee las líneas del archivo, las convierte a caracteres y las colecciona en un vector bidimensional
    let maze = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Ok(maze)
}
