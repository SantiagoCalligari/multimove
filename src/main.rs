use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use linecount::count_lines;
use std::thread::available_parallelism;
use tokio::task;
fn hola(){
    println!("hola");
}
#[tokio::main]
async fn main() {
    let reader = BufReader::new(File::open("test").expect("cannot open file"));
    let thread_count = available_parallelism().unwrap().get();
    /*let secondLine = reader.lines()
        .nth(2)
        .expect("el archivo tiene menos de 2 lineas")
        .expect("no se pudo leer");
*/
    for i in 0..thread_count {

        tokio::spawn( async move {
            {
                hola();
            }
            task::yield_now().await;
        } ).await.unwrap();
    }
}
