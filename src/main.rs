use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

enum TodoStatus {
    Pending,
    Completed,
    Cancelled,
}

struct TodoItem {
    description: String,
    status: TodoStatus,
}

impl TodoItem {
    fn new(description: String) -> TodoItem {
        TodoItem {
            description: description,
            status: TodoStatus::Pending,
        }
    }

    fn from_string(string: String) -> TodoItem {
        let parts: Vec<&str> = string.split('\t').collect();
        if parts.len() != 2 {
            panic!("无效的 Todo 项：{}", string);
        }

        let description = parts[0].to_owned();
        let status = match parts[1] {
            "pending" => TodoStatus::Pending,
            "completed" => TodoStatus::Completed,
            "cancelled" => TodoStatus::Cancelled,
            _ => panic!("无效的 Todo 状态：{}", parts[1]),
        };

        TodoItem {
            description: description,
            status: status,
        }
    }

    fn to_string(&self) -> String {
        let status_string = match self.status {
            TodoStatus::Pending => "pending",
            TodoStatus::Completed => "completed",
            TodoStatus::Cancelled => "cancelled",
        };

        format!("{}\t{}", self.description, status_string)
    }
}

fn read_todos() -> Vec<TodoItem> {
    let path = Path::new("todos.txt");
    let mut todos = vec![];

    // 检查 Todo 文件是否存在，如果不存在则创建
    if !path.exists() {
        File::create(path).expect("无法创建文件");
    }

    // 读取 Todo 文件中的内容
    let file = File::open(&path).expect("无法打开文件");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let todo = TodoItem::from_string(line.expect("无法读取行"));
        todos.push(todo);
    }

    todos
}

fn write_todos(todos: &[TodoItem]) {
    let path = Path::new("todos.txt");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("无法打开文件");

    for todo in todos {
        writeln!(file, "{}", todo.to_string()).expect("无法写入行");
    }
}

// add

fn add_todo_item(todos: &mut Vec<TodoItem>) {
    println!("请输入您要添加的 Todo：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("无法读取输入");

    let input = input.trim();
    if input.is_empty() {
        return;
    }

    let todo = TodoItem::new(String::from(input));
    todos.push(todo);

    println!("添加成功！");
}

fn cancel_todo_item(todos: &mut Vec<TodoItem>) {
    println!("请输入要取消的 Todo 编号：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("无法读取输入");

    let input = input.trim();
    if input.is_empty() {
        return;
    }

    let index: usize = input.parse().expect("无效的编号");
    if index < 1 || index > todos.len() {
        let todo: &mut TodoItem = todos.get_mut(index - 1).expect("无效的编号");
        todo.status = TodoStatus::Cancelled;

        println!("取消成功！");
    }
}

fn complete_todo_item(todos: &mut Vec<TodoItem>) {
    println!("请输入要完成的 Todo 编号：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("无法读取输入");

    let input = input.trim();
    if input.is_empty() {
        return;
    }

    let index: usize = input.parse().expect("无效的编号");
    if index < 1 || index > todos.len() {
        panic!("无效的编号：{}", index);
    }

    let todo = todos.get_mut(index - 1).expect("无效的编号");
    todo.status = TodoStatus::Completed;

    println!("完成成功！");
}

fn print_todos(todos: &[TodoItem]) {
    if todos.is_empty() {
        println!("当前没有待办事项！");
        return;
    }

    println!("当前待办事项：");
    for (i, todo) in todos.iter().enumerate() {
        let status_string = match todo.status {
            TodoStatus::Pending => "",
            TodoStatus::Completed => "√",
            TodoStatus::Cancelled => "X",
        };

        println!("{} {}. {}", i + 1, status_string, todo.description);
    }
}

fn main() {
    let mut todos = read_todos();

    loop {
        println!("请输入要执行的操作（add/cancel/complete/print/quit）：");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("无法读取输入");

        let input = input.trim().to_lowercase();
        match input.as_str() {
            "add" => add_todo_item(&mut todos),
            "cancel" => cancel_todo_item(&mut todos),
            "complete" => complete_todo_item(&mut todos),
            "print" => print_todos(&todos),
            "quit" => break,
            _ => println!("无效的操作：{}", input),
        }

        write_todos(&todos);
    }
}

