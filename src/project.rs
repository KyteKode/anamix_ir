use crate::block::Thread;

#[derive(Debug)]
pub struct Broadcast {
    id: String,
}

#[derive(Debug)]
pub struct Project {
    threads: Vec<Thread>,
    broadcasts: Vec<Broadcast>,

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
}
