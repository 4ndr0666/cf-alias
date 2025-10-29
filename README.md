# cf-alias

[![Release](https://img.shields.io/github/v/release/4ndr0666/cf-alias)](https://github.com/4ndr0666/cf-alias/releases)

> Create Cloudflare email aliases directly from your terminal or Alfred.

- [Overview](#overview)
- [Install](#install)
  - [Source](#source)
  - [Arch (AUR)](#arch-aur)
- [Usage](#usage)
  - [CLI](#cli)
  - [Alfred](#alfred)
- [License](#license)

## Overview

`cf-alias` lets you create and manage Cloudflare Email Routing aliases from the command line.  
It provides an interface to list, generate, or remove forwarding aliases linked to your Cloudflare account.  

The project uses an undocumented Cloudflare API and is offered as-is.

---

<!-- command-help start -->

```

cf-alias v0.1.9
CLI interface for Cloudflare Email Routing

USAGE:
cf-alias <SUBCOMMAND>

OPTIONS:
-h, --help       Print help information
-V, --version    Print version information

SUBCOMMANDS:
alfred        Commands for the Alfred extension
completion    Generates shell completions
create        Creates a new forwarding email
help          Print this message or the help of the given subcommand(s)
list          List existing email routes.

````

<!-- command-help end -->

---

## Install

### Source

```bash
git clone https://github.com/4ndr0666/cf-alias.git
cd cf-alias
cargo install --path .
````

### Arch (AUR)

Prebuilt binary available through the Arch User Repository:

```bash
yay -S cf-alias-bin
```

---

## Usage

Create a configuration file in `$HOME/.cf-alias.json` with the following keys:

```json
{
  "cloudflare_account_id": "cloudflare-account-id",
  "cloudflare_forward_email": "example@gmail.com",
  "cloudflare_root_domain": "example.com",
  "cloudflare_token": "cloudflare-api-token",
  "cloudflare_zone": "zone-id-for-example.com"
}
```

### CLI

* `cf-alias list` — list existing forwarders.
* `cf-alias create --email-prefix github` — create `github@example.com`.
* `cf-alias create --random` — generate a random alias.

### Alfred

The Alfred workflow is located in [`alfred/mx.alfredworkflow`](alfred/mx.alfredworkflow).
Download and import it into Alfred. Use `mx` as a prefix to trigger commands.

---

## License

Licensed under the [MIT License](LICENSE).
See [`THIRDPARTY.json`](THIRDPARTY.json) and [`THIRDPARTY.md`](THIRDPARTY.md) for dependency license metadata.
