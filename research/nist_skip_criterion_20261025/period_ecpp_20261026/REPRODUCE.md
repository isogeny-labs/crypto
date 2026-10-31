# Reproduce custody and inspect the saved trial

Hydrate with `python3 tools/artifacts/artifacts.py pull research/nist_skip_criterion_20261025/period_ecpp_20261026` using authorized store access. This is operational I/O only; scientific execution and admission are native.

Check `sha256sum -c runs.tar.xz.sha256` from the study directory. Extract the archive into a fresh directory. `MANIFEST.json` names every file with its bytes and SHA256; reject missing/unlisted files or symlinks, and compare every digest. The external `evidence/manifest.json` must equal the internal manifest byte for byte. `evidence/archive-delivery.json` records the full remote download and all 1,081 verified files. Reading source and logs needs no executable invocation.

The archive root contains `MANIFEST.json`, `custody/` and `runs/`. Repacking from its extraction root is deterministic:

```sh
tar --sort=name --mtime='2026-10-26 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - MANIFEST.json custody runs | xz -6 -T1 > runs.tar.xz
```

For an actual functional rerun, inspect the frozen `PROTOCOL.md`, native source, resource requirements, protected checkpoint configuration and vendor provenance first. `run.sh` requires a fresh output beneath this study's `runs/`, a successful checkpoint gate, PARI2.17.3/OpenSSL and original hash-bound vendor artifacts. `validate_saved.sh` revalidates exact004 packets in a separate fresh output. Neither launcher overwrites prior attempts. Their source snapshots, rather than a mutable current working copy, identify the executed code. No source edit or new scientific experiment is needed to inspect this package.

No independent general mathematical ECPP certificate verifier, formal Lean proof, unified machine-schema protocol/export, Cargo suite, Conductor scope reservation or performance measurement is asserted. The tested compiler, arithmetic, input-primality and software-admission controls are recorded separately.
