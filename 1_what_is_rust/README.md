# What is rust?

Rust is a proggraming language, ahead-of-time compiled language.

Rust syntax is similar to C.

Rust files ends with ``.rs``
## Rust CLI tools
- ``cargo``     package manager
- ``rustc``     compiler
    - ``rustc main.rs``  compiles main.rs file
- ``rustup``    rust version checker
    - ``rustup update`` updates rust version
- ``rustfmt``   rust formatter tool

## Syntax

Scopes must start with `{` and ends with `}`.

End line character is semicolon `;`

Program start method is called `main`

``println`` is a method but ``println!`` is macro. Using a ``!`` means is calling macro not a method.


## Cargo
Cargo is rust's build system and package manager.

### Cargo handles a lot of tasks like

- Building your code
- Downloading the libraries
- And building those libraries. libraries called ``dependencies``

if you want to dependencies in your rust project you must initialize project with cargo.

### Creating project with cargo
```cargo new [project_name]``` creates folder that named [project_name] and this folder includes;
#### Cargo.toml file is configuration file of project. Like metadata about project and dependencies.
```[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
``[package]`` is a section heading about projets metadata like what is the name of project as dependency, version, edition etc...

``[dependencies]`` is a section heading about list of project's dependencies.

#### ``src`` folder is project codebase


### Build project with cargo

``cargo build`` run this command in your project folder and its creates Cargo.lock file and target folder.

Cargo.lock file is tracking your exact versions of dependencies. You won't ever need to change this file manually.

target folder is has compiled executable application files of your project. /target/debug/[project_name] file is executable application of your project.

you can run it directly or simply run with cargo with ``cargo run``

After ``cargo run`` command cargo will feedback about compile statement and after that runs application.

If you want to just check this compile statement you should run ``cargo check``

### Building for release

When you project is finally ready for production you can use ``cargo build --release`` to compile it with Optimization.