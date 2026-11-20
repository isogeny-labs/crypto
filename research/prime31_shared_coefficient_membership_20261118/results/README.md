# Result artifacts

Commit small source-bound manifests, summaries, solver records, and replay
receipts here.  A compact RR9 field assignment (at most 75 field elements) may
also be committed so its tower, decoder, and EC replay remain directly
reviewable.  Large deterministic formulas, Boolean models, proofs, and raw
resource logs remain local/checkpoint artifacts and are represented by SHA-256
digests.

No registered row may be added before `scripts/checkpoint.sh gate` succeeds.
Timeout, OOM, and unchecked UNSAT stay `unknown`; only independently replayed
SAT models are accepted.

Each case has an arm-qualified filename, so CNF can be compared across
CryptoMiniSat, CaDiCaL, and native priority/plain controls without overwrite.
The compact committed set is the relation, encoding manifest, command record,
solver record, GNU-time resource record, replay receipt, summary, and optional
RR9 assignment.  Formula, full Boolean model, stderr/stdout, and proof files
remain checkpoint artifacts; their content hashes are carried by the compact
records where applicable.
