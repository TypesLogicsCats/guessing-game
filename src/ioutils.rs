use std::io;
use std::io::Write;

pub enum Answer {
    Yes,
    No,
}

pub fn prompt(str: &str) -> String {
    print!("{}", str);
    io::stdout().flush().unwrap();
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).unwrap();
    // Remove the trailing newline
    ans.pop();
    ans
}

pub fn prompt_yn(str: &str) -> Answer {
    print!("{} [y/n] ", str);
    io::stdout().flush().unwrap();
    let mut ans = String::new();
    loop {
        io::stdin().read_line(&mut ans).unwrap();
        match ans.as_str() {
            "yes\n" | "Yes\n" | "y\n" => break Answer::Yes,
            "no\n" | "No\n" | "n\n" => break Answer::No,
            _ => {
                // Reuse the same string
                ans.clear();
                print!("Answer yes or no: ");
                io::stdout().flush().unwrap()
            }
        }
    }
}
