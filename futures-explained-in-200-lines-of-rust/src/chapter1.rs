#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_use_os_threads() {
        println!("So we start the program here!");

        let thread_1 = thread::spawn(move || {
            thread::sleep(Duration::from_millis(200));
            println!("We create tasks which gets run when they're finished!");
        });

        let thread_2 = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            println!("We can even chain callbacks ... ");
            let thread_3 = thread::spawn(move || {
                thread::sleep(Duration::from_millis(50));
                println!("..like this!");
            });
            thread_3.join().unwrap();
        });

        println!("While our tasks are executing we can do other stuff here.");

        thread_1.join().unwrap();
        thread_2.join().unwrap();
    }
}
