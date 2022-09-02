# openxgauthenticator

Open source authenticator for Sophos XG. (https://www.sophos.com/en-us/medialibrary/pdfs/factsheets/sophos-xg-series-appliances-brna.pdf)

Advantages over their provided one:
- Code is auditable
- Easier to install
- Appears to ignore some types of user-account banning, whereby the firewall simply requests the client not let you join. (Unconfirmed)
- Can be modified to your liking
- Handles loss of connection better
- More configuration options
- Lightweight

Prebuilt binaries available at https://openxg.alexcj.co.uk/

To build yourself, install Rust stable, clone, go into `path/to/repo/openxgauthenticator/openxgauthenticator`, `cargo build --release`, and use the binary found in the `dist` folder.

The simulator folder contains a simulated version of the Sophos XG login server for testing the client against during development.
