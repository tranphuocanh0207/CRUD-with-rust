/*
CRUD Projects
=== STUDENTS ==
1. CREATED
2. READ
3. UPDATED
4. DELETED
*/
use std::io;

enum Manager {
    AddStudent,
    ViewStudents,
    EditStudent,
    DeleteStudent,
}

#[derive(Debug)]
pub struct Student {
    name: String,
    age: i32,
}

pub struct Students {
    class : Vec<Student>,
}

impl Students {
    fn new() -> Students {
        Students { class: Vec::new() }
    }

    fn add(&mut self, student : Student) {
       Vec::push(&mut self.class, student);        
    }

    fn view(&self) -> Vec<&Student> {
        self.class.iter().collect()
    }
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    while io::stdin().read_line(&mut input).is_err() {
        println!("Please enter input again!!!!");
    }
    let output: String = input.trim().to_owned();
    if &output == "" {
        None
    } else {
        Some(output)
    }
}

fn get_input_number() -> Option<i32> {
    let number = match get_input() {
        Some(input) => input,
        None => return None,
    };
    
    let parsed_number: Result<i32, _> = number.parse();
    if parsed_number.is_err() {
        return None
    } else {
        return Some(parsed_number.unwrap())
    }

}

pub mod manage {
    use crate::*;

    pub fn add_student(students :&mut Students) {
        println!("Enter student name:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter student age:");
        let age = match get_input_number() {
            Some(input) => input,
            None => return,
        };
        let student = Student { name, age};
        println!("Student {:?}", student);
        students.add(student);
    }

    pub fn view(students: &Students) {
        let list =  students.view();
        for student in list {
            println!("{:?}",student);
        }
    }
}

impl Manager {
    fn show() {
        println!("");
        println!("===============================");
        println!("MANAGE STUDENTS");
        println!("");
        println!("1. Add student");
        println!("2. View students");
        println!("3. Edit student");
        println!("4. Delete student");
        println!("");
        println!("Please Enter your choice!!!");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudents),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn main() {
    let mut students = Students::new();
    loop {
        Manager::show();
        let input: Option<String> = get_input();
        match Manager::choice(&input.unwrap()) {
            Some(Manager::AddStudent) => manage::add_student(&mut students),
            Some(Manager::ViewStudents) => manage::view(&students),
            Some(Manager::EditStudent) => println!("3. Edit student"),
            Some(Manager::DeleteStudent) => println!("4. Delete student"),
            _ => return,
        }
    }
}
