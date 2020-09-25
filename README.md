# rem-cli
Remember things, on the CLI

## Installation

```
$ cargo install rem-cli
```

Or clone this repository and compile!

## Usage

```
$ rem-cli add Is mayonnaise an instrument?

$ rem-cli add 987654

$ rem-cli cat
Is mayonnaise an instrument?
987654
```

## Notes Path

By default notes are stored in `$HOME/rem_notes.txt`.

Optionally you can change the notes path by setting an environment variable. You can insert this into your shell profile.
```
$ export REM_CLI_NOTES_PATH=/my-cloud-drive/notes.rem
```

## License

[MIT](https://github.com/wagnerm/rem/blob/master/LICENSE) License
