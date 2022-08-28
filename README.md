# IPFS Registry

Signed package registry backed by IPFS for storage.

## Preqrequsites

* [ipfs][]
* [rust][]

Minimum supported rust version (MSRV) is 1.63.0.

## Getting Started

Ensure a local IPFS node is running:

```
ipfs daemon
```

Start the server:

```
cd workspace/server
cargo run -- -c ../../sandbox/config.toml
```

## Upload a package

```
PUT /api/package
```

The default mime type the server respects for packages is `application/gzip` so you should ensure the `content-type` header is set correctly.

To upload a package it MUST be signed and the signature given in the `x-signature` header.

The `x-signature` header MUST be a base58 encoded string of a 65-byte Ethereum-style ECDSA recoverable signature.

The server will compute the address from the public key recovered from the signature and use that as the namespace for packages.

If a file already exists for the given package a 409 CONFLICT response is returned.

## Download a package

```
PUT /api/package/:address/:name/:version
```

To download a package construct a URL containing the Ethereum-style address that was used when the package was uploaded along with the package name and semver.

[ipfs]: https://ipfs.io/
[rust]: https://www.rust-lang.org/
