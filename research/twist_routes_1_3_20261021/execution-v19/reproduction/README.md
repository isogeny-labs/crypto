# Preserved measured executables

The adjacent managed `executables.tar.xz` preserves thirteen original executables/instrument objects byte for byte. [EXECUTABLES.json](EXECUTABLES.json) maps each member to its recorded original path, SHA-256 and byte count. The scientific sources and locks remain committed; this archive supplements reproducibility across workspace loss.

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v19/reproduction
(cd research/twist_routes_1_3_20261021/execution-v19/reproduction && sha256sum -c executables.tar.xz.sha256 && tar -xJf executables.tar.xz)
```

Use a compatible Linux x86-64 host and the original pinned dynamic libraries/tool environment. Restore objects at the paths the original PINS.json and plans name, or create a separately versioned replay manifest with explicit path remapping and unchanged hashes. The archive alone does not substitute for the runtime/library/host pin checks. Set VALGRIND_LIB to the restored Callgrind/core-preload directory; preserve the original GP LD_LIBRARY_PATH. The old dense and new sparse worker/harness/summary binaries, original and corrected old summaries, ROI library, GP and actual Callgrind backend are included. PARI/GP and Valgrind retain their respective upstream licences and source/version provenance from the frozen protocols.

This is byte preservation, not another cost experiment or an independent-host replay.
