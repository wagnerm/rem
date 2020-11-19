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

## Editing notes

You can edit notes with the `edit` command.

Please be aware:
* You must have `$EDITOR` set in your environment
* If your `$EDITOR` adds a trailing new line at the end of the note, it will be trimmed.

```
$ rem-cli cat -n
0: Is mayonnaise an instrument?
1: 987654

$ rem-cli edit 0
...
Note committed! Squidward, I used your clarinet to unclog my toilet!

$ rem-cli cat -n
0: Squidward, I used your clarinet to unclog my toilet!"
1: 987654
```

## Notes Path

By default notes are stored in `$HOME/rem_notes.txt`.

Optionally you can change the notes path by setting an environment variable. You can insert this into your shell profile.
```
$ export REM_CLI_NOTES_PATH=/my-cloud-drive/notes.rem
```

## License

[MIT](https://github.com/wagnerm/rem/blob/master/LICENSE) License
