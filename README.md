

# Hashcat Rule Engine

***This is a standalone third-party app not affiliated with hashcat.***

A simple cli tool to parse hashcat rules provided as command line arguments, apply the rules to stdin and output to stdout.

***Note:*** This is a rust port of [my other repo](https://github.com/KMikeeU/hcre).
The rust version should be a lot faster, albeit not implementing as many rules as of now.

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

- Nothing (**:**)
- Lowercase (**l**)
- Uppercase (**u**)
- Capitalize (**c**)
- Invert Capitalize (**C**)
- Reverse (**r**)
- Duplicate (**d**)
- Append Character (**$X**)
- Prepend Character (**^X**)
- Purge (**@X**)
- Replace Character (**sXY**)

To see how any of these rules function, please refer to the official [hashcat rule documentation](https://hashcat.net/wiki/doku.php?id=rule_based_attack#implemented_compatible_functions).


## Known issues

1. Always outputs initial word without rules (as if the ':' rule was applied)
2. Not enough rules. duh.
