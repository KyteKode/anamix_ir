use crate::block::Thread;

#[derive(Debug)]
pub struct Broadcast {
    id: String,
}

#[derive(Debug)]
pub struct Variable {
    id: String,
    name: String,
    value: String,
}

#[derive(Debug)]
pub struct List {
    id: String,
    name: String,
    value: Vec<String>,
}

#[derive(Debug)]
pub struct Project {
    threads: Vec<Thread>,
    broadcasts: Vec<Broadcast>,
    vars: Vec<Variable>,
    lists: Vec<List>,

    broadcast_counter: u64,
    var_counter: u64,
    list_counter: u64,
}

impl Project {
    pub fn add_thread(&mut self, thread: Thread) {
        self.threads.push(thread);
    }

    pub fn get_threads(&self) -> &Vec<Thread> {
        &self.threads
    }

    pub fn new_broadcast(&mut self) -> &Broadcast {
        let id_str = format!("broadcast_{:#016x}", self.broadcast_counter);
        self.broadcasts.push(Broadcast { id: id_str });
        self.broadcast_counter += 1;

        self.broadcasts
            .last()
            .expect("Vector should not be empty after push")
    }

    pub fn new_var(&mut self, name: String, value: String) -> &Variable {
        let id_str = format!("var_{:#016x}", self.broadcast_counter);
        self.vars.push(Variable { id: id_str, name, value });
        self.var_counter += 1;

        self.vars
            .last()
            .expect("Vector should not be empty after push")
    }

    pub fn new_list(&mut self, name: String, value: Vec<String>) -> &List {
        let id_str = format!("list_{:#016x}", self.broadcast_counter);
        self.lists.push(List { id: id_str, name, value });
        self.list_counter += 1;

        self.lists
            .last()
            .expect("Vector should not be empty after push")
    }
}
