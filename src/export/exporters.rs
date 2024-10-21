use icalendar::{Calendar, Component, EventLike, Todo, TodoStatus};
use crate::task::Task;

pub fn export_tasks_to_icalendar(calendar_name: &'static str, tasks: &[Task]) -> Calendar {
    let mut calendar = Calendar::new();
    calendar.name(calendar_name);
    for task in tasks {
        calendar.push(
            build_icalendar_todo_task(task)
        );
    }

    calendar.done()
}

fn build_icalendar_todo_task(task: &Task) -> Todo {
    let mut todo = Todo::new();
    if let Some(date) = task.date.0{
        todo.all_day(date);
    };
    todo.summary(task.title.as_str());
    todo.description(task.description.as_str());
    if task.completed {
        todo.status(TodoStatus::Completed);
    } else {
        todo.status(TodoStatus::NeedsAction);
    }
    todo.priority(task.priority.to_u8() as u32);
    todo.done()
}