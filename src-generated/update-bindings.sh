#!/bin/sh

bindgen \
    --output bindings.rs \
    --whitelist-function '^boolector_(.*)$' \
    --whitelist-type '^Btor(.*)$' \
    --whitelist-type '^Boolector(.*)$' \
    --no-recursive-whitelist \
    --raw-line 'use libc::FILE;' \
    --no-doc-comments \
    ../boolector/src/boolector.h
