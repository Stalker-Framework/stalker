#!/bin/bash

for arch in `ls data`;
  do
    mkdir -p results/$arch;
    cargo run --example deterministic_cipher --release $arch;
    cargo run --example deterministic_cipher --release $arch -p;
    cargo run --example digital_signature --release $arch;
    cargo run --example digital_signature --release $arch -p;
done;
