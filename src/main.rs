//Let hashmap usable without having to call all the path everytime
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

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
    }
}

struct Todo{
    //single field struct called "map"
    map: HashMap<String, bool>,
}

impl Todo {
    //the follow block is not a method once it doesn't starts with "self"
    fn new() -> Result<Todo, std::io::Error>{
        let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.txt")?;
    
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
    }

    //fn is for function
    //"mut" turns variables mutables
    fn insert (&mut self, key: String){
        //insert a new item in our map
        //value defined to be true
        self.map.insert(key,true);
    }

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
    }
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