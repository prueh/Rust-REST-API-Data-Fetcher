Rust REST API Data Fetcher

Rust REST API Data Fetcher is a minimal, self-contained Rust program that retrieves data from public REST APIs and prints it to the terminal. The project demonstrates idiomatic data handling in Rust, focusing on HTTP requests and JSON parsing without the complexity of asynchronous runtimes. It was built to demonstrate rapid technology mastery using Generative AI.  

Features

 ● HTTP Requests: Uses reqwest::blocking for simple, synchronous networking.
 
 ● JSON Parsing: Uses serde to safely convert API data into Rust structs.
 
● Error Handling: Implements robust error propagation using the ? operator. 

 Tech Stack 
 
● Language: Rust

 ● Crates: reqwest, serde, serde_json 

 Quick Start
 
1.	Clone the repo: 
git clone
(https://github.com/your-username/rust-api-fetcher.git)
cd rust-api-fetcher

2. Run the project:
     cargo run
3. Expected Output: 
Fetching a random fact... ------------------------------------------------ 
RUST FACT FETCHER SAYS: The only letter that doesn't appear on the periodic table is J
