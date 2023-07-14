use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use linecount::count_lines;
use std::thread::available_parallelism;
use tokio::task;


fn hola(thread_id: usize, filepath: &str, line_count: usize, thread_count: usize ){
    println!("hola, las lineas que le corresponden al thread {} son: ", thread_id);
    let to_read_lines = line_count/thread_count;
    let reader = BufReader::new(File::open(filepath).expect("cannot open file"));
    let lines = reader.lines().collect::<Result<Vec<String>, _>>().unwrap();

    //println!("La cantidad de lineas a leer son {},  las lineas son: {} los cores son {} ", to_read_lines, line_count, thread_count);
    for i in 0..to_read_lines{
        let to_read = thread_id + i*thread_count;
        if let Some(line) = lines.get(to_read)
        {
            println!("Contenido de la linea {}: {} ", to_read, line);
        }
        else {
            println!("No se encontro la linea {} ", to_read);
        }
    }
}



#[tokio::main]
async fn main() {
    let filepath = "test"; //TODO cambiar por el argumento de clamp
    //let reader = BufReader::new(File::open("test").expect("cannot open file"));
    // TODO unccomment let thread_count = available_parallelism().unwrap().get();
    let thread_count = 3;
    let lines = count_lines(std::fs::File::open(filepath).unwrap()).unwrap();
    /*let secondLine = reader.lines()
        .nth(2)
        .expect("el archivo tiene menos de 2 lineas")
        .expect("no se pudo leer");
*/
    for thread_id in 0..thread_count {

        tokio::spawn( async move {
            {
                hola(thread_id, filepath, lines, thread_count);
            }
            task::yield_now().await;
        } ).await.unwrap();
    }
}
