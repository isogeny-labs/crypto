# Strict matched-panel analysis

`analyze.py` parses the compiled matched runner's JSONL receipts without executing
cryptography. It requires one exact header and30 ordered records: six setup/prepare
records followed by24 queries, preserving the target-parity schedule. It rejects
duplicate JSON keys, nonfinite values, unexpected fields, wrong dimensions,
non-u64 counters, boolean/float counters, stage contamination, malformed recovery
metadata, missing/extra/reordered rows and nonzero producer exit status.

All baseline/A-A counters must match for setup, preparation and each target.
Geometry and decoder query contracts are checked. Primary field products include
query plus independent verification; polynomial products and allocation-request
bytes are query-only. Unit inversions include query and verification. Allocation
requests are neither live nor peak memory. A supported result requires strict
primary-count reduction on all eight fixtures. Equality or any loss yields the
fixed mixed/no-reduction verdict, with every paired difference preserved.

The command requires a trusted producer exit status and expected protocol SHA256:

```
python3 analyze.py rows.jsonl --protocol-sha256 SHA256 --producer-exit STATUS --out comparison.json
```

Outputs use exclusive creation; raw data is never modified. Invalid panels produce
an invalid_measurement receipt with raw SHA256 and a failing exit code. This parser
does not authenticate supplied metadata or independently replay elliptic arithmetic:
source/binary custody and independent review remain external gates.

Five synthetic unit methods pass. CLI checks verify successful parsing, rejection
of output overwrites, and retention of an invalid partial-panel digest. These
manufactured counts are parser fixtures, not cryptographic measurements.

The revised protocol draft binds the published runner commit and binary SHA plus
the analysis SHA. Canonical validation passes with zero findings. The initial build
metadata schema failure is retained. The protocol remains unfrozen: preflight,
remaining budget and authoritative EKS capacity/quota admission are outstanding.
Live reads were Forbidden under the existing worker role. No comparison was run.
Original4563-case cohort unchanged.

Sources: native matched main.rs and receipt.json, original meter.rs, previous
monic comparison draft, canonical experiment-protocol schema/tool (provenance
retained in feature-native-monic/protocol-tool-provenance.json).
