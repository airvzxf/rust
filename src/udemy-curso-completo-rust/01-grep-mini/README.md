# Mini GREP

This program is intended to replicate the Linux `grep` command. This replica is elementary.


Execute:

```bash
grep_mini file.txt text_search
```


## Build

It is using rust, you need to do:

```bash
git clone https://github.com/airvzxf/rust.git
cd src/udemy-curso-completo-rust/01-grep-mini
cargo build --release
./target/release/grep_mini file.txt bye
```
