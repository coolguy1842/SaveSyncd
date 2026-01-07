# SaveSyncd
Server for [SaveSync](https://github.com/coolguy1842/SaveSync), built with [Rocket.rs](https://github.com/rwf2/Rocket)

### Note
All platforms except for Linux are untested

## Documentation
Documentation for the API is hosted [here](https://coolguy1842.github.io/SaveSyncd/)

## Docker

There is a container you can use, which is automatically built and distributed on GHCR.

It expects two mounts:

- The directory for the configuration should be mounted to `/config`
- The directory for the save data should be mounted to `/data`

There are two examples below. Both will create a `config/` and `data/` directory in your current directory and mount them.

### Manual run

```shell
docker run -p 8000:8000 -v ./config:/config -v ./data:/data ghcr.io/coolguy1842/savesyncd:master
```

### Docker Compose

```yaml
services:
  savesyncd:
    image: ghcr.io/coolguy1842/savesyncd:master
    restart: unless-stopped
    ports:
      - 8000:8000
    volumes:
      - ./config:/config
      - ./data:/data
```

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
