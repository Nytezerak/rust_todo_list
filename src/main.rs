//Let hashmap usable without having to call all the path everytime
use std::collections::HashMap;

/* No more in use since serde is used
use std::io::Read;
use std::str::FromStr;
*/

fn main() {
    //env returns the arguments that the program is started with
    //nth starts on 1 because "0" is reserved for the program
    let action = std::env::args().nth(1).expect("Specify an action");
    let item = std::env::args().nth(2).expect("Specify an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        //"." calls "todo insert"
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved!"),
            Err(why) => println!("An error ocurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved!"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}

struct Todo{
    //single field struct called "map"
    map: HashMap<String, bool>,
}

impl Todo {
    //the follow block is not a method once it doesn't starts with "self"
    fn new() -> Result<Todo, std::io::Error>{
        let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        //.open("db.txt")?;
        .open("db.json")?;
    
        //serializing Json as Hashmap
        match serde_json::from_reader(f){
            //mut f isn't needed to allocate the content on a string anymore once serde does the work
            Ok(map) => Ok(Todo {map}),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
        //fn is for function
        //"mut" turns variables mutables
        fn insert (&mut self, key: String){
            //insert a new item in our map
            //value defined to be true
            self.map.insert(key,true);
        }   

        fn save(self) -> Result<(), Box<dyn std::error::Error>> {
            //open db.json
            let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
            //write to file with serde
            serde_json::to_writer_pretty(f, &self.map)?;
            Ok(())
        }   

        fn complete(&mut self, key: &String) -> Option<()> {
            match self.map.get_mut(key) {
                Some(v) => Some(*v = false),
                None => None,
            }
        }
    /* below is the code that matches "db.txt"
    let mut content = String::new();
    //reads all the bytes in the file and appends them into the <content> string
    f.read_to_string(&mut content)?;
    //convert string to hashmap
    let map: HashMap<String, bool> = content
        .lines()
        //"splitn" split lines on tab character // transforms an interator into a relevant collection //
        .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //"Vec" transforms Split string into a Vector of borrowed string slices by appending "::Vec<&Str> to the method"
        //which tells the compiler the collection we want in the end
        .map(|v| (v[0], v[1]))
        //converts the two elements on tuple to string and boolean
        .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        .collect();
    //if there is no errors return struct with "OK"
    Ok(Todo {map})

    //"->" annotates returned type from the function, here it returns a result
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        //iteration over map separating key and value
        for(k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            //push formated string to content
            content.push_str(&record)
        }
        //saves content on "db.text"
        std::fs::write("db.txt", content)
    }*/
}

    


//fn main(){
//-Rust owners is kinda weird, pay attention! ex:
//-x is the owner:
//let x = String::from("Hello");
//-doSomething function is x owner now:
//doSomething(x)
//-The value will not be printed bellow because the value owner is now "doSomething":
//println!("{}", x);
//}