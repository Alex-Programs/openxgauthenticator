# openxgauthenticator

Free, open source authenticator for [Sophos XG]((https://www.sophos.com/en-us/medialibrary/pdfs/factsheets/sophos-xg-series-appliances-brna.pdf)). Login to firewalls with a single command.

It utilises my [libopenxg](https://github.com/Alex-Programs/libopenxg) library.

### Advantages
- Code is auditable and customisable due to being open source.
- Easier to install, producing a single executable file.
- Fast and lightweight
- Handles loss of connection better
- Additional configuration options
- User agent faking feature allows you to pretend you're using a browser to authenticate
- Can be downloaded from github or the website instead of having to manually authenticate then use the local web portal to download the binary.
- Absolutely no data exfiltration, telemetry, or spyware - and you can prove it by looking at the code.

---

## Installation
### Precompiled Binary Executables (Easier)

[Linux Download](https://openxg.alexcj.co.uk/downloads/openxgauthenticator-linux)

[Windows Download](https://openxg.alexcj.co.uk/downloads/openxgauthenticator.exe)

### Circumventing download
The website, [openxg.alexcj.co.uk](https://openxg.alexcj.co.uk), contains a tool for downloading the executables behind firewalls that block them.

There are no precompiled MacOS binaries because I don't have one to compile on. You can still compile from source.

### Compile from source (Code can be checked)
1. Install Rust Stable from your package manager or [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone the repository to your local machine
3. Open the repository in a terminal and go into the `openxgauthenticator` directory
4. [Optional] read the code in the `src` directory to check that it's safe.
5. Run `cargo build --release`
6. Wait for compilation to complete. Subsequent compilations should be faster.
7. Your compiled binary executable should be in the `target/release` folder. There will be additional files alongside the executable - the executable should just be the file `openxgauthenticator` for linux, or `openxgauthenticator.exe` for windows

---

## Usage
Move `openxgauthenticator` to a directory where you can easily access it, or make a shortcut to it. Run it and give it your credentials. These credentials will be stored in the same folder, unencrypted, in the `config.toml` file. 

The host of the login page will be set to `https://172.29.39.130:8090` by default. Depending on your setup, this may need to be changed. To do so, just modify the `config.toml` file in a text editor.

### Optional Configuration
Additional config options in `config.toml` are:
- `keepalive_delay`, or the time in seconds between telling the firewall you're still there. Default is 90.
- `retry_delay`, or the time between retrying a login/keepalive attempt if it fails.
- `do_stealth_ua`. With this at `false` the user-agent for requests will be set to `openxgauthenticator/1.0.0 (OS not shared); Rust Reqwest; by Alex. See openxg.alexcj.co.uk`, informing the network admin that you're using this tool. If you don't want them to know that, you can set `do_stealth_ua` to `true`, and it will pretend that it's a common browser configuration - `Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36`
