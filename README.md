# project-vagabond
As my current (actual) senior project, this is a multiplayer dueling game written in Rust. While other languages were very enticing as well as combining more than one, I chose Rust because I have been learning it for aprox. 1 year and wanted to push myself. The final product will consist of a multi-threaded TCP server and a graphical client written with GGEZ. Due to the fact that I am still learning, I can not guarantee that everything I do will be best practice or even optimal. Lastly, if you know of any resources (even paid) that could aid me in improving how I write my clients+servers, please let me know because I have really been enjoying the backend of things so far for my project.
# How to run (NOTE: currently there is major desync between clients and the server because they reset each others data unfortunately)
No matter how you plan to run the project, either as an executable or via cargo, you must start the server first and then the clients otherwise the clients will crash on startup.

1. `cargo run --release --bin server`
2. `cargo run --release --bin client`
# Assets
- Art Assets provided by my wife
- Sound Assets TBD
