# clap-interactive

**A work in progress interactive parser for clap**

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

This is a an extremely basic approach at getting clap enums to parse interactively using inquire. I think if someone put some time into it it could be very useful. Unfortunately I don't have the time to really make this shine. If you make improvements to this (which currently isn't hard haha), please submit a PR.

---