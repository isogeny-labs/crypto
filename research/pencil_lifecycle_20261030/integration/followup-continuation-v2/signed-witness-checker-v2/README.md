# Complete signed witness checker

All 64 pinned certificates passed direct modular-sum verification: 39 determined at scalar 2 and 25 undetermined, covering 6,900 source-bound rows. Eleven deliberately corrupted coverage, row, scalar, terminal and summary controls rejected. See check-result.json and check.exit (0). No producer, elimination or collector ran.

The wrapper enforces unique and complete certificate/source/archive/summary sets, member hashes and lengths, canonical node/sign extraction, terminal scalar and exhaustion agreement, and exact summary joins. Input pins preserve the original reviewed revision and sources, including the original checker hash. Run from this directory with the bounded command in PLAN.md. The wrapper imports only the pinned direct check_certificate.py, never certificates.py or run.py. No assertions are used as validation gates.

This is deterministic software validation of the existing 64 retained traces. The original packet is unchanged. The reviewer of that original packet authored this wrapper; wrapper source review is still independent work. Identity-event extraction is not exercised by the retained traces; geometric/source-group verification is inherited from the earlier native packet.

The companion signed-witness-independent-review packet contains the original non-author source review, its exact source hashes and findings. No new scientific result, timing or cost claim is made. Canonical scientific plots and the original packet figures remain unchanged; these PDFs document software review scope only.
