use super::Command;
use super::Cmd;
pub struct Jobs;

#[derive(Clone)]
pub struct Job {
    pid: i32,
    name: String,
    state: String,
}

use std::sync::Mutex;
use std::sync::LazyLock;
static JOBS: LazyLock<Mutex<Vec<Job>>> = LazyLock::new(|| Vec::<Job>::new().into());

pub fn add_job(pid: i32, name: &str, state: String) {
    JOBS.lock().unwrap().push(Job { pid, name: name.to_string(), state });
}

impl Command for Jobs {
    fn name(&self) -> &'static str {
        "jobs"
    }
    fn run(&self, cmd: &mut Cmd) {
        let args_count = cmd.args.len();
        for job in (*JOBS.lock().unwrap()).iter() {
            if args_count > 1 {
                for arg in &cmd.args {
                    if job.name.starts_with(arg) {
                        println!("[{}]: {} {}", job.pid, job.state, job.name);
                    }
                }
            } else {
                println!("[{}]: {} {}", job.pid, job.state, job.name);
            }
        }
    }
}
