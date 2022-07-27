# CNS contract

## Introduction
Most addresses of blockchain are difficult to recognize as address is generated randomly by using key algorithm like secp256k1 and is too long. For easily reminding address, Naming service is that short name(as 'domain') is mapped to blockchain account address similarly DNS. This contract supports to operate methods are creating/retrieving domain in the CNS system. It is inspired by [ENS](https://ens.domains/).
This contract saves domain-account address mapping info.

## Prerequisites
- Rust & cargo
- build
  - cargo run-script optimize
  - Generate wasm file in the directory(/artifact)