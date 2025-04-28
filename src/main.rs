use std::time::Duration;


mod shell;
mod init;

pub(crate) const TAGLINE: &str = "Welcome to Menhera OS 0.1.0b-dev !";

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

    std::thread::spawn(|| {
        loop {
            unsafe { libc::wait(std::ptr::null_mut()) };
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    println!("We've just booted!");
    println!("{}", TAGLINE);

    shell::search_serial();

    shell::shell();

    println!("Shutting down...");
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    shutdown();
}
