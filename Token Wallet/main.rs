use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use dotenv::dotenv; // For loading environment variables
use std::env; // For accessing environment variables
use mysql::*; // MySQL crate for database interaction
use mysql::prelude::*; // MySQL prelude for common traits
use actix_files as fs; // For serving static files

// Struct for token transactions, used for sending/receiving tokens
#[derive(Serialize, Deserialize)]
struct TokenTransaction {
    address: String,
    amount: u64,
}

// Struct for representing the balance response
#[derive(Serialize)]
struct Balance {
    balance: u64,
}

// Establish a connection to the MySQL database
fn establish_connection() -> Pool {
    dotenv().ok(); // Load environment variables from a .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // Get the database URL
    let opts = Opts::from_url(&database_url).expect("Invalid DATABASE_URL"); // Parse database URL into options
    Pool::new(opts).expect("Failed to create pool.") // Create a connection pool
}

// Handler for sending tokens
async fn send_tokens(transaction: web::Json<TokenTransaction>) -> impl Responder {
    let pool = establish_connection(); // Connect to the database
    let mut conn = pool.get_conn().expect("Failed to get connection from pool");

    // Insert a 'send' transaction into the transactions table
    let result = conn.exec_drop(
        "INSERT INTO transactions (address, amount, transaction_type) VALUES (:address, :amount, 'send')",
        params! {
            "address" => &transaction.address,
            "amount" => transaction.amount,
        },
    );

    match result {
        Ok(_) => {
            // Update the balance by subtracting the sent amount
            conn.exec_drop(
                "UPDATE balance SET balance = balance - :amount",
                params! {
                    "amount" => transaction.amount,
                },
            ).expect("Failed to update balance");
            HttpResponse::Ok().json("Tokens sent") // Respond with success
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to send tokens"), // Handle errors
    }
}

// Handler for receiving tokens
async fn receive_tokens(transaction: web::Json<TokenTransaction>) -> impl Responder {
    let pool = establish_connection();
    let mut conn = pool.get_conn().expect("Failed to get connection from pool");

    // Insert a 'receive' transaction into the transactions table
    conn.exec_drop(
        "INSERT INTO transactions (address, amount, transaction_type) VALUES (:address, :amount, 'receive')",
        params! {
            "address" => &transaction.address,
            "amount" => transaction.amount,
        },
    ).expect("Failed to insert transaction");

    // Update the balance by adding the received amount
    conn.exec_drop(
        "UPDATE balance SET balance = balance + :amount",
        params! {
            "amount" => transaction.amount,
        },
    ).expect("Failed to update balance");

    HttpResponse::Ok().json("Tokens received") // Respond with success
}

// Handler for getting the balance
async fn get_balance() -> impl Responder {
    let pool = establish_connection();
    let mut conn = pool.get_conn().expect("Failed to get connection from pool");

    // Query the current balance from the balance table
    let result: Result<Option<u64>, _> = conn.exec_first("SELECT balance FROM balance", ());

    match result {
        Ok(Some(balance)) => HttpResponse::Ok().json(Balance { balance }), // Respond with the balance
        _ => HttpResponse::InternalServerError().json("Failed to get balance"), // Handle errors
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Serve static files from the "frontend" directory
            .service(fs::Files::new("/", "frontend").index_file("index.html"))
            // Define API routes
            .route("/send", web::post().to(send_tokens))
            .route("/receive", web::post().to(receive_tokens))
            .route("/balance", web::get().to(get_balance))
    })
    .bind("127.0.0.1:8000")? // Bind the server to the specified address and port
    .run()
    .await
}
