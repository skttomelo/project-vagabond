# project-vagabond
As my current (actual) senior project, this is a multiplayer dueling game written in Rust. While other languages were very enticing as well as combining more than one, I chose Rust because I have been learning it for aprox. 1 year and wanted to push myself. The final product will consist of a multi-threaded TCP server and a graphical client written with GGEZ. Due to the fact that I am still learning, I can not guarantee that everything I do will be best practice or even optimal. Lastly, if you know of any resources (even paid) that could aid me in improving how I write my clients+servers, please let me know because I have really been enjoying the backend of things so far for my project.
# The project in action 😎
![](screenshots_recordings/p5SPIpGdOl.gif)
# How to run
No matter how you plan to run the project, either as an executable or via cargo, you must start the server first and then the clients otherwise the clients will crash on startup.

1. `cargo run --release --bin server [ip_address:port]`
2. `cargo run --release --bin client [ip_address:port]`
# Assets
- Art Assets provided by my wife
# Post Senior Project Plans
- Implement audio
- Sudden Death Round (already have the art for the round created)
- Better damage feedback
- Beginning menu screen where you can enter ip there instead of having to do it via the terminal (TBD)
