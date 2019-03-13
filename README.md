## How to use

```console
$ cargo run -- --help

USAGE:
    text2image [OPTIONS] <TEXT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --brand <BRAND>    Brand text. Default: Reing
        --output <path>    Output file path. Default: ./output.jpg
        --rgb <RGB>        Background color. Default: 00,A2,9A

ARGS:
    <TEXT>    Text to write.
```

## Example

```console
$ cargo run -- '5000兆円欲しい！！' --output example/5000tyouen.jpg
```

![](./example/5000tyouen.jpg)

```console
```
