#!/bin/bash

# configuracion para desarrollo usando watchexec

DEBOUNCE="${1-1000}"

watchexec --restart --debounce "${DEBOUNCE}" --exts rs,toml,env -w src -w Cargo.toml -w .env -- cargo run
