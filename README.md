# rfx
A Tiny (333KB) Command-line utility 🛠 and pretty JSON viewer 🍿

[![asciicast](https://asciinema.org/a/dn2BYgANfRcW3rY5eN2CjRtLQ.svg)](https://asciinema.org/a/dn2BYgANfRcW3rY5eN2CjRtLQ)

> Simple as 1 2 3

```bash
$ curl www.example.com/users/1 | rfx | less
```
or you can simply read from the file

```bash
$ rfx simplefile.json
```

to download a pre-build binary for linux checkout [github releases](https://github.com/shekohex/rfx/releases).

to build it localy using Cargo

```bash
$ cargo build --release
```

and then add it to your `$PATH`.
or you could do

```bash
$ cargo install --path .
```


## Contributing

You are welcome to contribute to this project, just open a PR.

## Authors

* **Shady Khalifa** - _Initial work_

See also the list of [contributors](https://github.com/shekohex/rfx/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.