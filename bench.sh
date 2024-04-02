#!/bin/bash

rustup override set nightly
cargo bench -- bn256_different_input_size