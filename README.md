# Jialox

A crafting interpreter of programming language Jialox implemented in rust, which is based on book "[CRAFTING INTERPRETERS](http://www.craftinginterpreters.com/contents.html)" and authored by [Jiashu](https://github.com/Jiashu-ht).

I started learning this book from [Uncle Scientist](https://www.youtube.com/@UncleScientist), a youtuber who has released many practical Rust tutorials. Welcome to subscribe to his channel. This [series](https://www.youtube.com/watch?v=WdoAJ_ouWRM) of videos teaches this book.

# Run
Download the corresponding branch and enter the project directory.
```sh
# Read and execute line by line.
cargo run

# Read an entrie file and excute.
cargo run -- example.txt

```

# Introduction

This project is divided into different release versions based on chapter learning, for example, Section 4 corresponds to version 0.1.0, Section 5 corresponds to version 0.2.0, and so on. Alternatively, you can directly refer to the [Release branches](#release-branches) to find the desired version.

You can start learning from version 0.1.0, gradually improve from a small project, and finally complete the entire interpreter. It is really impressive!

# Release branches

- release/0.1.0

This version completes the content of [Section 4 Scanning](http://www.craftinginterpreters.com/scanning.html).

**Added** nested block block comments - Section 4 challege 4

- release/0.2.0

This version completes the content of [Section 5 Representing Code](http://www.craftinginterpreters.com/representing-code.html)

**Added** buid.rs and generate_ast to automatically generate code `./src/expr.rs`, which is not included and ignored in project.

**Discarded** the file 'ast_printer.rs', which will not be used later.

- release/0.3.0

This version completes the content of [Section 6 Parsing Expressions](http://www.craftinginterpreters.com/parsing-expressions.html)

**Added** a parser to parse the token sequence passed by the scanner.

**Added** ast_printer to print the parsed expression. And the file 'ast_printer.rs' will be used for a period of time.