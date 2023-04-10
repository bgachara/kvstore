use std::env::args;
use std::fs::write;
use std::iter::Skip;
use std::collections::HashMap;

fn main() {
    let mut arguments: Skip<std::env::Args> = args().skip(1);
    let key: String = arguments.next().unwrap();
    let value: String = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);

    let contents: String = format!("{}\t{}\n", key, value);
    write("jara.db", contents);

    let database: Database = Database::new();
    
}


struct Database {
    map: Hashmap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        //read the jara db file
        let mut map = Hashmap::new();
        let contents: String = match std::fs::read_to_string("jara.db") {
            Ok(c:String) => c,
            Err(error:Error) => {
                return Err(error);
            }
        };
        //other implementation of above
        //let contents: String = std::fs::read_to_string("jara.db")?;
        for line: &str in contents.lines(){
            let mut chunks = line.splitn(2, '\t');
            let key: &str = chunks.next().expect("No key");
            let value: &str = chunks.next().expect("No value");
            map.insert(key, value);
        }
        //parse the string
        //populate our map
        Ok(Database{map: map})
    }
}
