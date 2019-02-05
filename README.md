# Teems Rust

This is the rust implementation of my terminal colorscheme switcher application. It's a toy project I use to learn new languages. That doesn't mean you can't use it, it just might not support a lot of applications.

* [Javascript (NodeJS)](https://github.com/cideM/teems)
* [Bash/awk](https://github.com/cideM/teems-awk)
* [Haskell](https://github.com/cideM/teems-haskell)

  ## Supported Terminal Emulators

* Alacritty
* Kitty
* X
* XTerm
* Termite

## Usage

```
Teems 0.1
Florian B. <yuuki@protonmail.com
Easily switch themes for your terminal(s)

USAGE:
    teems-rust --config <FILE> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    a required json file containing the themes

SUBCOMMANDS:
    activate    Activate a theme
    help        Prints this message or the help of the given subcommand(s)
    list        List all themes
```
