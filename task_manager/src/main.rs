use clap::{Arg, ArgAction, Command};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Your Name")
        .about("Displays and manages system processes")
        .arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .value_name("FILTER")
                .help("Filter processes by name or PID")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("sort")
                .short('s')
                .long("sort")
                .value_name("COLUMN")
                .help("Sort processes by column (pid, name, cpu, memory)")
                .action(ArgAction::Set),
        )
        .get_matches();

    let filter = matches.get_one::<String>("filter").map(|s| s.as_str());
    let sort_by = matches
        .get_one::<String>("sort")
        .map(|s| s.as_str())
        .unwrap_or("pid");

    let mut system = System::new_all();
    system.refresh_all();

    let mut processes: Vec<_> = system.processes().values().collect();

    if let Some(filter) = filter {
        let filter_lower = filter.to_lowercase();
        processes.retain(|process| {
            process.name().to_lowercase().contains(&filter_lower)
                || process.pid().to_string() == filter
        });
    }    

    match sort_by {
        "pid" => processes.sort_by_key(|p| p.pid()),
        "name" => processes.sort_by_key(|p| p.name().to_lowercase()),
        "cpu" => processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap()),
        "memory" => processes.sort_by(|a, b| b.memory().cmp(&a.memory())),
        _ => eprintln!("Invalid sort column. Sorting by PID."),
    }

    println!(
        "{:<10} {:<30} {:<10} {:<10}",
        "PID", "Name", "CPU%", "Memory"
    );
    for process in processes {
        println!(
            "{:<10} {:<30} {:<10.2} {:<10}",
            process.pid(),
            process.name(),
            process.cpu_usage(),
            process.memory()
        );
    }
}
