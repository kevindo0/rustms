#[macro_use]
extern crate log;

use project;

fn main() {
	project::project_init();
	debug!("this is a debug {}", "message");
	info!("world");
	error!("error");
}
