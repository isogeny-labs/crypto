# Qualification v3 review map

Review PLAN.md, mode-preservation.patch, cache-mode-inventory.json, materialize.py and durable-worker.py. The source archive and all control inputs are unchanged. The shared inventory uses one mode-table index per sorted cachepath; the parser validates counts, executableindices, sourcecachehash and regular-only/no-special-bit policy.

Prospective launcher requires a separately published admitted record. No admission.json is present. Canonical archive/large-manifest publication belongs to root. Mutable artifact metadata stays separate from semantic hashes.
