
# Todo Ratatui

This is an introductory project to the Ratatui library to learn some terminal user interfaces and improve my Rust knowledge. I do not recommend anyone to seriously use this as a Todo App.


**What it is** 

It is a terminal user interface Todo App, with a nice opening Ascii Art screen, Ascii Art title, dynamic input help based on InputMode, and a popup system to edit and view the body of TodoItems.

There are rough edges that I know of, and even more that I'm sure I don't know of. However both the UI and behaviour should be the same on all platforms, mac linux and windows. I've tested all 3.

It will save your todos in a .todo_temp.json file at the root of where you clone this repo, and load them when you next open the app.

I'll keep working on it in my free time when I think of something I want to add, and may do a big pass over the code at some point in the future.
## Acknowledgements

 - [Ratatui](https://crates.io/crates/ratatui)
 - [Serde](https://crates.io/crates/serde)
 - [Serde_json](https://crates.io/crates/serde_json)
 - [Chrono](https://crates.io/crates/chrono)
 - [Indoc](https://crates.io/crates/indoc)
 - [Itertools](https://crates.io/crates/itertools)

