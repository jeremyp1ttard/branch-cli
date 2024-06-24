# Brancher

## A tool to create branch names inline with the cashies branching convention

1. This tool is written in rust for funsies. If you don't have Rust installed on your machine your first step will be to head here <https://www.rust-lang.org/tools/install> and follow the instructions.
2. Next you need to build the package. From the root directory of the project run

```cmd
cargo build --release
```

3. You then need to install it to your path. The most simple way to do that wil be to use cargo. You can drop it into your bin yourself and build the exe but that sounds boring.

```cmd
cargo install --path path/to/root/of/project
```

4. If successful you will be able to use 'brancher' from the command line. 
```cmd 
brancher
```

and it should ask you to input a ticket number.