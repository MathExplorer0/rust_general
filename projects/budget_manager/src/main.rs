use std::io;

#[derive(Debug)]
struct Budget {
    income: f32,
    expenses: f32,
}

impl Budget {
    fn new() -> Self {
        Self {
            income: 0.0,
            expenses: 0.0,
        }
    }

    fn add_income(&mut self, amount: f32) {
        self.income += amount;
        println!("Income increaced by: {}", amount);
    }

    fn add_expenses(&mut self, amount: f32) {
        self.expenses += amount;
        println!("Expenses increaced by: {}", amount);
    }

    fn remove_income(&mut self, amount: f32) {
        self.income += amount;
        println!("Income decreaced by: {}", amount);
    }

    fn remove_expenses(&mut self, amount: f32) {
        self.expenses -= amount;
        println!("Expenses decreaced by: {}", amount);
    }

    fn data(&mut self) {
        println!("{:?}", self);
    }
}

fn main() {
    let mut b1 = Budget::new();
    run(&mut b1);
}

fn run(budget: &mut Budget) {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim().parse::<usize>().unwrap();

        match command {
            0 => break,
            1 => budget.add_income(get_amount()),
            2 => budget.add_expenses(get_amount()),
            3 => budget.remove_income(get_amount()),
            4 => budget.remove_expenses(get_amount()),
            5 => budget.data(),
            _ => println!("Unknown command !"),
        }
    }
}

fn get_amount() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f32>().unwrap()
}
