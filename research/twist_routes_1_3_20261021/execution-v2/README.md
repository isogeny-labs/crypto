# Follow-up execution, 2026-10-23

[Current results and remaining requirements](../reports/followup-20261023/RESULTS.md)
and [combined PDF](../reports/followup-20261023/RESULTS.pdf).
[Native assessment](assessment-20261023-v3/ASSESSMENT.json),
[tracking update](TRACKING_20261023.json), and
[universal supporting certificates](identities-20261023-v1/SUMMARY.json).

E18–E25 and E29 have bounded, locally verified partial evidence. E26–E28
remain unimplemented/unrun, and final E28 target labels 32–63 have not been
generated. The new E30 finite-incidence pilot completes its 270 correctness
cells, including independent PARI replay, and finds no maximum-generator-degree
reduction in its fixed grid. Its full solver/cost experiment remains open.
All sixteen route/follow-up cards remain blocked and unfunded; no cairn lease,
allocation, publication or funding transaction was performed.

No calibrated IC speedup, novelty, independent-host replay or NIST DLP is
established. Public P-521 work constructs torsion and explicit cover maps;
it does not attempt a public-curve logarithm. BHLS degree-three geometry is
prior art (arXiv:1403.6911, Algorithm 5.4 and Appendix A).

## Evidence publication and extraction

The full deterministic raw archive exceeds Git's 10 MiB limit. It is prepared
in the persistent workspace, with its digest, size and exact local location
in [RAW_ARTIFACTS.json](RAW_ARTIFACTS.json). **The object has not been uploaded
to the configured store, and no resolvable store name/index entry is claimed.**
The environment has no AWS credentials. The installed AWS CLI reaches the
credential check and reports `NoCredentials`; both store-status attempts are
retained in [checks/final-20261023/](checks/final-20261023/).
See the [publication blocker](../../../problems/twist-followup-evidence-publication.md).

Small `SUMMARY.json`, `ADJUDICATION.json` and `INVENTORY.json` files stay loose
because the current report, geometry input manifest and assessment name them.
The archive contains the remainder of `runs/`, including failed runs, native
coordinate witnesses, CAS inputs/outputs, parsed bases and checker scripts.
It does not contain duplicate copies of those loose metadata files. Original
commands inside receipts are historical extraction/replay inputs; restore
the complete tree before invoking a checker. The `/runs/` ignore rule prevents
accidentally adding the large raw tree; the selected loose files are tracked
explicitly. No committed historical tree was repacked.

Once the local bundle has been supplied, or after a successful store upload:

```sh
# From this directory, after verifying RAW_ARTIFACTS.json's exact digest:
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

Repack the raw remainder with the exact command in `RAW_ARTIFACTS.json`.
The archive uses sorted names, 2026-10-23 midnight UTC, uid/gid zero,
normalized modes, GNU tar and single-threaded xz -6. A successful store
upload must be followed by a verified pull and a committed `store/index.json`
entry before this publication blocker can close. Do not split the blob or
add a local-only store entry to evade the limit.

## Source and protocol freezes

Raw stdout/stderr remain byte-preserved, including tool-emitted trailing
whitespace. Scoped Git attributes cover only those log extensions; source
and documentation retain strict whitespace checks.

Each command requires fresh output. The receipt records the exact source
revision, pinned binary, argv, hashes, caps, thread count and isolation L0.
No recorded wall time is admitted. Run the historical revision for replay.
The mathematical drivers and statistical assessment are native Rust/CAS;
JavaScript handles transport metadata, hashing and document rendering only.

- [Gluing protocol and first correction](GLUING_PROTOCOL_V2.md): actual
  three-torsion and necessary module filters. All four corrected torsion
  replays pass; original malformed GP attempts remain.
- [Divisor protocol](DIVISOR_PROTOCOL_V1.md) and
  [chart/field correction](DIVISOR_AMENDMENT_V2.md): explicit BHLS transfer
  and independent arbitrary-class replay. All original source points and
  5,855 old divisor classes have accepted independent replay. Polarized graph
  identification, second-map transfer and formal bridges remain open.
- [Higher-prime protocol](HIGHER_TORSION_PROTOCOL_V1.md) and
  [constructor branch correction](HIGHER_TORSION_AMENDMENT_V2.md): five
  selected toy torsion/pairing/Frobenius/graph certificates pass; quotients
  and maps at ell>=5 remain unconstructed.
- [Coupled-component protocol](COUPLED_PROTOCOL_V1.md): image/fiber census,
  identical projected-row controls and exact energy/random-base diagnostics.
- [Projection/partial protocol](PROJECTION_PARTIAL_PROTOCOL_V1.md) and
  [witness-retention correction](PARTIAL_AMENDMENT_V2.md): the original E23
  filenames omitted arity. Preserve v1 as incomplete evidence. V2 and the
  fresh native assessment replay 20,976 unique arm witnesses successfully.
- [Homogeneous protocol](HOMOGENEOUS_PROTOCOL_V1.md), its
  [declaration repair](HOMOGENEOUS_AMENDMENT_V2.md) and
  [complete-basis alternative](HOMOGENEOUS_AMENDMENT_V3.md): applicable F4
  affine outputs pass completeness; three full homogeneous checks remain
  incomplete. Both signature modes fail every cell and the positive control.
- [Assessment protocol](ASSESSMENT_PROTOCOL_V1.md) and
  [streaming correction](ASSESSMENT_AMENDMENT_V3.md): v1 catches the E23
  witness checksum bug; v2 exhausts the unchanged memory cap; v3 streams and
  passes. Each earlier failure and partial output is retained.
- [E30 incidence protocol](INCIDENCE_PROTOCOL_V1.md): all 270 fixed cells
  and independent finite-grid projection checks pass; no degree reduction.
- [Supporting identity protocol](IDENTITIES_PROTOCOL_V1.md): four IDC1
  records, 32 issuer plus 64 extra checker points, false controls rejected.
  These cover universal coordinate/interpolation steps over the schema's
  Mersenne field, not the specific model identities in different fields.

The manual E25 S-pair checker printed its 49-row summary but lost its JSON
receipt when its mutable wrapper's receipt tail failed to parse. It is not
given an invented successful receipt. The separately frozen leading-ideal
replay has a complete successful receipt. See
[FREEZE_CORRECTION_20261023.md](FREEZE_CORRECTION_20261023.md) for the earlier
pre-freeze invocation error. Stack-warning gate amendments retain both the
initial conservative rejections and subsequent exact-warning acceptance;
real GP errors remain inadmissible.

## Verification and reports

Full root library tests pass 4,492 cases with 119 existing ignored tests.
The standalone experiment suite passes its recorded 13/111/16-test binaries;
the isolated supporting identity suite passes 22 cases. Logs retain the
initial preserve-order/canonicalization test failure and its isolated replay.
The [Lean v2 replay](../lean-v2/replay.sh) has durable build/axiom receipts in
[lean-v2/receipts/resume-20261023-v1](../lean-v2/receipts/resume-20261023-v1/):
17 exported theorems, zero failing and zero trusted/native_decide declarations.
The concrete twist-group, odd-subgroup/projector and row-module formal bridges
are still open.

The report builder reads recorded native results and produces editable SVG,
plot inputs, Markdown/HTML and PDFs. Chromium attempts failed in the managed
runtime; their logs are retained, and the final renderer is Pandoc/XeLaTeX
with Inkscape vector conversion. Every final PDF is inspected before commit.
Existing scoreboard, timeline, curve-cover catalog and historical study graphs
are unchanged: no performance value or complete typed transfer route is
promoted. The new report's visual record describes this wave separately.
