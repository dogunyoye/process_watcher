# Usage
Add the following crate import at the top of your ```main.rs```
```extern crate process_watcher;```

Also include the following trait to be implemented
```use process_watcher::ProcessWatcherCallback;```

# Example
```
extern crate process_watcher;

use process_watcher::ProcessWatcherCallback;

struct PWCallback { }

impl ProcessWatcherCallback for PWCallback {
	fn on_open(&self, pid: u32) -> () {
		println!("OnOpen: {}", pid);
	}

	fn on_close(&self, pid: u32) -> () {
		println!("OnClose: {}", pid);
	}
}

fn main() {

	let callback = PWCallback { };
	process_watcher::watch(&callback);
}
```
