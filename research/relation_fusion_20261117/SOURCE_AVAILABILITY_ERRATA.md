# Source availability correction

Earlier frozen notes described main as retiring the source crates and journal. That diagnosis was incorrect: symbolic Git main fetched an older snapshot than the GitHub API. Exact API-confirmed main c5cbb67b19fe39864da7f8c9b29726696ba01580 retains both original crates. This correction changes no frozen source pins, raw observations, proof obligations or timing.

DAG_SOURCE_74.json describes the source at 0a4080c649; PRODUCT_SOURCE_77.json describes 715ca1c86d. SOURCE_PROVENANCE and phase71 describe the earlier 6aeefbe4de source. Later mutable files do not retroactively satisfy those pins. Phase71 parent clock ends at child exit; controller readback is outside that clock. New scan includes that readback explicitly. [pcc:unverified]

ROOT_DAG_SOURCE_87_88.json describes0bab6ab79e before the later test-only connected-path addition. Frozen96 postpins were captured before that test edit. Live runtime math did not change after the measured build.
