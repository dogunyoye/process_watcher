use std::process::{Command, Stdio};
use std::collections::HashSet;
use std::thread;

struct ProcessSets {
    prev_set: HashSet<Process>,
    curr_set: HashSet<Process>,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Process {
    pub pid: u32,
    pub description: String,
}

pub trait ProcessWatcherCallback : Send {
    fn on_open(&self, process: Process) -> ();
    fn on_close(&self, process: Process) -> ();
}

fn get_processes(sets: &mut ProcessSets) -> (HashSet<Process>, HashSet<Process>) {

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

            fields.next();
            fields.next();
            let desc : String = fields.next().unwrap().to_owned();

            //filtering processes opened by calling "/bin/ps"
            if desc.ne("/bin/ps") {
                sets.curr_set.insert(Process { pid: pid, description: desc });
            }
        }
    }

    let open_set;
    let mut closed_set: HashSet<Process> = HashSet::new();

    //first run, the previous set will be empty
    if sets.prev_set.is_empty() {
        open_set = sets.curr_set.iter().cloned().collect();
    }
    else {
        open_set = sets.curr_set.difference(&sets.prev_set).cloned().collect();
        closed_set = sets.prev_set.difference(&sets.curr_set).cloned().collect();
    }

    sets.prev_set = sets.curr_set.clone();
    sets.curr_set.clear();

    return (open_set, closed_set);
}

pub fn watch_with_callback<TCallback : 'static + ProcessWatcherCallback>(callback: TCallback) -> () {
    let mut sets = ProcessSets { prev_set: HashSet::new(), curr_set: HashSet::new() };

    thread::spawn( move || {
        
        loop {
            let changed_sets = get_processes(&mut sets);    

            for open in changed_sets.0.iter() {
                callback.on_open(open.clone());
            }

            for close in changed_sets.1.iter() {
                callback.on_close(close.clone());
            }
        }
    });
}

pub fn watch_with_closure(on_open: &Fn(Process), on_close: &Fn(Process)) {
    let mut sets = ProcessSets { prev_set: HashSet::new(), curr_set: HashSet::new() };
    let changed_sets = get_processes(&mut sets);

    for open in changed_sets.0.iter() {
        on_open(open.clone());
    }

    for close in changed_sets.1.iter() {
        on_close(close.clone());
    }
}