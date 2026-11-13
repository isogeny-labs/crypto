# Trace-fiber geometry instruments

Status: compiled implementation only; no geometry, audit, mutation or proof path has executed. Compilation and help-path checks are producer observations, not mathematical or signed PCC verification. [pcc:unverified]

This continues the auxiliary trace-fiber lead in `../auxiliary-20261113/`. Its source references, derivation, projective cases and source-DLP acceptance criteria still apply. No novelty claim is added here. The immediate question is whether the quartic chart covers the actual finite trace fibers, including tangents and the identity fiber. [pcc:unverified]

The Rust producer enumerates quartic parameters. The separate C++ reference enumerates all extension-field curve points, selects by group trace and reconstructs parameters from those points. They have separate field and group arithmetic: Rust uses a norm inverse, C++ exponentiation; C++ computes Frobenius by exponentiation. Shared libraries handle JSON and hashing only. The reference never obtains its point population from candidate output. These are implementation intentions pending scientific replay. [pcc:unverified]

Each query retains all 1369 candidate outcomes and every accepted geometric point. The fixed scope is only P=(1,8); full scope includes O and all 18 nonidentity points of the supplied order-19 subgroup. The auditor requires the expected scope independently of the dataset. Ten mutation paths alter and rehash payloads; inapplicable controls fail explicitly and must not be counted as successful changed-payload rejection. None has run. [pcc:unverified]

The trace witness is auxiliary: many witnesses for one projected coefficient relation do not increase useful relation rank. This geometry instrument performs zero scalar recoveries and no relation solve, so its matrix degree is null. Even a complete coverage pass would not establish useful-yield, relevant-degree or original-source-DLP advantage. Those require the separately frozen factor-base, cold public setup, complete source-policy frontier and original scalar checks described in the auxiliary note. [pcc:unverified]

## Ordinary build observations

Rust 1.85.1 release build passed using the tracked Cargo.lock and two workers. C++20 compiled with g++14.2.0, nlohmann/json3.11.3 and OpenSSL. Initial C++ compilation passed with two indentation warnings; final compilation passed `-Wall -Wextra -Werror`. Both help paths returned0 before scientific arithmetic. The first source copies and logs are retained in `validation/build-attempts.tar.xz`. These are producer observations only. [pcc:unverified]

Use user-owned external build directories; do not commit binaries or dependencies. The C++ command is:

```sh
timeout 600 g++ -std=c++20 -O2 -Wall -Wextra -Werror \
  -I/path/to/pinned/nlohmann/include auditor.cpp -lcrypto -o /outside/auditor
```

Cargo uses the producer manifest, `--release --locked --jobs 2` and an outside-checkout target directory. There are no build scripts in this crate.

## Before execution

The existing branch upload permission persists. This fresh runtime has no configured checkpoint credential; no upload or scientific execution was attempted. After secure narrow access is configured, use the current unchanged official checkpoint script successfully before any produce/audit/mutate invocation. Do not use broad credentials or an alternate uploader. [pcc:unverified]

First freeze a fixed-case engineering protocol with source, contract, lockfile, binary, input, analysis, dependency and command hashes. Create a fresh manifest with schema `twist.trace-fiber-inputs/v1` and exactly `producer`, `auditor`, `contract` entries, each containing its repository-relative path and SHA256. Pass the manifest hash independently to both instruments. The expected full/fixed scope is a separate auditor argument. Preserve first failures and false payloads. Then freeze a new canonical full-study protocol using the measured engineering resource estimate. The current CONTRACT.json is not that canonical freeze. [pcc:unverified]

The subsequent `run-fixed.sh` and `ENGINEERING_PROTOCOL.md` now implement the fixed-point engineering launch contract, pinned by `ENGINEERING_INPUTS.json` and `ENGINEERING_PINS.json` before data. The script requires a successful official checkpoint before native commands; it has only syntax/help and prelaunch guards checked. It does not implement the later full census or relation-yield experiment. [pcc:unverified]

No receipt, scientific result, new theorem, identity certificate or signed PCC verdict is claimed. The existing two-page derivation report is unchanged; there is no new result graph for compilation.
