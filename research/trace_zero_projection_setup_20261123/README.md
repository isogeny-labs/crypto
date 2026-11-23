# Adjoint multiplication for complete projected S3 setup

The fixed ECC2K-130 uniform index-calculus query uses **13.82% fewer complete-oracle simulated instructions** and **6.18% fewer complete-cold-query instructions** than the current merged solver. All90 primary outputs match; separate polynomial/group arithmetic replays all60 saved tuples. Statement PCC-20261123-435311 is unsigned producer evidence. [pcc:unverified]

| Fixed d12 query | Baseline cold Ir | Adjoint cold Ir | Cold removed | Oracle removed |
|---|---:|---:|---:|---:|
| 83/planted | 29,542,434 | 28,519,674 | 3.46% | 9.79% |
| 83/uniform | 34,457,766 | 33,333,976 | 3.26% | 7.24% |
| 83/cancellation | 24,752,165 | 24,698,227 | 0.22% | 0.95% |
| 131/planted | 39,031,522 | 36,264,943 | 7.09% | 21.41% |
| 131/uniform | 46,922,548 | 44,024,513 | 6.18% | 13.82% |
| 131/cancellation | 32,452,734 | 32,399,425 | 0.16% | 0.84% |

Five repeats per cell, with the baseline A/A arm interleaved. Exact repeat ranges and every cell remain in [summary.json](summary.json). All preregistered thresholds pass in [grid-audit.json](grid-audit.json). Cancellation does not construct a projector; its small changes are compiled-path effects rather than evidence of projector work. This panel uses the current merged solver on the new Intel host; no ratios are compounded with older AMD-host studies. [pcc:unverified]

[Three-page report](report.pdf), [editable report](report.typ), [graph](instructions.svg), [150dpi PNG](instructions.png), [layout-only renderer](render-report.py), [pilot report](reports/preflight.pdf).

## Construction

The existing algebraic oracle eliminates the product variable through Artin–Schreier. It still has13 coefficient bits at d12. This change constructs exactly the same complete projected equations; it does not shrink the system or establish additional Z[tau] relation-finding leverage.

For a polynomial-basis coefficient functional `r` and multiplier `h`, the required mask `c` obeys `dot(c,z)=dot(r,h*z)`. The old constructor generates `h*X^k`, reduces each modulo W and transposes all quotient coordinates. The replacement computes each coefficient mask as `N^-1(h*N(r))`.

For `f=X^n+sum X^t`, write `R` for reversal of exactly n bits, `S(a)=XOR_{t>0}(a>>(n-t))`, and `N=(I+S)R`. Direct coefficient comparison gives `N*M_X^T=M_X*N`; polynomial substitution gives the identity for every multiplier. Since S strictly lowers degree, `N^-1=R(I+S+S²+...)`. The exact131-bit modulus needs one nonzero inverse fold and the83-bit modulus at most two. This is established transposed multiplication, not a novelty claim.

Dispatch checks the complete degree and tail polynomial, including duplicate rejection. Enabled moduli are `X^131+X^13+X²+X+1` and `X^83+X^45+X²+X+1`. Every other modulus retains the original constructor. All quotient rows, order, RHS masks, W pivots, half-traces, packed row maps, full-W membership, roots/signs, deficient-system fallback and original-group replay remain intact.

## Validation and accounting

The [focused final test log](checks/final-tests.txt) records24 passing tests, including every bilinear basis triple on both exact moduli, forward/inverse identities, all projector coefficients over five dimensions and the retained65-case answer/witness/model/diagnostic comparison. The [separate schoolbook polynomial/group checker](native/src/bin/check_polynomial.rs) checks all65 fixture admissions and negative cases, and replays the60 saved primary tuples in [polynomial-replay.json](polynomial-replay.json). Development statement PCC-20261123-faf655 and primary statement PCC-20261123-435311 are unsigned; these are producer-written checks. [pcc:unverified]

Flow:90 planned,90 attempted,90 completed,0 excluded,0 retries. Separate12-call largest-class engineering pilot and phased/unphased counter calibration precede the primary freeze. The uniform n131 thresholds were3% oracle and1% cold reduction, with every A/A discrepancy at most1% and no other cold cell regressing more than1%. [pcc:unverified]

Whole cold Ir includes input parsing, fixture/context/base setup, admission, every per-query preparation, relation search, roots/signs/replay, output, destruction and exit. The oracle partition includes all projector construction. Generator/general multiplication events share an informational tally name but have different actual costs; total Ir prices both. Field ledgers are secondary and are not falsely asserted identical.

The [full-root release build](checks/root-tests.txt) terminated with [exit124](checks/root-tests.exit) at its600second guard, while compiling dependencies; it reached no test executable. It is not a test pass. The focused facade compiles actual current numerical modules and imports the prior qualified SAT/chain snapshots only for nonexecuted interfaces. Broad root/SAT integration remains a CI/review gate. [pcc:unverified]

Scope is the standard ECC2K-130 K0-131 algebraic relation finder and an E0 degree83 control, not standard K1-83. Counts are Callgrind simulated instructions. No population, native-runtime, fastest-external or complete-DLP conclusion follows from this panel.

## Reproduction and custody

- Source: `68a0dece3c745cea3980bf0a2d383abf0f8d24fb`; [conservative source inventory](checks/source-inputs.json); [locked focused workspace](native/Cargo.lock).
- Frozen [protocol](protocol.json), [receipt](freeze-receipt.json), publication `f86b451e9ed60339021931943b8cdb02564923d7` before primary data. Protocol hash `d19325b556ce274bea13085fb969f53220551819431f7dc87a3f9a50b48fb7fd`.
- [Exact input index](input-index.json), [original workload recovery](checks/input-recovery.json), [raw rows](rows.jsonl), [accepted final export](export-final.json), [strict export check](checks/export-final-check.txt).
- [Deterministic raw archive](RAW-CALLGRIND.tar.xz), [per-file manifest](raw-tree-manifest.json), [custody receipt](checks/archive-custody.json), [compressed frozen executable](FROZEN-BINARY.xz). Large files await the repository's artifact-autofix migration, currently blocked by403; after successful remote publication, hydrate with `python3 tools/artifacts/artifacts.py pull research/trace_zero_projection_setup_20261123`.
- Rust1.100.0, Valgrind3.24.0, Intel Xeon Platinum8573C. CPU0 and2GiB child address-space limit; combined calibration/pilot/freeze/primary guard600seconds, zero retries or paid resources. Native elapsed pilot values are resource-planning observations only.

Build with `cargo build --locked --release --manifest-path research/trace_zero_projection_setup_20261123/native/Cargo.toml`. The executable's `one INPUT adjoint` and `one INPUT baseline` commands reproduce the two numeric paths. `analyze rows.jsonl` recomputes the frozen Rust summary; `audit_adjoint rows.jsonl summary.json` checks every grid cell and decision criterion. The standalone `check_polynomial` checker takes the original65-case workload followed by any recorded row files. A new measurement needs its own frozen record; the retained controller intentionally refuses to reuse an expired600second envelope.

Initial export adapter attempts omitted explicit instance mapping/instrument metadata and then used the wrong note shape. They and their error reports are preserved. `export-final.json` supplies the administrative fields before serialization and passes the canonical strict check; no scientific row or frozen analysis changed. The first freeze call rejected an abbreviated commit; the full-commit freeze succeeded and was published before any primary data. [pcc:unverified]

Sources checked (KB): unavailable, wrapper127, no loaded MCP. Repository S3/word-field/rowmap sources; De Feo, Doliskani and Schost, *Fast arithmetic for the algebraic closure of finite fields*, ISSAC2014,section2 on dual maps and transposition ([author PDF](https://cs.uwaterloo.ca/~eschost/publications/embed.pdf)). Current canonical protocol files were VLT1 encrypted; the exact last plaintext tool revision `0c76edb5` was recovered from Git, with [provenance](checks/protocol-tool.json). No novelty claim.

The next independent setup optimization is to reuse the already-constructed compact half-trace/square maps when forming the h_i columns, rather than recomputing general-element half-traces. This is a proposed follow-up, not part of the measured replacement. Existing admitted benchmark/runtime/leaderboard rows remain historical; these report copies do not promote an inadmissible row.

## Publication status

PR5015 is ready for review with normal auto-merge enabled subject to gates. The original artifact-autofix run45213255631 failed authenticated HeadObject requests with403. Whole original payloads remain preserved in the PR; no remote upload success is claimed. [Exact obstruction](../../problems/ecc2k-adjoint-artifact-store-upload.md) and full logs retain the unresolved migration. The final root-build gate also remains pending after its recorded timeout. [pcc:unverified]
