# Claim standing and evidence boundaries

Current development claim: **PCC-20261123-f873fd**.

- Kind: statement; claim type: data.
- Subject: `DEVELOPMENT_RECEIPT_v2.json` and `identity/certificate.json`.
- Standing: **unverified**; no signed execution or proof verdict.
- Scope: fixed optimized native contracts, parser/source checks, prospective
  input generation and evaluated identity evidence.
- Outside scope: admitted campaign execution, recovery yield, solving cost,
  performance ranking, formal proof and the whole-repository library gate.

The receipt reports 76 passing native tests across 17 immutable executables,
77 screening inputs, 385 unselected recovery candidates, and zero campaign
attempts. Its exact identity field is a decimal string. [pcc:unverified]

Earlier **PCC-20261123-cff73c** remains in the append-only ledger and pins the
preserved original metadata receipt. Its `cli.identity_field` was rounded by
JavaScript Number parsing during packaging. [ERRATA.md](ERRATA.md) explains
the one-field correction; native certificate bytes were already correct.
The earlier claim is also unverified. [pcc:unverified]

Raw development evidence, claim subjects and the immutable native source
inventory are retained in this directory. A producer's receipt, a successful
source hash check or a co-agent review does not supply a signed PCC verdict.
Use `python3 tools/pcc/pcc.py standing <id>` for authoritative current standing.

## Published-source and document custody

Additional claim **PCC-20261123-a63eb4** pins `POST_PUBLICATION.json`, the
launcher output-preservation regression and final report render receipt.
The immutable binary matched all84 source files to published implementation
commit `44e05d7b3cb7ece0d1e761222f8df4e64b2197a5`; its pre/post hash stayed
unchanged. The continuation is PR #4945 after tracking-only PR #4937 merged.
This claim is also **unverified**, with no signed verdict or experiment
admission. The development claim above retains its original scope and subjects.
[pcc:unverified]
