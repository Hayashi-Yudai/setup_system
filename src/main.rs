use colored::control;
use colored::Colorize;
use regex::Regex;
use std::io;
use std::process::{Command, Stdio};

struct CondaEnvStatus {
    is_exist: bool,
    path: Option<String>,
}

fn check_conda() -> Result<(), String> {
    let mut conda_exist = false;
    match Command::new("conda").arg("--version").output() {
        Ok(_) => {
            println!("Check conda is installed correctly ... {}", "OK".green());
            conda_exist = true;
        }
        Err(_) => println!("Check conda is installed correctly ... {}", "NG".red()),
    }

    if !conda_exist {
        return Err("conda is not found. Install Aanaconda or Miniconda".to_string());
    }

    Ok(())
}

fn environment_exist() -> Result<CondaEnvStatus, String> {
    let mut exist = false;
    let mut env_path = None;
    match Command::new("conda").arg("env").arg("list").output() {
        Ok(output) => {
            let pattern = Regex::new(r"^thz\s+(.+)").expect("Invalid pattern");
            let pathes = String::from_utf8(output.stdout).unwrap();
            let lines = pathes.lines();

            for line in lines {
                let cap = pattern.captures(line);
                match cap {
                    Some(cap) => {
                        exist = true;
                        env_path = Some(cap[1].to_string());
                        println!("Environment exist in: {}", &cap[1]);
                        break;
                    }
                    None => continue,
                }
            }

            if !exist {
                println!("Vurtural environment for the application does not exist");
            }
        }
        Err(msg) => {
            return Err(msg.to_string());
        }
    }

    Ok(CondaEnvStatus {
        is_exist: exist,
        path: env_path,
    })
}

fn create_environment(env_status: CondaEnvStatus) -> Result<(), String> {
    loop {
        let mut ans = String::new();
        if env_status.is_exist {
            println!("Do you create environment again? (y/N):");
        } else {
            println!("Do you create environment? (y/N):");
        }

        io::stdin().read_line(&mut ans).unwrap();

        match &*ans.trim() {
            "y" => {
                match env_status.path {
                    Some(path) => {
                        println!("Deleting old environment...{}", path);
                        Command::new("conda")
                            .arg("remove")
                            .arg("-n")
                            .arg("thz")
                            .arg("--all")
                            .output()
                            .expect("Falied to delete old environment");
                        println!("Finished");
                    }
                    None => {}
                }
                println!("Creating new environment...");
                let create_env_status = Command::new("conda")
                    .arg("create")
                    .arg("-n")
                    .arg("thz")
                    .arg("-y")
                    .arg("python=3.9.7")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output();
                match create_env_status {
                    Ok(_) => {}
                    Err(_) => {
                        let msg = format!("{}", "Failed to create new environment".red());
                        return Err(msg);
                    }
                }
                println!("Finished");
                break;
            }
            "N" | "n" => {
                return Ok(());
            }
            _ => {
                println!("{}", "Enter y(es) or N(o)".red());
                continue;
            }
        }
    }
    Ok(())
}

fn install_packages() -> Result<(), String> {
    let mut ans = String::new();
    loop {
        println!("Install required packages? (y/N)");
        io::stdin().read_line(&mut ans).unwrap();

        match *&ans.trim() {
            "y" => break,
            "N" | "n" => return Ok(()),
            _ => {
                println!("{}", "Enter y(es) or N(o)".red());
            }
        }
    }

    let install_status = Command::new("activate.bat")
        .args(&["&", "pip", "install", "-r", "requirements.txt"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();
    match install_status {
        Ok(_) => {}
        Err(_) => {
            let msg = format!("{}", "Failed to install required packages".red());

            return Err(msg);
        }
    }
    println!("Finished");
    Ok(())
}

fn check_visa() -> Result<(), String> {
    let pattern = Regex::new(r"visa(32|64).dll").unwrap();
    let result = Command::new("activate.bat")
        .args(&["&", "python", "gpib_check.py"])
        .output();
    match result {
        Ok(ok) => {
            let err = String::from_utf8(ok.stderr).unwrap();
            let output = String::from_utf8(ok.stdout).unwrap();

            if err == "" && pattern.is_match(&output) {
                println!("NI-VISA status ... {}", "OK".green());
            } else {
                println!("NI-VISA status ... {}", "NG".red());
                return Err("NI-VISA is not found".to_string());
            }
        }
        Err(_) => {
            println!("NI-VISA status ... {}", "NG".red());
            return Err("NI-VISA is not found".to_string());
        }
    }
    Ok(())
}

fn wait_something() {
    println!("Enter something to close this window");
    let mut something = String::new();
    io::stdin().read_line(&mut something).unwrap();
}

fn main() {
    if cfg!(target_os = "windows") {
        control::set_virtual_terminal(true).unwrap();
    }

    println!("Start setup process. Need Internet access.");

    match check_conda() {
        Ok(_) => {}
        Err(msg) => {
            println!("{}\n{}", msg, "Setup Failed".red());
            wait_something();
            return;
        }
    }
    match environment_exist() {
        Ok(env_status) => match create_environment(env_status) {
            Ok(_) => {}
            Err(msg) => {
                println!("{}\n{}", msg, "Setup Failed".red());
                wait_something();
                return;
            }
        },
        Err(msg) => {
            println!("{}\n{}", msg, "Setup Failed".red());
            wait_something();
            return;
        }
    }
    match install_packages() {
        Ok(_) => {}
        Err(msg) => {
            println!("{}\n{}", msg, "Setup Failed".red());
            wait_something();
            return;
        }
    }
    match check_visa() {
        Ok(_) => {}
        Err(msg) => {
            println!("{}\n{}", msg, "Setup Failed".red());
            wait_something();
            return;
        }
    }
    println!("{}", "Finished all setup process successfully".green());

    wait_something();
}
