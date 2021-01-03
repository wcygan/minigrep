# Minigrep

This is a project from Chapter 12 of the [Rust Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)  

## Usage
Run with `cargo run search_string file_to_search`  
e.g., `cargo run toxic toxic.txt`

Running a search for "toxic" against Britney Spears' classic "Toxic" should yield the following:
```
You're toxic, I'm slippin' under
Don't you know that you're toxic?
Don't you know that you're toxic?
You're toxic, I'm slippin' under
Don't you know that you're toxic?
Don't you know that you're toxic?
Don't you know that you're toxic?
You're toxic, I'm slippin' under
Don't you know that you're toxic?
You're toxic, I'm slippin' under (toxic)
Don't you know that you're toxic?
Intoxicate me now, with your lovin' now
Intoxicate me now, with your lovin' now
```

Further, you can search without case-sensitivity by setting the environment variable `CASE_INSENSITIVE` to any value.

Attached in the repository is a file `result.txt` which was generated from running this command:
`CASE_INSENSITIVE=1 cargo run toxic toxic.txt > result.txt`.