use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

pub fn read_file(source_file: &str) -> String {

    let mut file = match File::open( source_file ) {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content

}

pub fn write_file(filepath: &str, content: &String) {

    let path = Path::new(filepath);
    let display = path.display();

    let mut file = match File::create( &path ) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }

}
