#!/bin/bash
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include/
export OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib
export PATH="$PATH:$(pwd .)"
cd ./offloader
cargo build