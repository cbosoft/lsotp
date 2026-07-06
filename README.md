# lazy sod's least secure OTP


Alright, to be clear, this should not be used. OTP secrets are stored in plain text in your home folder and it circumvents the whole purpose of MFA. I don't want to improve this, as it will give a false sense of security.

That said... why on earth does a flow chart tool require MFA? Looking at you, Miro. Absolute nonsense. This tool is for those stupid over zealous apps that have decided they need to be as secure as my banking app. When you add the security conscious to your MFA app, take a screenshot of the QR code and import it to `lsotp` too, giving it a memorable profile name:
```bash
lsotp import NAME /path/to/qr/screenshot.png
```
The OTP will be copied into your clipboard if it recognises it's being run in an interactive terminal. If running non-interactively, or it can't get access to the clipboard, it'll output to stdout.

Then fetch the OTP from the commandline with
```bash
lsotp get NAME
```

## Installation

[Install rust first.](https://rust-lang.org/tools/install/)

I'm not putting this on crates.io because it's terrible and should not be used except by the most dedicated lazy sod.

Download this repo and install with cargo:
```bash
git clone git@github.com:cbosoft/lsotp
cargo install --path lsotp
```
