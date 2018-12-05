# Rusty Chat

### Emma "Crusty" Suarez, Megan Paik, Yash Palkhiwala aka the "jaded rustaceans"

## Description
Rusty Chat is  a chat server that allows various users to communicate via a chat server. 

## Instructions
- `cd server` and then `cargo run` to run server
- (on separate terminal windows):
  - `cd client` and then `cargo run` to run client
  - send messages to stdin and see them sent to all clients

## Code Architecture
* Server
   * take advantage of Rust’s concurrency
   * Receive/relay messages to the channel
   * Support various channels (stretch goal)
* Client
   * Terminal chat window (termion)
      * use commands to interact with server, for example:
         * /msg <msg>
         * /nickname <nickname>

## Example

Yash's Window:
```
User0 joined.
User1 joined.
User0 changed their name to Yash.
User1 changed their name to Megan.
Yash: Hello
Megan: Hi


> /nickname Yash
> /msg Hello
>
```
--------------------------------------------------------------------------------------------------------------
Megan's Window:
```
User0 joined.
User1 joined.
User0 changed their name to Yash.
User1 changed their name to Megan.
Yash: Hello
Megan: Hi



> /nickname Megan
> /msg Hi
>
```