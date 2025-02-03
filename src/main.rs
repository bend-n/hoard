use std::{
    env::args,
    thread::sleep,
    time::{Duration, SystemTime},
};
fn main() {
    if matches!(
        args().nth(1).as_deref(),
        Some("--help" | "-h" | "help" | "h")
    ) {
        println!("{} [OUTFILE] (KEY | env[AIR_KEY])", args().next().unwrap());
        std::process::exit(0);
    }
    let Some(output) = args().nth(1) else {
        println!("\x1b[31;1mno output file provided!\x1b[0m");
        std::process::exit(0);
    };
    let Some(key) = args().nth(2).or(std::env::var("AIR_KEY").ok()) else {
        println!("\x1b[31;1mno key provided!\x1b[0m");
        println!("get one at https://dashboard.iqair.com/personal/api-keys for free");
        println!("then set AIR_KEY or pass it to this binary");
        std::process::exit(0);
    };

    loop {
        let mut result = iqair::nearest(&key);
        while result.is_err() {
            result = iqair::nearest(&key);
            sleep(Duration::from_secs(5 * 60));
        }
        let result = result.unwrap();
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&output)
            .expect("file accessible");
        let json = serde_json::to_string(&result.current).unwrap();
        use std::io::Write;
        writeln!(f, "{json}").unwrap();
        f.flush().unwrap();
        drop(f);

        let unix = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // see you next hour
        let next = 65 * 60 - (unix % (60 * 60));
        println!("sleeping {} minutes...", next / 60);
        sleep(Duration::from_secs(next));
    }
}
