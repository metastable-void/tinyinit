use std::{os::unix::fs::MetadataExt, time::Duration};


pub(crate) fn shell() {
    let mut rl = match rustyline::DefaultEditor::new() {
        Err(e) => {
            println!("ReadlineError: {:?}", e);
            return;
        },

        Ok(r) => r,
    };

    loop {
        let line = rl.readline("# ");
        let line = match line {
            Err(_) => {
                continue;
            },

            Ok(l) => l,
        };

        let parts = line.split(" ").map(|s| s.trim()).collect::<Vec<_>>();

        match parts[0] {
            "" => continue,
            
            "lsmod" => lsmod(),

            "sh" => sh(),

            "ls" => {
                if parts.len() < 2 {
                    ls("/");
                } else {
                    ls(parts[1]);
                }
            },

            "exit" | "quit" | "shutdown" | "halt" => break,

            _ => {
                println!("help:");
                println!("  sh | lsmod | exit | quit | shutdown | halt");
                println!("  ls <path>");
                println!("  help | ?");
            }
        }
    }
}

fn lsmod() {
    let mods = std::fs::read_to_string("/proc/modules");
    let mods = match mods {
        Err(_) => {
            println!("lsmod: failed");
            return;
        },

        Ok(m) => m,
    };

    let mut i = 0usize;
    for modline in mods.split("\n") {
        let modname = modline.split(" ").next().unwrap();
        println!("  {}", modname);
        i += 1;
        if i % 10 == 0 {
            std::thread::sleep(Duration::from_secs(3));
        }
    }

    println!("\n{} modules", i);
}

fn statfile(file: &str) {
    let meta = match std::fs::metadata(file) {
        Err(_) => return,
        Ok(s) => s,
    };

    let d = if meta.is_dir() { "d" } else { "-" };
    println!(
        "{}--------- {}\t{}",
        d,
        meta.size(),
        std::fs::canonicalize(file).unwrap_or("<invalid path>".into()).to_string_lossy(),
    );
}

fn ls(dir: &str) {
    if let Ok(d) = std::fs::read_dir(dir) {
        for ent in d {
            let ent = match ent {
                Err(e) => {
                    println!("ReadDir error: {:?}", e);
                    continue;
                },
                Ok(e) => e,
            };
            let path = ent.path();
            let path = match path.to_str() {
                None => continue,
                Some(p) => p,
            };
            statfile(path);
        }
    }
}

fn sh() {
    let mut cmd = std::process::Command::new("/bin/sh");
    match cmd.status() {
        Err(e) => {
            println!("Failed to invoke sh: {:?}", e);
        }
        Ok(_) => {},
    }
}
