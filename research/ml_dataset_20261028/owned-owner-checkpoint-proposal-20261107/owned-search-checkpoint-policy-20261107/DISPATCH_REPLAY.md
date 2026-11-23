In dispatch/, run the following using the bound existing dependency cache. Do not rebuild dependencies implicitly.

```sh
rustc +1.98 --edition=2021 --test harness.rs -L dependency=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps --extern serde=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libserde-cac565bd9dd1312a.rlib --extern serde_json=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libserde_json-10f254ba1c7de3e0.rlib -o tests
./tests --test-threads=1
```

Compile was bounded by 60 seconds and 1.5 GiB virtual memory, followed by a 30-second test cap. Both exited 0. The harness uses the actual proposed types and selection, and actual production profile. It duplicates the six prior request controls for the new policy and includes four existing type controls. No owned worker execution occurs.
