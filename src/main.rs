use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use text_io::read;

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).expect("El archivo seleccionado no existe.");
    return io::BufReader::new(file).lines();
}

fn main() {
    println!("Seleccione el laberinto a utilizar (sin la extensión del archivo)");
    let nombre_laberinto:String = read!();
    let mut ruta_laberinto:String = "src/laberintos/".to_string();
    ruta_laberinto.push_str(&nombre_laberinto);
    ruta_laberinto.push_str(".txt");
    let lines = read_lines(ruta_laberinto);
    for line in lines{
        println!("{}",line.unwrap())
    }
}
