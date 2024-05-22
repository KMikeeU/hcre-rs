
# Hashcat Rule Engine

[![Build (Release)](https://github.com/KMikeeU/hcre-rs/actions/workflows/release.yml/badge.svg)](https://github.com/KMikeeU/hcre-rs/actions/workflows/release.yml)

***This is a standalone third-party app not affiliated with hashcat.***

A simple cli tool to parse hashcat rules provided as command line arguments, apply the rules to stdin and output to stdout.


## Installation

### From release

Simply download a binary from the release section of this repo.

### From source

```shell
cargo install --git https://github.com/KMikeeU/hcre-rs
```

## Usage

```shell
hcre-rs -r <path to rule file>
```

### Examples

#### hcre + Gobuster

```shell
cat directory-list.txt | hcre-rs -r example.rule | gobuster dir -u http://localhost:8080/ -w - -x php
```

## Implemented rules

**NOTE**: Rules which have not yet been implemented will be ignored

- Nothing (`:`)
- Lowercase (`l`)
- Uppercase (`u`)
- Capitalize (`c`)
- Invert Capitalize (`C`)
- Toggle Case (`t`)
- Toggle @ (`TN`)
- Reverse (`r`)
- Duplicate (`d`)
- Duplicate N (`pN`)
- Refect (`f`)
- Rotate Left (`{`)
- Rotate Right (`}`)
- Append Character (`$X`)
- Prepend Character (`^X`)
- Truncate left (`[`)
- Truncate right (`]`)
- Delete @ N (`DN`)
- Extract range (`xNM`)
- Omit range (`ONM`)
- Insert @ N (`iNX`)
- Overwrite @ N (`oNX`)
- Truncate @ N (`'N`)
- Replace (`sXY`)
- Purge (`@X`)
- Duplicate first N (`zN`)
- Duplicate last N (`ZN`)
- Duplicate all (`q`)

To see how any of these rules function, please refer to the official [hashcat rule documentation](https://hashcat.net/wiki/doku.php?id=rule_based_attack#implemented_compatible_functions).


## Known issues

1. Always outputs initial word without rules (as if the ':' rule was applied)
2. Missing rules
   1. Memory rules
   2. Reject rules
   3. hashcat specific rules/functions
