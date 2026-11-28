# Packed S3 rows: index-calculus relation-finder improvement

The ECC2K-130 uniform relation-query control uses **22.87% fewer complete-oracle instructions** and **11.21% fewer complete-query instructions** against the current merged optimized kernel. The matched n83 control uses31.58%/16.77% fewer. All90 outputs and original witness indices match; separate polynomial arithmetic replays all60 saved tuples. Statement PCC-20261117-21fc13 is unsigned producer evidence [pcc:unverified].

| Fixed d12 query | Baseline cold Ir | Lookup cold Ir | Cold removed | Oracle removed |
|---|---:|---:|---:|---:|
| 83/planted | 31,611,467 | 29,445,307 | 6.85% | 17.60% |
| 83/uniform | 40,805,623 | 33,962,805 | 16.77% | 31.58% |
| 83/cancel | 24,910,969 | 24,860,333 | 0.20% | 0.90% |
| 131/planted | 39,974,973 | 39,214,706 | 1.90% | 5.73% |
| 131/uniform | 52,068,848 | 46,231,195 | 11.21% | 22.87% |
| 131/cancel | 32,885,116 | 32,834,844 | 0.15% | 0.81% |

This comparison uses the merged optimized kernel as baseline, with lazy context fixed in both arms. It does not reuse earlier v1 as reference or multiply ratios across studies. All18 arm/case cells and five-repeat min/median/max remain in [summary.json](summary.json). Immediate cancellation does not construct a table; its small count differences are compiled-path effects rather than table-work evidence [pcc:unverified].

- [Inspected two-page report](report.pdf), [editable source](report.typ), [instruction graph](instructions.svg), [PNG](instructions.png), [recorded graph values](plot-data.json).
- [Canonical frozen protocol](protocol.json), [narrative companion](PROTOCOL.v2.md), [freeze receipt](freeze-receipt.json), [source hashes](SOURCES.sha256).
- [Raw rows](rows.jsonl), [strict grid/raw-statistic audit](grid-audit.json), [separate polynomial arithmetic audit](independent-audit.json), [accepted export](export.json).
- [21 focused checks](checks/validation-tests.txt), [failed upstream/full-root checks](checks/root-check.txt), [pilot/resource gate](preflight-gate.json), [phase calibration](calibration.json).
- [Raw archive manifest](raw-tree-manifest.json), [archive digest](runs.tar.xz.sha256), [extraction/repack receipt](checks/archive-check.txt), [exact input hashes](input-index.json).

## One replacement, same relation problem

The existing algebraic trace-zero/subspace S3 oracle already has13 coefficient bits at d12. This change evaluates the SAME binary-linear coefficient maps in z and RHS maps in t=y+zH(z). It packs up to16 coefficient rows into four words, builds four-bit XOR combination tables and evaluates all packed rows together. RHS images have separate16-bit lanes. The quadratic computation of t is still performed normally.

Wider-than16 coefficient systems and rows after16 retain scalar parity; equation/model order, elimination, complete W membership, monic roots, supplied signs, full numeric kernel admission and original-group replay stay fixed. Tables are prepared only when the existing projector is needed, with all construction charged to the one query. Table payload is17,952 bytes at n131 and11,424 at n83, excluding existing maps/vector headers; this is not a peak-RSS measurement.

Cost chain: C_old=setup+complete_oracle+post; replace only row-map evaluation with charged lookup construction/evaluation; C_new has the SAME boundaries and answers. No removed parsing, free base/symbolic setup or batch tail. This is a component of index-calculus relation finding.

## Frozen experiment and checks

Preregistered criterion: BOTH uniform controls need>=5% fewer complete-oracle instructions and>=1% fewer complete-query instructions, every A/A discrepancy<=1%, exact outputs and no other cold cell regression>1%. All six cells pass. Six disclosed d12 queries ×three interleaved arms ×five repeats=90 primary executions; each has fresh native-tally and Callgrind processes,180 child processes total. Zero invalid, missing, excluded, interrupted or retried rows [pcc:unverified].

All21 focused checks pass, including every input basis vector plus independent random pairs at widths1/13/16, width>16 refusal, deliberately inserted16 zero leading equations forcing scalar-tail evaluation, small complete model classifications and all65 prior case outputs/indices/diagnostics. The separate Rust grid auditor recomputes ALL summary minima/medians/maxima from raw rows, checks all90 unique cells and exact phase sums, and confirms identical field ledgers [pcc:unverified].

The separate schoolbook-polynomial/Euclid checker imports no production field/group/solver code. It rechecks all65 known truths,19 negative proofs and base admissions, then replays all60 saved point tuples. The checker source is copied byte-exact into native/src/bin/check_polynomial.rs; a fresh focused rebuild reproduces the original independent-audit JSON exactly in independent-rebuild-audit.json. Prior corrupt-point/false-refutation canaries remain source-bound. These producer-written checks are not independently signed verification [pcc:unverified].

Whole Ir includes fresh input parsing, fixture/context/base/admission, every per-query preparation, residual search, roots/signs/replay/output and exit. Setup/oracle/post sums are exact; final-binary phased/unphased calibration passes0.1%. Field multiplications/squares stay separate; inversions are informational. Ir measures simulated instructions, with no native-runtime or fastest-external conclusion.

Local authorized CPU0/2GiB/600second combined final calibration/pilot/freeze/primary, zero paid resources/retries. All12 largest-class positive/uniform pilot calls pass before freeze. Pilot estimates are planning only. Source and final protocol/analysis were pushed before primary data [pcc:unverified].

## Explicit build qualification

Actual main at the frozen source has malformed SAT delimiters; broad root checking additionally fails hashed-ecdl-tensor metrics imports. The study-local facade compiles ACTUAL current numerical solver, field, curve and trace-kernel modules. It pins prior compiled SAT/chain support fromd7a006293 because those modules are imported for unused interfaces. Those support solvers are not executed in this numeric backend. BOTH arms share the same qualification. Full current-root/SAT build success is not claimed.

Initial SAT, workspace and facade failures remain in checks/. The old retained-binary discovery profile is historical; current main adds a corrected general square tally. A second profile uses fresh current arithmetic and the exact final binary. Older result/field-ledger pins are never rewritten. Original preregistration PROTOCOL.md remains byte-equal to its claim675806; PROTOCOL.v2.md changes one prose word only, with identical scientific rules.

## Identity, reproduction and custody

- Source commit: `45b0279c0280b5cb2394cf7710a07464142f42ec`.
- Protocol SHA256: `7703daa3af347a76a52bd0bc5b5587516e658ecb04c0b5c887d6b7d550aadfcf`.
- Binary SHA256: `7380fefb6583d434734d2cbffae23fb16b8a10282a9bcf5b67461687a5cc9601`.
- Original workload SHA256: `2a1712a97cb882ddd26e0a6c37a17c70a5ba9fc8fb57bb0a2d681331e0cabeb5`.

ECC2K-130 is icv1-f2m131-tm22283658519494248867-115e0dc5. The matched E0 n83 fixture is icv1-f2m83-tm6151469093347-debefd74; it is used because this model supports E0, while standard K1-83 is E1. Legacy input IDs/bytes retain exact point order. Every arm reads the same single-query payload.

Build from the recorded SOURCE in an isolated checkout, hydrate required manifests, set VALGRIND_INCLUDE to installed headers and use the locked study manifest. A rebuild of an old study against newer production is not reproduction. The pinned study-local support is explicit in native/support/ and SOURCES.sha256.

~~~sh
python3 tools/artifacts/artifacts.py pull research/trace_zero_linearized_20261115
python3 tools/artifacts/artifacts.py pull research/trace_zero_rowmap_20261117
cargo test --locked --release --manifest-path research/trace_zero_rowmap_20261117/native/Cargo.toml -- --test-threads=1
research/trace_zero_rowmap_20261117/native/target/release/trace-zero-rowmap analyze research/trace_zero_rowmap_20261117/rows.jsonl
python3 ../crypto-autoresearcher/tools/experiment_protocol.py check-export research/trace_zero_rowmap_20261117/export.json --protocol research/trace_zero_rowmap_20261117/protocol.json --verify-files
cd research/trace_zero_rowmap_20261117
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
~~~

The deterministic raw archive contains728 exact primary/pilot/final-calibration/discovery files. Every member was extracted and byte-hashed; repacking is byte-identical. Policy-sized rows/archives use the existing artifact-autofix content-addressed manifests; hydrate identical bytes before export checks. All six report/profile pages were visually inspected [pcc:unverified].

Sources checked (KB): unavailable, wrapper127. Reviewed repository S3/half-trace, symmetric subspace linear maps, Four Russians combination tables and their documented setup regressions. Established implementation technique, no novelty or Z[tau]-module claim. Native-time/external-best and independently signed review remain open. Current-main ledger has151 inherited missing-claim-type diagnostics; own claims are typed and base-prefix preservation is checked. Existing benchmark/leaderboard/runtime values stay historical; report copies are supplied without promoting an inadmissible row.

Next technical lead: reduce charged projector-map construction on queries answered after few residuals, while preserving complete tail equations and fallback semantics. No follow-up experiment has been run here.
