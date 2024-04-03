#!/bin/bash

rustup override set nightly
cargo bench -- goldilocks_8
cargo bench -- goldilocks_12
cargo bench -- goldilocks_16
cargo bench -- goldilocks_20