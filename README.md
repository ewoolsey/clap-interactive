# clap-interactive

**A work in progress interactive parser for clap**

---

## Demo

---

https://user-images.githubusercontent.com/8366997/198078221-5fa01e97-a921-4441-b054-f75f4d1ff272.mp4

---

## Usage

---

```rust
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Git {
        #[command(subcommand)]
        subcommand: SubCommand,
        arg: String
    }

    #[derive(Parser, Debug)]
    #[clap(rename_all = "snake_case", infer_subcommands=true)]
    enum SubCommand {
        Commit {
            message: String
        },
        Clone {
            address: String
        }
    }

    fn main() {
        let git = Git::interactive_parse().unwrap();
        println!("{:?}", git);   
    }
```
---

## Looking for others to contribute

---

This is a an extremely basic approach at getting clap enums to parse interactively using inquire. If you make improvements to this please submit a PR, and if you have any issues or bugs please submit an issue. I'm currently actively maintaining this project as a person development tool.

---
