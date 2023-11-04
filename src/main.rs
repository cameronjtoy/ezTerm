use std::process::Command;

let mux echo_command = Command::new("tmux")
    .arg("new-window")
    .arg("-n")
    .arg("echo")
    .arg(echo_command)
    .spawn()
    .expect("failed to execute process");


