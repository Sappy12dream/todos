use std::io;

struct Todo {
    tasks: Vec<String>,
}
impl Todo {
    fn new() -> Todo {
        Todo { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    fn mark_complete(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks[index] = format!("✔︎ {}", self.tasks[index]);
        }
    }

    fn delete_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        }
    }
    fn print_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
    }
}

fn main() {
    println!("Welcome to the Todo List App!");
    let mut todo = Todo::new();
    loop {
        println!("Todo List: 📝");
        todo.print_tasks();
        println!("Menu: 📋");
        println!("1. Add Task ➕");
        println!("2. Mark Task as Complete ✅");
        println!("3. Delete Task ❌");
        println!("4. Exit 🚪");
        println!("Enter your choice: 🤔");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read input");
        match choice.trim().parse() {
            Ok(1) => {
                println!("Enter task:");
                let mut task = String::new();
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read input");
                todo.add_task(task.trim().to_string());
            }
            Ok(2) => {
                println!("Enter task number to mark as complete:");
                let mut index_str = String::new();
                io::stdin()
                    .read_line(&mut index_str)
                    .expect("Failed to read input");
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    todo.mark_complete(index - 1);
                }
            }
            Ok(3) => {
                println!("Enter task number to delete:");
                let mut index_str = String::new();
                io::stdin()
                    .read_line(&mut index_str)
                    .expect("Failed to read input");
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    todo.delete_task(index - 1);
                }
            }
            Ok(4) => break,
            _ => println!("Invalid input, please try again."),
        }
    }
}
