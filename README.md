# Rusty Chat

### Emma "Crusty" Suarez, Megan Paik, Yash Palkhiwala aka the "jaded rustaceans"

## Description
Rusty Chat is  a chat server that allows various users to communicate via a chat server. 

## Instructions
- `cd server` and then `./target/debug/chat-server` to run server (with `--help` to see options)
- (on separate terminal windows):
  - `cd client` and then `./target/debug/chat-client` to run client (with `--help` to see options)

## Code Architecture
* Server
   * Take advantage of Rust’s concurrency
   * Receive/relay messages to the channel
   * Keep track of connected users
* Client
   * Terminal chat window (GUI)
   * Let user send messages and change nickname

## Example
User 0: 
<br>
<img src="assets/user0.png" />

User 1:  
<br>
<img src="assets/user1.png" />
