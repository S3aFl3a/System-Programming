#[derive(Debug)]
// According to assignment, will make a public method
// new(initial_balance: f64) -> BankAccount to create a new account
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount {
            balance: if initial_balance >= 0.0 { initial_balance } else {0.0},
        }
    }

    // Public methods: 
    //  deposit(&mut self, amount: f64)
    // Should increase the balance. Ignore the operation if the amount is negative.
    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        // This focuses on adding money into the account so allowing postive deposits
        if amount > 0.0 {
            self.balance += amount;
        }
    }


    //  withdraw(&mut self, amount: f64)
    // Should decrease the balance. If the amount is greater than the balance or neagative,
    // the balance should remain unchanged.
    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        // Withdrawing means removing/taking money from the account
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    //  balance(&self) -> f64
    // Should return the current balance 
    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }

    // This is for the bonus part o
    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate; //Will increase balance by the interest rate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 1e-10;
    // According to the assignment, make tests based on this.
        // Creating a new account
        // Depositing money
        // Withdrawing money
        // Checking the balance
        // Edge cases(e.g., depositing/withdrawing negative amounts,
        // withdrawing more than the balance)


    // Test for Creating a new account
    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(100.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    // Test for Depositing money
    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0); //will deposit 

        assert!((account.balance() - 150.0).abs() < EPSILON);
    }

    // Test for Withdrawing money
    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(30.0); //will withdraw
        assert!((account.balance() - 70.0).abs() < EPSILON);

    }

    // Add more tests here

    // Test for Checking the balance
    #[test]
    fn test_check_balance() {
        let account = BankAccount::new(200.0);
        assert!((account.balance() - 200.0).abs() < EPSILON);
    }
    // Test for depositing negative amounts
    #[test]
    fn test_deposit_negative_amount() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);

        // Balance should remain unchanged
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }
    // Test for withdrawing negative amounts
    #[test]
    fn test_withdraw_negative_amount() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-20.0);
        // Balance should remain unchanged here
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }
    // Test for withdrawing more than the balance
    #[test]
    fn test_withdraw_more_than_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(150.0);

        // Balance should remain unchanged
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }
}

fn main() {
    let mut account = BankAccount::new(100.0);
    println!("Starting balance of: {}", account.balance());

    account.deposit(50.0);
    println!("After deposit: {}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawal: {}", account.balance());
}