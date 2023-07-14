use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use linecount::count_lines;
use std::thread::available_parallelism;
use tokio::task;
use std::sync::Arc;


fn move_files(thread_id: usize, filepath: &str, line_count: usize, thread_count: usize ){
    println!("hola, las lineas que le corresponden al thread {} son: ", thread_id);
    let mut to_read_lines = line_count/thread_count;
    if thread_id < line_count % thread_count {
        to_read_lines += 1;
    }

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
    let binding = available_parallelism().unwrap().get().to_string();
    let matches = App::new("multimove")
        .version("0.1")
        .author("Santiago Calligari. <santiago@calligari.ar> \n Chad G. Pete")
        .about("Given a file and a path moves all the files in the file to the new path")
        .arg(Arg::with_name("filepath")
             .required(true)
             .help("you have to specify the file to be readed"))
        .arg(Arg::with_name("threads")
             .short("t")
             .default_value(&binding)
             .required(false)
             .help("You can limit the amount of threads you want to use to move the files"))
        .get_matches();
    
    let thread_count: usize = matches
        .value_of("threads")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    
    let filepath = Arc::new(
        matches
            .value_of("filepath")
            .expect("No filepath provided")
            .to_owned(),
    );


    let filepath_clone = Arc::clone(&filepath);
    let lines = count_lines(std::fs::File::open(&*filepath_clone).unwrap()).unwrap();
    for thread_id in 0..thread_count {
        let filepath_clone = Arc::clone(&filepath);

        tokio::spawn( async move {
            {
                move_files(thread_id, &filepath_clone, lines, thread_count);
            }
            task::yield_now().await;
        } ).await.unwrap();
    }
}
