use std::io::{stdin};
use std::vec::Vec;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

struct Todo {
   completed: bool,
   content: String
   // date_due: err...
}

impl Todo {
   fn new(content: String, completed: String) -> Self {
      let completed_bool = match completed.as_str() {
         "X" => true,
         _ => false
      };

      Todo {
         content: content,
         completed: completed_bool
      }
   }
}

fn clear_screen() {
   print!("{esc}c", esc = 27 as char)
}

fn create_file_if_not_exists(filename: &str) {
   if !Path::new(filename).exists() {
      let _ = match File::create(filename) {
         Err(e) => panic!("{}", e),
         Ok(file) => file
      };
   }
}

fn load_todo_list_from_file(list: &mut Vec<Todo>, filename: &str) {
   create_file_if_not_exists(filename);
   let file = match File::open(filename) {
      Err(e) => panic!("{}", e),
      Ok(file) => file
   };
   let reader = BufReader::new(file);
   for line in reader.lines() {
      let unwrapped = line.unwrap();
      let todo_parts: Vec<&str> = unwrapped.split(":").collect();
      let todo = Todo::new(todo_parts[1].to_string(), todo_parts[0].to_string());
      list.push(todo);
   }
}

fn save_todo_list_to_file(list: &mut Vec<Todo>, filename: &str) {
   let mut file = match File::create(filename) {
      Err(e) => panic!("{}", e),
      Ok(file) => file
   };

   for todo in list {
      let completed_str = get_todo_completed_str(todo);
      writeln!(file, "{}:{}", completed_str, todo.content).unwrap();
   }
}

fn get_input(prompt: &str) -> String {
   let mut input = String::new();
   println!("{}", prompt);
   stdin().read_line(&mut input).ok();
   return input.trim().to_string();
}

fn create_new_todo() -> Todo {
   let todo_content = get_input("Enter the todo contents:");
   return Todo::new(todo_content, "".to_string());
}

fn edit_todo(list: &mut Vec<Todo>, idx: usize) {
   // todo add check that idx is valid
   let mut todo: &mut Todo = &mut list[idx];

   // show todo contents to user
   println!("Editing '{}'", todo.content);
   // get new contents
   let new_content = get_input("Enter new content for this todo:");
   // save contents
   todo.content = new_content;
}

fn get_todo_completed_str(todo: &Todo) -> &str {
   return match todo.completed {
      true => "X",
      false => " "
   };
}

fn print_todo_list(list: &mut Vec<Todo>) {
   let mut index = 1;
   for todo in list {
      let completed_str = get_todo_completed_str(todo);
      println!("{}: [{}] >> {}", index, completed_str, todo.content);
      index += 1;
   }
}

fn print_command_list() {
   println!("- Commands:");
   println!("  q: quit");
   println!("  e: edit a todo");
   println!("  c: toggle completeness of a todo");
   println!("  n: create new todo");
   println!("  r: reload list of todos");
   println!("  s: save todo list to disk");
}

fn main() -> std::io::Result<()> {

   const TODO_FILENAME: &str = "todos.list";

   // this is the "global" list of todos
   let mut list: Vec<Todo> = Vec::new();

   // load list of todos from disk
   load_todo_list_from_file(&mut list, TODO_FILENAME);

   let mut input = String::new();
   while input != "q" {
      clear_screen();
      print_todo_list(&mut list);
      print_command_list();
      input = get_input("Choose a command:");
      match input.as_str() {
         "e" => {
            let index = get_input("Edit which todo?");
            // display indexes start at 1, so we need to -1 to get an array index
            let idx: usize = index.parse::<usize>().unwrap() - 1;
            edit_todo(&mut list, idx);
         },
         "n" => {
            let todo: Todo = create_new_todo();
            list.push(todo);
         },
         "s" => {
            println!("Saving...");
            save_todo_list_to_file(&mut list, "todos.list");
            println!("Done");
         },
         "r" => {
            list.clear();
            load_todo_list_from_file(&mut list, TODO_FILENAME);
         },
         "q" => {},
         _ => {
            println!("Invalid command given, stop being silly!");
         }
      }
   }


   Ok(())
}