use procfs::process::all_processes;
use std::process;

use super::Command;
use super::Cmd;
pub struct Jobs;

impl Command for Jobs {
    fn name(&self) -> &'static str {
        "jobs"
    }
    fn run(&self, _cmd: &mut Cmd) {
        let my_pid = process::id() as i32;

        println!("My PID: {}", my_pid);
        println!("Child processes:");

        for proc_res in all_processes().unwrap() {
            if let Ok(proc) = proc_res {
                if let Ok(stat) = proc.stat() && stat.ppid == my_pid {
                    println!("PID: {}, state: {}", proc.pid, stat.state);
                }
            }
        }
    }
}
