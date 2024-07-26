# What's this? 🤔
This is a small CLI-application, which aims to be a combination of `sha1sum`, `sha256sum`, `sha512sum`, `md5` programs

# How to build/use this?
To begin with, you must obtain a copy of the source code. Then, you can get more help by running the following:
```sh
git clone https://github.com/Jujumba/msf-hasher.git
cd msf-hasher
cargo run -- --help # This will build the project and print some useful help!
```

After reading the help, you are able to pass any arguments to the program in this way:
```sh
cargo run -- $argv # Note the -- (!)
```
