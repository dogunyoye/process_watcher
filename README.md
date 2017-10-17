## Usage
Add the following crate import
```extern crate process_watcher;```

Also include the following trait to be implemented
```use process_watcher::ProcessWatcherCallback;```

## Example
```
extern crate process_watcher;

use process_watcher::ProcessWatcherCallback;
use process_watcher::Process;

struct PWCallback { }

impl ProcessWatcherCallback for PWCallback {
	fn on_open(&self, process: Process) -> () {
		println!("OnOpen - pid: {}, desc: {}", process.pid, process.description);
	}

	fn on_close(&self, process: Process) -> () {
		println!("OnClose - pid: {}, desc: {}", process.pid, process.description);
	}
}

fn main() {

	let callback = PWCallback { };
	process_watcher::watch_with_callback(&callback);
}
```

## License

See [LICENSE](LICENSE) file.
