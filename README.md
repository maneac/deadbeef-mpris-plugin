# DeaDBeeF MPRIS Plugin

Plugin to register DeaDBeeF as a MPRIS media player for GNOME media key support.

Built for, and tested on, Linux only.

## Building

### Docker

```sh
docker run --rm -it -v "$PWD:/output" docker.io/rust /bin/bash -c "cd /tmp
apt update && apt install -y libdbus-1-dev pkg-config libclang-dev && \
git clone --recursive --depth=1 https://github.com/maneac/deadbeef-mpris-plugin && \
cd deadbeef-mpris-plugin && \
cargo build --release && \
cp ./target/release/libmpris.so /output/mpris.so"

mkdir -p ~/.local/lib/deadbeef
mv ./mpris.so ~/.local/lib/deadbeef/mpris.so
```

### Local Tools

Ensure the following are installed and available locally:

- Git
- Rust: <https://www.rust-lang.org/tools/install>

The following will be required to build the plugin, and will
have installation hints during the build for your architecture
if they are not available:

- Clang
- DBus development tools

And of course, DeaDBeeF!

### Steps

1. Clone this repository including the DeaDBeeF submodule:

```sh
git clone --recursive --depth=1 https://github.com/maneac/deadbeef-mpris-plugin
cd deadbeef-mpris-plugin
```

2. Build the plugin:

```sh
cargo build --release
```

3. Copy the plugin to your local DeaDBeeF plugin directory:

```sh
mkdir -p ~/.local/lib/deadbeef
cp ./target/release/libmpris.so ~/.local/lib/deadbeef/mpris.so
```

Note the change in name - this is required for the plugin to load!

4. Start DeaDBeef