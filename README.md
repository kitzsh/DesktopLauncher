# DesktopLauncher

> [!NOTE]
> Most X11 desktop environments currently don't work with this, due to some extra initialisation requirements. Support is in development.

DesktopLauncher is a simple command line display manager which lets the user select and start a Wayland ~~or X11~~ session after logging in.

## Features
- Lists all available Wayland ~~and X11~~ sessions
- Saves the previous selection for easy access
- Simple and fast

## Building
1. Install [Rust](https://www.rust-lang.org/). This can be done through [rustup](https://www.rust-lang.org/tools/install) or your system's package manager. It should be available on almost if not all popular Linux distributions.

2. Clone the repository and change into the directory it cloned to.
```bash
git clone https://github.com/kitzsh/DesktopLauncher
cd DesktopLauncher
```

3. Install dependencies and build the package.
```bash
cargo build --release
```

## Installing
1. Change into the directory which contains the `desktoplauncher` binary. If built from source, this would be `target/release/`.
```bash
cd target/release/
```

2. Copy the binary to `/usr/local/bin`, or any directory in `$PATH`.
```bash
sudo cp desktoplauncher /usr/local/bin/desktoplauncher
```

3. At the end of your default shell's rc file (e.g `~/.bashrc` for bash, `~/.zshrc` for zsh), make `desktoplauncher` run if the current TTY is `/dev/tty1`, or whichever TTY you want it to run on.
```bash
if [[ "$(tty)" == "/dev/tty1" ]]; then
    desktoplauncher
fi
```
4. Disable existing display managers, if present.

## Notes
This is my first Rust project. If you have any criticisms or improvements, feel free to make an issue or pull request.

I already have some ideas for this which I could implement in the future, such as: 
- TUI and GUI front-ends with custom login screens.
- Systemd integration.

## License
MIT