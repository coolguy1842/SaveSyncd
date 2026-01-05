# SaveSyncd
Server for [SaveSync](https://github.com/coolguy1842/SaveSync), built with [Rocket.rs](https://github.com/rwf2/Rocket)

### Note
All platforms except for Linux are untested

## Documentation
Documentation for the API is hosted [here](https://coolguy1842.github.io/SaveSyncd/)

## Usage
Run with:
```sh
cargo build --release
./target/release/SaveSyncd
```
You can also build with: `--no-default-features` to disable the tray icon.
The default paths are:

### Config
| Platform | Path                                                                     |
| -------- | ------------------------------------------------------------------------ |
| Windows  | C:\Users\user\AppData\Roaming/SaveSyncd/config.json                      |
| Linux    | /home/user/.config/SaveSyncd/config.json                                 |
| Mac      | /Users/user/Library/Application Support/SaveSyncd/config.json            |

### Data (Defined in Config)
| Platform | Path                                                                     |
| -------- | ------------------------------------------------------------ |
| Windows  | C:\Users\user\AppData\Roaming/SaveSyncd                      |
| Linux    | /home/user/.local/share/SaveSyncd                            |
| Mac      | /Users/user/Library/Application Support/SaveSyncd            |