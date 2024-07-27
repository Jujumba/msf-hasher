# What's this? 🤔
This is a small CLI-application, which aims to be a combination of `sha1sum`, `sha256sum`, `sha512sum`, `md5sum` programs.

That is, this program can hash contests of specified files and verify hashes accordingly

# How to build/use this?
To begin with, you must obtain a copy of the source code. Then, you can get more help by running the following:
```sh
$ git clone https://github.com/Jujumba/msf-hasher.git
$ cd msf-hasher
$ cargo run -- --help # This will build the project and print some useful help!
```

After reading the help, you are able to pass any arguments to the program in this way:
```sh
$ cargo run -- $argv # Note the -- (!)
```

For example, to hash `lorem ipsum.txt` using `SHA512` algorithm, one can run the following snippet:
```sh
$ cargo r -- --hash SHA512 "lorem ipsum.txt"
b2a944ef3800ff1d95052e9af751cc6c  lorem ipsum.txt
```

A slightly advanced example of usage is piping the output:
```sh
$ cargo r -- --hash SHA512 "lorem ipsum.txt" | cargo r -- --hash SHA512 -c
lorem ipsum.txt: OK
```
