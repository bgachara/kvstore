use std::collections::HashMap;
use std::env::args;
//use std::fs::write;
use std::iter::Skip;

fn main() {
    let mut arguments: Skip<std::env::Args> = args().skip(1);
    let key: String = arguments.next().unwrap();
    let value: String = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);

    //let contents: String = format!("{}\t{}\n", key, value);
    //write("jara.db", contents);

    let mut database: Database = Database::new().expect("What is this");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        //read the jara db file
        //other implementation of above
        let mut map = HashMap::new();
        //let contents: String = match std::fs::read_to_string("jara.db") {
        //Ok(c:String) => c;
        //Err(error:Error) => {
        //return Err(error);
        //}
        //};
        let contents: String = std::fs::read_to_string("jara.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key: &str = chunks.next().expect("No key");
            let value: &str = chunks.next().expect("No value");
            map.insert(key.to_owned(), value.to_owned());
        }
        //parse the string
        //populate our map
        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for pairs in self.map {
            let kvpair = format!("{}\t,{}\n", pairs.0, pairs.1);
            contents.push_str(&kvpair);
        }
        std::fs::write("kv.db", contents)
    }
}
