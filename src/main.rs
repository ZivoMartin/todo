use std::{env, collections::HashMap, fs::File, io::prelude::*};

static FILE_PATH: &str = "tasks";

fn usage(_vec: Vec<String>) {
    println!("Possible command: 
    add/a $task_name $task_content: add a new task
    del/d $task_name: delete an existing task
    list/l : list all the task
    reset/r : reset the task list
    help/h : print this message
    default: list")
}

fn default() {
    list(vec!())
}

fn add(vec: Vec<String>) {
    if !(vec.len() > 2) {
        return usage(vec);
    }
    let task_name = vec[2].clone();
    let task_content = if vec.len() == 3 {""}else{&vec[3]};
    let mut f = get_file();
    f.write((task_name+": " + task_content+"\n").as_bytes()).expect("Failed to write the task :/");
    println!("The task {} has been add successfully", vec[2]);
}

fn del(vec: Vec<String>) {
    if vec.len() != 3 {
        return usage(vec);
    }
    let mut tasks_string = String::new();
    let mut f = get_file();
    f.read_to_string(&mut tasks_string).expect("Failed to read the task list");
    let mut new_content = String::new();
    let mut task_to_delete = vec[2].clone();
    let mut deleted = false;
    task_to_delete.push(':');
    for t in tasks_string.lines() {
        if deleted || !t.starts_with(&task_to_delete) {
            new_content.push_str(t);
            new_content.push('\n');
        }else{
            deleted = true;
        }
    }
    if deleted {
        f.set_len(0).expect("The task exists but we failed to delete it.");
        f.write(new_content.as_bytes()).expect("The task exists but we failed to delete it.");
        println!("The task {} has been deleted successfully", vec[2]);
    }else{
        println!("The task {} doesn't exists.", vec[2]);
    }
}   

fn list(_vec: Vec<String>) {
    let mut tasks_string = String::new();
    get_file().read_to_string(&mut tasks_string).expect("Failed to read the task list");
    println!("{}", if tasks_string.is_empty() {"None task yet"}else{&tasks_string});
}

fn reset(_vec: Vec<String>) {
    let f = get_file();
    f.set_len(0).expect("Failed to reset.");
    println!("The task list has been reset successfully");  
}

fn build_map() -> HashMap<String, fn(Vec<String>)> {
    let mut res: HashMap<String, fn(Vec<String>)> = HashMap::new();
    res.insert("del".to_string(), del);
    res.insert("d".to_string(), del);
    res.insert("add".to_string(), add);
    res.insert("a".to_string(), add);
    res.insert("list".to_string(), list);
    res.insert("l".to_string(), list);
    res.insert("help".to_string(), usage);
    res.insert("l".to_string(), list);
    res.insert("reset".to_string(), reset);
    res.insert("r".to_string(), reset);
    res
}

fn get_file() -> File {
    match File::options().read(true).append(true).open(FILE_PATH) {
        Ok(res) => res,
        Err(_) => File::create(FILE_PATH).unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_map = build_map();
    if args.len() == 1 {
        default();
    }else{
        match task_map.get(&args[1]) {
            Some(f) => f(args),
            _ => {
                println!("The command {} doesnt exists.", args[1]);
                usage(args)
            }
        }
    }

}
