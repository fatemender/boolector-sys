#!/bin/sh

bindgen \
    --output bindings.rs \
    --whitelist-function '^boolector_(.*)$' \
    --whitelist-type '^Btor(.*)$' \
    --no-doc-comments \
    ../boolector/src/boolector.h
