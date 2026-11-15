# Pre-data protocol amendment v2

Native `dataset preflight` rejected the original `protocol.json` with exit code 1: `protocol canonical body differs from its freeze hash; file an amendment before new data`. Its exact output is retained in `preflight-v1-rejected.log`, and `preflight-v1-exit-code.txt` records the exit code. No cell was generated and the proposed output directory did not exist when this amendment was filed.

The original protocol file remains immutable, SHA-256 `4b72a9dd291394d05000302a7b29c75f3c14afdcde0dab9ce1c069025b1cd40b`. Its declared body hash is `20eae2acce336d36c6eda01fe9d15db07a8032f37eadfe3bcca4345f188984d1`; the native canonical rule (sorted keys, compact UTF-8 JSON, top-level `freeze` omitted) gives `382af6ea4be9dbe81db9224b371669cf2b3d28a77e466ee314d65c2ab7a52778` for the actual v1 body. The declaration was wrong before any data.

`protocol-v2.json` changes only the version, adds this amendment linkage, and replaces the freeze record with a dated hash of its own canonical body: `a60e98a5b60a2777c33424ab82aaf78c42a9b933eac83b4bcaff9607f0d868ee`. Its file SHA-256 is `0a51be43a049a6e7b8b046112150112a3ebd3b5c8d1a98e8a992b513b56267dc`. The 44-cell n47/n61 × m5..15 × both-twist matrix, seed, work/time/memory/trace caps, exact-computation claim, partial-label analysis, and stopping rule are unchanged. The v2 plan binding is committed separately after this amendment so no data can be attributed to the invalid v1 hash.

The native preflight validates freeze and plan binding; this checkout does not contain the general `tools/experiment_protocol.py` checker, so this functional CLI smoke has no claim of a passed `experiment.export/v1` validation.
