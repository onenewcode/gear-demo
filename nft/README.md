[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=non-fungible-token/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/non_fungible_token_io)

# [Non-fungible token](https://wiki.gear-tech.io/docs/examples/Standards/gnft-721)

### 🏗️ Building

```sh
cargo b -p "nft*"
```

### ✅ Testing

Run all tests, except `gclient` ones:
```sh
cargo t -p "nft*" -- --skip gclient
```

Run all tests:
```sh
# Download the node binary.
cargo xtask node
cargo t -p "nft*"
```
# 概述
NFT是指数字艺术品
