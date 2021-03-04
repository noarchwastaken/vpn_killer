# vpn_killer

Kill any Android VPN in the browser, and expose the client's real IP address.


## Background

Me and a friend of mine [@A5dblk](https://t.me/A5dblk) stumbled upon [this bug](https://bugzilla.mozilla.org/show_bug.cgi?id=1463833), which allows any Javascript to eat up an infinite amount of RAM on Firefox, or up to 4GB per tab on Chromium.

After playing with the exploit for a while, we noticed that, Android will kill most background processes, including VPNs, when the Javascript is consuming memory.

So I came up with the idea of spamming requests while allocating memory. After the VPN is killed by Android, a few requests will be made without VPN before the browser tab is killed, revealing the client's real IP address.


## Affected Platform

**Any version of Android.**

> The memory exhaution bug affects all platforms that Firefox and Chromium runs on, but the VPN-killing behavior is only tested on Android.


## Installing

[Prebuilt binaries](https://github.com/noarchwastaken/vpn_killer/releases)

The backend of `vpn_killer` is built using [Rocket.rs](https://rocket.rs/) and Rust.

Currently, only binaries for x86_64 GNU/Linux are built.

### Building

0. Clone this repository.

1. [Install and switch to Rust nightly](https://rocket.rs/v0.4/guide/getting-started/) for this repository.

2. `cargo build --release`


## Usage

For `vpn_killer` to work, you need a client that is connected to a VPN and making requests from different IPs with and without the VPN.

### Setting up a VPN for Intranet testing

For testing purposes, you can [set up a Wireguard server](https://git.zx2c4.com/wireguard-tools/about/src/man/wg-quick.8) on your computer, and connect to it on your phone.

With this setup, remember to browse the non-VPN IP of your computer; for example, if your computer (server) has `192.168.1.30` for home intranet and `10.26.0.1` for VPN, you should use the former in the address bar.

### Running

```sh
$ vpn_killer
```

If you are building it yourself:

```sh
$ cargo run --release
```

You will see Rocket launching and listening on `http://0.0.0.0:8000`:

```
🔧 Configured for production.
    => address: 0.0.0.0
    => port: 8000
    => log: critical
    => workers: 24
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => read timeout: 5s
    => write timeout: 5s
    => tls: disabled
Warning: environment is 'production', but no `secret_key` is configured
🚀 Rocket has launched from http://0.0.0.0:8000
```

Browse port `8000` on your computer using an Android device, and click "Get my real IP":

```
63e078cb-343f-4777-bdca-3a4add7e2a14 connected | Initial IP: 10.26.0.30
63e078cb-343f-4777-bdca-3a4add7e2a14 IP changed | known IPs: [10.26.0.30, 192.168.1.250]
```


## Hacking

### Run the exploit automatically

Uncomment the line containing `killVPN();` in `src/assets/index.html`; build again.


## Protecting yourself against it

If you run Android 9 or later, turn on **Always-on VPN** and **Block connections without VPN** in your system VPN settings.

Be aware that this will break split-tunneling (a.k.a. Per-app proxy).

Or you can use [Tor Browser](https://www.torproject.org/), which relies on an internal Tor for proxy, and cannot make connections without it.
