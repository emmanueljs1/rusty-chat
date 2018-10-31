<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
Current Usage:

- `cd server` and then `cargo run` to run server
- (on separate terminal windows):
  - `cd client` and then `cargo run` to run client
  - send messages to stdin and see them sent to all clients

Chat Server Project

Overview:
We will build a chat server that will allow various users to communicate in chat rooms. We plan to use the terminal to allow users to chat, and our stretch goal will be to implement in a GUI.


Components:
* Backend
   * take advantage of Rust’s concurrency
   * Receive/relay messages to the channel
   * Support various channels (stretch goal)
* Frontend
   * Terminal chat window (termion)
      * use commands to interact with server, for example:
         * /msg <msg>
         * /nickname <nickname>
         * /color <color>
         * /leave
         * /join <channel> (strech goal)

Example:

Yash's Window:
```
User0 joined.
User1 joined.
User0 changed their name to Yash.
User1 changed their name to Megan.
Yash: Hello
Megan: Hi


> /nickname Yash
> /color red
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
   * Potentially a GUI (stretch goal)
      * Use relm (https://github.com/antoyo/relm) or conrod (https://github.com/PistonDevelopers/conrod)
