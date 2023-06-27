# https://github.com/casey/just

default:
    @just --list

run:
    cd crates/app && dioxus serve