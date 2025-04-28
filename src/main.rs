
mod shell;
mod init;

fn shutdown() -> ! {
    unsafe { libc::sync() };
    unsafe { libc::reboot(libc::RB_POWER_OFF) };
    loop {}
}

fn main() -> ! {
    println!("Loading...");

    if let Err(_) = init::prepare_fs() {
        println!("Critical error during init :/");
        //shutdown();
    }

    println!("We've just booted!");
    println!("Welcome to Menhera OS 0.1.0b-dev !");

    shell::shell();

    println!("Shutting down...");
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    shutdown();
}
