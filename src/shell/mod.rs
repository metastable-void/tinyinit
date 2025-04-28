use std::{ffi::c_char, os::unix::fs::{FileTypeExt, MetadataExt}, path::Path, time::Duration};


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

        rl.add_history_entry(&line).ok();

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
        let modname = modline.split(" ").next().unwrap().trim();
        if modname.is_empty() {
            break;
        }

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

pub(crate) fn sh_serial<P: AsRef<Path>>(tty: P) {
    let tty = tty.as_ref();
    if let Err(_) = std::fs::write(tty, "Initializing console...\n") {
        return;
    }

    std::fs::write(tty, format!("{}\n", crate::TAGLINE)).ok();

    println!("Enabling tty: {:?}", tty);

    let pid = unsafe { libc::fork() };
    if pid < 0 {
        println!("fork() failed");
        return;
    }

    if pid > 0 {
        // parent
        return;
    }

    // safety: no other thread is running in child
    unsafe { std::env::set_var("TERM", "vt100") };

    unsafe { libc::setsid() };

    // this becomes the controlling terminal
    let tty_read = unsafe { libc::open(
        tty.as_os_str().as_encoded_bytes()
            .as_ptr() as *const _ as *const c_char,
            libc::O_RDONLY,
    ) };

    if tty_read < 0 {
        unsafe { libc::exit(0) };
    }

    let tty_write = unsafe { libc::open(
        tty.as_os_str().as_encoded_bytes()
            .as_ptr() as *const _ as *const c_char,
            libc::O_WRONLY,
    ) };

    if tty_write < 0 {
        unsafe { libc::exit(0) };
    }

    if 0 > unsafe { libc::dup2(tty_read, 0) } {
        unsafe { libc::exit(0) };
    }

    if 0 > unsafe { libc::dup2(tty_write, 1) } {
        unsafe { libc::exit(0) };
    }

    if 0 > unsafe { libc::dup2(tty_write, 2) } {
        unsafe { libc::exit(0) };
    }

    let mut fd = tty_write;
    while fd > 2 {
        unsafe { libc::close(fd) };
        fd -= 1;
    }

    unsafe { libc::execv(
        b"/bin/sh\0".as_ptr() as *const _ as *const c_char,
        [b"-sh\0".as_ptr() as *const _ as *const c_char, std::ptr::null()].as_ptr(),
    ) };

    unsafe { libc::exit(0) };
}

pub(crate) fn search_serial() {
    let dir = std::fs::read_dir("/dev");
    let dir = match dir {
        Err(e) => {
            println!("Failed to open /dev ({})", e);
            return;
        },

        Ok(d) => d,
    };

    for ent in dir {
        let ent = match ent {
            Err(_) => continue,
            Ok(e) => e,
        };

        let ft = match ent.file_type() {
            Err(_) => continue,
            Ok(ft) => ft,
        };

        if !ft.is_char_device() {
            continue;
        }

        let filename = ent.file_name();
        let filename = filename.to_str();
        if filename.is_none() {
            continue;
        }

        let filename = filename.unwrap();
        if !filename.starts_with("ttyS") {
            continue;
        }

        sh_serial(format!("/dev/{}", filename));
    }
}
