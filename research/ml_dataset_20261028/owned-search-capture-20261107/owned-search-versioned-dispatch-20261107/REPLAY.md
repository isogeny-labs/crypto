Reproduce in a new copy of this packet to preserve the frozen logs. Do not run prepare.py against a changed donor without checking SOURCE_BINDINGS.json. Cached dependencies must match their hashes.

```sh
python3 test_prepare.py
timeout 60 bash -c 'ulimit -v 1572864; ulimit -t 60; rustc +1.98 --edition=2021 --test harness.rs -L dependency=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps --extern serde=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libserde-cac565bd9dd1312a.rlib --extern serde_json=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libserde_json-10f254ba1c7de3e0.rlib -o request-tests' >compile.log 2>&1
timeout 30 ./request-tests --test-threads=1 >tests.log 2>&1
```

Compiler exit must be checked before running a test binary. This command uses the existing dependency cache; it does not authorize rebuilding that cache.
