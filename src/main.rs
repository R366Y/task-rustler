use std::io::{self, Write};
use to_do::task_manager::{Priority, SortOrder, Task, TasksService};

fn main() {
    let tasks = TasksService::new(&"tasks.db".to_string());
    loop {
        println!("\n--- To-Do List Manager ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Change task priority");
        println!("5. Delete Task");
        println!("6. Quit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_task(&tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_completed(&tasks),
            "4" => change_task_priority(&tasks),
            "5" => delete_taks(&tasks),
            "6" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn add_task(tasks: &TasksService) {
    print!("Enter task description: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    tasks.add_task(description.trim().to_string());
    println!("Task added successfully!");
}

fn view_tasks(tasks: &TasksService) {
    if tasks.is_empty() {
        println!("No tasks in the list.");
    } else {
        let sorted_tasks = tasks.get_all_tasks_sorted(SortOrder::High);
        for task in sorted_tasks.iter().filter(|t| !t.completed) {
            println!();
            print_task(task);
        }
        for task in sorted_tasks.iter().filter(|t| t.completed) {
            println!();
            print_task(task);
        }
    }
}

fn print_task(task: &Task) {
    let status = if task.completed { "[X]" } else { "[ ]" };
    let priority = match task.priority {
        Priority::High => "!!! ",
        Priority::Medium => "!! ",
        Priority::Low => "! ",
    };

    println!("{} Task {}: {}{}", status, task.id, priority, task.description);
}

fn mark_completed(tasks: &TasksService) {
    view_tasks(tasks);
    if tasks.is_empty() {
        return;
    }

    print!("Enter the task number to mark as completed: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(task_id) = input.trim().parse::<i32>() {
        if tasks.mark_completed(task_id) > 0 {
            println!("Task marked as completed");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Invalid input. Please enter a number");
    }
}

fn change_task_priority(tasks: &TasksService){
    view_tasks(tasks);
    if tasks.is_empty() {
        return;
    }

    print!("Enter the task number to change priority: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(task_id) = input.trim().parse::<i32>() {
        if tasks.get_task(task_id).is_some()  {
            display_change_priority(tasks, task_id);
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Invalid input. Please enter a number");
    }
}

fn delete_taks(tasks: &TasksService) {
    view_tasks(tasks);
    if tasks.is_empty() {
        return;
    }

    print!("Enter the task number to delete: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(task_id) = input.trim().parse::<i32>() {
        if tasks.delete_task(task_id) > 0 {
            println!("Task deleted");
        } else {
            println!("Invalid task number");
        }
    } else {
        println!("Invalid input. Please enter a number");
    }
}

fn display_change_priority(tasks: &TasksService, task_id: i32) {
    print!("Enter the priority for the task (H)igh, (M)edium, (L)ow: ");
    io::stdout().flush().unwrap();
    let mut pri = String::new();
    io::stdin().read_line(&mut pri).unwrap();

    let _ = match pri.trim() {
        "H" => tasks.change_priortiy(task_id, Priority::High),
        "M" => tasks.change_priortiy(task_id, Priority::Medium),
        "L" => tasks.change_priortiy(task_id, Priority::Low),
        _ =>{
            println!("Invalid value for priority");
            0
        }
    };
}
