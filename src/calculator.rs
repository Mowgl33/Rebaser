use std::io::stdin;

pub struct Calculator {
    decimal: i32,
    base: i32,
    result: Vec<i32>,
}

impl Calculator {
    /// Create a new calculator
    pub fn new() -> Calculator {
        Calculator {
            decimal: 0,
            base: 0,
            result: vec![],
        }
    }

    /// Start the calculator loop
    pub fn start(&mut self) {
        self.frontmatter();
        loop {
            self.inner_loop();
            if !self.repeat_requested() {
                break;
            }
        }
        self.farewell();
    }

    /// Frontmatter block
    fn frontmatter(&mut self) {
        println!("\n*****************************************************************");
        println!("Rebaser is pleased to make your acquaintance. Beep boop.");
        println!("Rebaser will take your uncouth decimal numbers and convert them \ninto respectable rebased numbers using a base of your choice. Beep boop beep.");
        println!("(As many times as you wish. Rebaser is tireless. Boop beep boop.)");
        println!("---")
    }

    /// Inner loop
    pub fn inner_loop(&mut self) {
        self.request_decimal();
        self.request_base();
        self.calculate_result();
        self.output_result();
    }

    /// Request decimal number
    fn request_decimal(&mut self) {
        println!("Please start by providing me with the decimal number (base-10) that you would like to convert.");
        println!("The number should be less than 10000.");

        let mut decimal = String::new();
        loop {
            decimal.clear();
            match stdin().read_line(&mut decimal) {
                Ok(_) => {
                    decimal = decimal.trim().to_string();
                    match decimal.parse::<i32>() {
                        Ok(decimal) => {
                            if decimal < 10000 {
                                self.decimal = decimal;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            println!("Try again...");
        }
    }

    /// Request base
    fn request_base(&mut self) {
        println!("Now please provide me with the base number.");
        println!("The number should be between 2 and 9.");

        let mut base = String::new();
        loop {
            base.clear();
            match stdin().read_line(&mut base) {
                Ok(_) => {
                    base = base.trim().to_string();
                    match base.parse::<i32>() {
                        Ok(base) => {
                            if base > 1 && base < 10 {
                                self.base = base;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            println!("Try again...");
        }
    }

    /// Calculate result
    fn calculate_result(&mut self) {
        self.result.clear();
        
        // Determine highest base power
        let mut power = 0;
        loop {
            if self.decimal / (self.base.pow(power as u32)) < 1 {
                power -= 1;
                break;
            }
            power += 1;
        }

        // Loop through powers in descending order to determine digits
        let mut decimal = self.decimal;
        for power in (0..=power).rev() {
            let digit = decimal / (self.base.pow(power as u32));
            self.result.push(digit);
            decimal = decimal - (digit * (self.base.pow(power as u32)));
        }
    }

    /// Output result
    fn output_result(&mut self) {
        let mut result = String::new();
        for digit in &self.result {
            result += &digit.to_string();
        }

        println!(
            "Decimal number [{}] is equivalent to [{}, base {}].",
            self.decimal, result, self.base
        );
        println!("---");
    }

    /// Ask if user wants to repeat loop
    fn repeat_requested(&mut self) -> bool {
        println!("Would you like to try again with another number? (y/n)");

        let mut repeat = false;
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
                if let Some('y') = input.chars().next() {
                    repeat = true;
                } else if let Some('Y') = input.chars().next() {
                    repeat = true;
                } else {
                    // no op
                }
            }
            _ => {}
        }
        println!("---");

        repeat
    }

    /// Print farewell
    fn farewell(&mut self) {
        println!("Thank you for using Rebaser. See you next time.");
        println!("*****************************************************************\n");
    }
}
