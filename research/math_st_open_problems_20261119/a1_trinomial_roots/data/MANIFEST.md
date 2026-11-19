# Raw outputs (not in Git; regenerate deterministically with the commands below)

```
cargo build --release
./target/release/a1_trinomial_roots sweep 128 4096 data/sweep_128_4096.csv
for n in 65497 65519 65521; do ./target/release/a1_trinomial_roots prime $n data/prime_$n.csv; done
```

| file | bytes | sha256 |
|---|---:|---|
| data/prime_65497.csv | 1625683 | e7cf3936df7456b7c1c483617cfb758b3d68178f81466a52bf5f202b89937138 |
| data/prime_65519.csv | 1651892 | 9e782e1f28a11931f4fd9fab737d0a651ad96f9421790669ff5c495a3c635445 |
| data/sweep_128_4096.csv | 23934308 | 2f0b03b5d4bb9592f9be47108d3b562af1c614d41e5ec4e0c2821b8bce632d6f |
