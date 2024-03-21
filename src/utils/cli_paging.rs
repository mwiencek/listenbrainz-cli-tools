use inquire::{InquireError, Select};

#[derive(Debug)]
pub struct CLIPager {
    count: i32,
    max_count: i32
}

impl CLIPager {
    pub fn new(max_count: i32) -> Self {
        Self {
            count: 0,
            max_count
        }
    }

    pub fn execute(&mut self, f: fn() -> bool) {
        f();

        self.count += 1;

        if self.count == self.max_count {
            if Self::ask_continue() {
                self.count = 0;
            } else {
                return;
            }
        }
    }

    fn ask_continue() -> bool {
        loop {
            let options = vec!["Next", "Exit"];

            let ans: Result<&str, InquireError> =
                Select::new("What's your favorite fruit?", options).prompt();

            match ans {
                Ok(choice) => {
                    if choice == "Next" {
                        return true;
                    } else {
                        return false;
                    }
                }
                _ => println!("There was an error, please try again"),
            }
        }
    }
}
