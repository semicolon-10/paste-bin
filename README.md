# Paste Bin

A Pastebin application built with Rust and SQLite.

## Setup

1. Clone the repository:

   ```bash
   git clone git@github.com:semicolon-10/paste-bin.git
   cd paste-bin
   ```
2. Setup SQLite Container and Database:
   ```bash
   docker pull alpine
   docker run -it --name sqlite_container alpine sh
   apk add sqlite
   sqlite3 contentDB.sqlite
   ```
3. Run the Application:
   ```bash
   cargo run
   ```
# Subscribe to my youtube Channel 🎥

[Semicolon](https://www.youtube.com/@Semicolon10)
