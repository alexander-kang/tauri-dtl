# tauri-dtl

tauri-dtl is a desktop app for lab administrators in Northwestern University's McCormick PC Support team to quickly & easily distribute files to lab machines within the covered network.

The details of how to use the tool are located in [README.txt](https://github.com/alexander-kang/tauri-dtl/blob/main/README.txt), which should also be passed around with the executable.

## Features

- Copy to multiple labs at once
- Select specific systems in each lab to target
- Logging of successful and failed operations
- Supports both file and folder operations
- Runs from a single executable that can be stored on a flash drive
- Small (storage), lightweight (memory), and fast

## Tech

tauri-dtl is built on:

- [Tauri](https://tauri.app/) - a toolkit built on Rust to help make desktop apps
- [Node.js](https://nodejs.org/en) - used by Tauri

## Setup

Note that you only need to follow the steps below if you wish to develop or build executables.
End users need only run an executable to use the tool.

Tauri requires Rust and system dependencies to run. It's probably best to follow [this guide](https://tauri.app/v1/guides/getting-started/prerequisites) from the Tauri website.

If you haven't already, clone this repository to your desired location.
```sh
cd <yourdesiredfilelocation>
git clone https://github.com/alexander-kang/tauri-dtl.git
cd tauri-dtl
```

Next, install the Node.js dependencies and devDependencies.

```sh
npm install
```

Finally, compile Rust packages and their dependencies.

```sh
cd src-tauri
cargo build
```
