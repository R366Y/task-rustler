use std::io::{self, Write};
use to_do::task_manager::Tasks;

fn main() {
    let mut tasks = Tasks::new();
    loop {
        println!("\n--- To-Do List Manager ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Delete Task");
        println!("5. Quit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_completed(&mut tasks),
            "4" => delete_taks(&mut tasks),
            "5" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }

}

fn add_task(tasks: &mut Tasks) {
    print!("Enter task description: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    tasks.add_task(description.trim().to_string());
    println!("Task added successfully!");
}

fn view_tasks(tasks: &Tasks) {
    if tasks.is_empty() {
        println!("No tasks in the list.");
    } else {
        for task in tasks.get_all_tasks() {
            println!(
                "{}. [{}] {}",
                task.id,
                if task.completed { "x" } else { " " },
                task.description
            );
        }
    }
}

fn mark_completed(tasks: &mut Tasks) {
    view_tasks(tasks);
    if tasks.is_empty(){
        return;
    }

    print!("Enter the task number to mark as completed: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(task_id) = input.trim().parse::<i32>() {
        if tasks.mark_completed(task_id) > 0 {
            println!("Task marked as completed");
        }
        else {
            println!("Invalid task number!");
        }
    } else {
        println!("Invalid input. Please enter a number");
    }
}

fn delete_taks(tasks: &mut Tasks) {
    view_tasks(tasks);
    if tasks.is_empty(){
        return;
    }

    print!("Enter the task number to delete: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(task_id) = input.trim().parse::<i32>() {
        if tasks.delete_task(task_id) > 0{
            println!("Task deleted");
        }
        else {
            println!("Invalid task number");
        }
    } else {
        println!("Invalid input. Please enter a number");
    }
}
