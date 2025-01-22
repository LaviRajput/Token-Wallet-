// A simple struct to manage token balances
pub struct TokenManager {
    balance: u64, // The current balance of tokens
}

impl TokenManager {
    // Constructor to create a new TokenManager with a balance of 0
    pub fn new() -> Self {
        TokenManager { balance: 0 }
    }

    // Method to send tokens, reducing the balance
    // Returns a Result indicating success or an error if the balance is insufficient
    pub fn send_tokens(&mut self, amount: u64) -> Result<(), String> {
        if self.balance < amount {
            Err(String::from("Insufficient balance")) // Error if not enough balance
        } else {
            self.balance -= amount; // Deduct the amount from the balance
            Ok(()) // Return success
        }
    }

    // Method to receive tokens, increasing the balance
    pub fn receive_tokens(&mut self, amount: u64) {
        self.balance += amount; // Add the amount to the balance
    }

    // Method to get the current balance
    pub fn get_balance(&self) -> u64 {
        self.balance // Return the balance
    }
}
