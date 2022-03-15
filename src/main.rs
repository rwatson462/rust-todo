use std::io::{stdin};
use std::vec::Vec;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Clone)]
struct Todo {
   completed: bool,
   content: String
   // date_due: err...
}

impl Todo {
   fn completed_str(&self) -> &str {
      return match self.completed {
         true => "X",
         _ => " "
      };
   }
}

struct TodoManager {
   list: Vec<Todo>
}

impl TodoManager {
   fn new() -> TodoManager {
      return TodoManager {
         list: Vec::new()
      };
   }

   fn add_todo(&mut self, todo: Todo) {
      self.list.push(todo);
   }

   fn get_todo(&mut self, index: usize) -> Todo {
      return self.list[index].clone();
   }

   fn update_todo(&mut self, index: usize, todo: Todo) {
      self.list.remove(index);
      self.list.insert(index, todo);
   }

   fn complete_todo(&mut self, index: usize) {
      let old_todo = self.get_todo(index);
      let new_todo = Todo {
         completed: !old_todo.completed,
         ..old_todo
      };
      self.update_todo(index, new_todo);
   }

   fn delete_todo(&mut self, index: usize) {
      self.list.remove(index);
   }

   fn get_todo_count(&self) -> usize {
      return self.list.len();
   }

   fn load_from_file(&mut self, filename: &str) {
      create_file_if_not_exists(filename);
      let file = match File::open(filename) {
         Err(e) => panic!("{}", e),
         Ok(file) => file
      };

      // clear existing todo list
      self.list.clear();

      // read file line by line and create todos for each one
      let reader = BufReader::new(file);
      for line in reader.lines() {
         let unwrapped = line.unwrap();
         let todo_parts: Vec<&str> = unwrapped.split(":").collect();
         let todo = Todo {
            completed: match todo_parts[0] {
               "X" => true,
               _ => false
            },
            content: todo_parts[1].to_string()
         };
         self.add_todo(todo);
      }
   }

   fn save_to_file(&mut self, filename: &str) {
      let mut file = match File::create(filename) {
         Err(e) => panic!("{}", e),
         Ok(file) => file
      };
   
      for todo in &self.list {
         writeln!(file, "{}:{}", todo.completed_str(), todo.content).ok();
      }
   }
}

/**
 * Prints a character sequence that clears the terminal screen
 */
fn clear_screen() {
   //print!("{esc}c", esc = 27 as char)
}

/**
 * Creates a file with the given name if it doesn't exist
 */
fn create_file_if_not_exists(filename: &str) {
   if !Path::new(filename).exists() {
      let _ = match File::create(filename) {
         Err(e) => panic!("{}", e),
         Ok(file) => file
      };
   }
}

/**
 * Gets a line of input from the terminal
 */
fn get_input(prompt: &str) -> String {
   let mut input = String::new();
   println!("{}", prompt);
   stdin().read_line(&mut input).ok();
   return input.trim().to_string();
}

/**
 * Asks the user for an integer input representing a todo number (1-based index in the list)
 * @param message the text to display to the user
 * @return usize the chosen array index (input-1)
 * @panic if the input is not numeric
 * @panic if the input is not a valid index in the list
 */
fn get_todo_index_from_user(message: &str, max_index: usize) -> usize {
   // todo put this in a loop to keep requesting a number until we get a correct one
   // todo handle a case for the user to return to the menu
   let stringdex: String = get_input(message);
   // start index from 1
   let index: usize = stringdex.parse::<usize>().unwrap() - 1;

   // todo revisit this and see if we can provide a more helpful interaction
   if index >= max_index {
      panic!("Error, requested todo number not valid");
   }
   return index;
}

/**
 * Requests a new title from the user, amends the given todo and stores the updated version
 * @param index the array index to amend
 */
fn edit_todo(manager: &mut TodoManager, index: usize) {
   let old_todo: Todo = manager.get_todo(index);
   println!("Editing '{}'", old_todo.content);
   let new_content: String = get_input("Enter new content for this todo:");
   let new_todo = Todo {
      content: new_content,
      ..old_todo
   };
   manager.update_todo(index, new_todo);
}

fn print_todo_list(manager: &TodoManager) {
   let mut index = 1;
   for todo in &manager.list {
      let completed_str = todo.completed_str();
      println!("{:2}: [{}] >> {}", index, completed_str, todo.content);
      index += 1;
   }
}

fn print_command_list() {
   println!("- Commands:");
   println!(" q: quit");
   println!(" e: edit a todo");
   println!(" c: toggle completeness of a todo");
   println!(" d: delete a todo");
   println!(" n: create new todo");
   println!(" r: reload list of todos");
   println!(" s: save todo list to disk");
}

fn main() -> std::io::Result<()> {
   const TODO_FILENAME: &str = "todos.list";

   let mut manager = TodoManager::new();

   // load list of todos from disk
   manager.load_from_file(TODO_FILENAME);

   let mut input: String = String::new();
   while input != "q" {
      clear_screen();
      print_todo_list(&manager);
      print_command_list();
      input = get_input("Choose a command:");
      match input.as_str() {
         "c" => {
            let idx = get_todo_index_from_user("Mark which todo as complete?", manager.get_todo_count());
            manager.complete_todo(idx);
         },
         "d" => {
            let idx = get_todo_index_from_user("Delete which todo?", manager.get_todo_count());
            manager.delete_todo(idx);
         },
         "e" => {
            let idx = get_todo_index_from_user("Edit which todo?", manager.get_todo_count());
            edit_todo(&mut manager, idx);
         },
         "n" => {
            let todo_content: String = get_input("Enter the todo contents:");
            let todo: Todo = Todo {
               completed: false,
               content: todo_content
            };
            manager.add_todo(todo);
         },
         "s" => {
            manager.save_to_file(TODO_FILENAME);
         },
         "r" => {
            manager.load_from_file(TODO_FILENAME);
         },
         "q" => {},
         _ => {
            println!("Invalid command given, stop being silly!");
         }
      }
   }


   Ok(())
}
