# Private v6 clock/preparation binding correction

V6 supersedes the unapplied v5 increment; apply only after v4 plus root's reap/publication fixtures. Prior v5 remains unchanged. No build, test execution or live application has occurred.

Exact session-preparation.json bytes omit clock_id and include a static derivation rule. The exclusive publication returns their SHA-256; the journal and every execution prefix/tail use runner_id + '-preparation-sha256-' + that digest as clock_id. An external owner must hash the retained exact preparation file, reproduce that identifier and verify its agreement with the exact journal before using origin_ns. No metadata self-hash or circular identity is introduced. A pure unrun fixture changes origin bytes, whitespace and runner identity to check binding.

All v5 scope limits remain: same verified Linux boot/time namespace/offsets required; restoration unsupported; missing or changed metadata fails closed; no mandatory /proc access without diagnostics; portable fallback cannot align externally. Per-observation metadata reads contribute actual observer overhead. External authenticated exit receipt, final worker/CPU costs, CLI startup and early-failure custody remain unresolved. No launch or measurement qualification is introduced. Journal capacity admission uses the actual longer clock identifier and unchanged 8MiB cap.
