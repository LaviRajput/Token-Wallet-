# ICP Token Wallet

## Overview
ICP Token Wallet is a Rust-based application for managing token transactions. It provides functionalities to send, receive, and check the balance of tokens. The application is built using the Actix-web framework and interacts with a MySQL database to persist transaction data.

## Features
- **Send Tokens**: Transfer tokens to a specified address.
- **Receive Tokens**: Add tokens to the wallet balance.
- **Check Balance**: Retrieve the current token balance.

## Technologies Used
- **Rust**: The programming language used to build the application.
- **Actix-web**: A powerful, pragmatic, and extremely fast web framework for Rust.
- **MySQL**: The relational database used for storing transaction data.
- **dotenv**: For loading environment variables from a `.env` file.
- **Serde**: For serializing and deserializing data structures.

## Setup and Installation

### Prerequisites
- Rust installed on your machine. You can download it from [rust-lang.org](https://www.rust-lang.org/).
- MySQL installed and running.

### Steps
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd icp_token_wallet
   ```
2. Create a `.env` file in the root directory with your database connection details:
   ```env
   DATABASE_URL=mysql://username:password@localhost/dbname
   ```
3. Install the necessary Rust dependencies:
   ```bash
   cargo build
   ```
4. Run the application:
   ```bash
   cargo run
   ```
5. The application will be accessible at `http://127.0.0.1:8000`.

## API Endpoints

### Send Tokens
**POST** `/send`
- **Request Body**:
  ```json
  {
    "address": "recipient_address",
    "amount": 100
  }
  ```
- **Response**:
  - `200 OK`: Tokens sent successfully.
  - `500 Internal Server Error`: Failed to send tokens.

### Receive Tokens
**POST** `/receive`
- **Request Body**:
  ```json
  {
    "address": "sender_address",
    "amount": 100
  }
  ```
- **Response**:
  - `200 OK`: Tokens received successfully.
  - `500 Internal Server Error`: Failed to receive tokens.

### Get Balance
**GET** `/balance`
- **Response**:
  ```json
  {
    "balance": 1000
  }
  ```
  - `200 OK`: Returns the current balance.
  - `500 Internal Server Error`: Failed to get balance.

## Code Explanation

### `TokenManager`
A struct that manages the token balance.

- **`new()`**: Initializes a new `TokenManager` with a balance of 0.
- **`send_tokens(amount: u64)`**: Deducts the specified amount from the balance if sufficient funds are available.
- **`receive_tokens(amount: u64)`**: Adds the specified amount to the balance.
- **`get_balance()`**: Returns the current balance.

### Database Interaction
The application uses MySQL for storing and retrieving transaction data. It interacts with the database using the `mysql` crate and leverages a connection pool for efficient database access.




