# SUI vanity address generator

**Currently, only the ED25519 signature scheme are supported, using the official sui-sdk which is slow but safe.**

PS. Yes, I know about GPU generators.

```bash
./target/release/sui-vanity-address --help

$ Usage: sui-vanity-address [OPTIONS] <prefix>
$
$ Arguments:
$   <prefix>  The hex prefix need to match
$
$ Options:
$  -t, --threads <threads>  Number of threads for lookup [default: all cpus]
$  -e, --exit               Exit on first match
$  -s, --stat <stat>        Print genrate stats every X seconds [default: 10]
$  -h, --help               Print help
$  -V, --version            Print version
```