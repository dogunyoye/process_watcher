use std::process::{Command, Stdio};
use std::collections::HashSet;

struct ProcessSets {
    prev_set: HashSet<u32>,
    curr_set: HashSet<u32>,
}

pub trait ProcessWatcherCallback {
    fn on_open(&self, pid: u32) -> ();
    fn on_close(&self, pid: u32) -> ();
}

fn get_processes(callback: &ProcessWatcherCallback) -> () {
    let mut sets = ProcessSets { prev_set: HashSet::new(), curr_set: HashSet::new() };

    let child = Command::new("/bin/ps").arg("-e")
        .stdout(Stdio::piped()).spawn().expect("process spawn failed");

    let output = child.wait_with_output().expect("failed to wait on process");
    let vector = output.stdout.as_slice();
    let newline : u8 = 10;
    let iter = vector.split(|num| num == &newline);

    for line in iter {
        let str_line = String::from_utf8_lossy(line);
        let mut fields = str_line.split_whitespace();

        let field = fields.next();
        if field.is_some() {
            let pid: u32 = match field.unwrap().trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            sets.curr_set.insert(pid);
        }
    }

    //first run, the previous set will be empty
    if sets.prev_set.is_empty() {
        for x in sets.curr_set.iter() {
            callback.on_open(x.clone());
        }
    }
    else {
        let open_set: HashSet<u32> = sets.curr_set.difference(&sets.prev_set).cloned().collect();
        let closed_set: HashSet<u32> = sets.prev_set.difference(&sets.curr_set).cloned().collect();

        for opened in open_set.iter() {
            callback.on_open(opened.clone());
        }

        for closed in closed_set.iter() {
            callback.on_close(closed.clone());
        }
    }

    sets.prev_set = sets.curr_set.clone();
    sets.curr_set.clear();
}

pub fn watch(callback: &ProcessWatcherCallback) -> () {
	get_processes(callback); 
}
