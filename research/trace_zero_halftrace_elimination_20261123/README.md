# Exact half-trace construction for ECC2K-130 index calculus

The fixed degree131 uniform relation query uses **20.50% fewer complete cold simulated instructions**, from44,023,404 to34,999,782; the degree83 control saves15.78%. Uniform oracle instructions increase0.22% and0.11%, respectively. All90 primary outputs match. These are unsigned producer observations, claim PCC-20261123-811919. [pcc:unverified]

[Report (PDF)](report.pdf), [editable report](report.typ), [graph](instructions.svg), [150dpi PNG](instructions.png), [all six cells](results-table.md), [frozen Rust summary](summary.json).

## Construction

The existing `F2mElement::half_trace` already uses cached linear byte tables. This change accelerates their first construction. For odd n, the binary linear map `B(x)=x²+x+Tr(x)` is invertible. Tracing a kernel element gives Tr(x)=0; then x²+x=0, and B(1)=1 excludes the nonzero kernel element. With `epsilon=((n+1)/2) mod2`, the exact conventional half-trace is `H(x)=B^-1(x)+(1+epsilon)Tr(x)`. This fixes the same root on trace-one as well as trace-zero inputs.

Construct B with n basis squarings, invert one packed binary matrix and populate the existing byte tables. Production dispatch checks every polynomial term of `X^83+X^45+X²+X+1` and ECC2K's `X^131+X^13+X²+X+1`; the distinct NIST131 field is a negative dispatch control. Other moduli and singular inversions retain scalar chains. No pre-generated trace/half-trace tables are shipped. No summation-polynomial system or factor base is changed; no new Z[tau] relation-finding leverage follows.

## Requirements and evidence

| Requirement | Status and evidence |
|---|---|
| Exact half-trace root and every byte entry | Focused release44 checks, [log](checks/focused-tests-02.txt); original failed NIST fixture preserved. Producer-only. |
| Original query behavior | All65 inputs on two separate executables,130 retained [rows](equivalence-rows.jsonl). Producer-only. |
| Fixed charged comparison |90/90 completed, no exclusion/retry; [protocol](protocol.json), [raw rows](rows.jsonl), [Rust audit](grid-audit.json). Every frozen criterion passes. Producer-only. |
| Saved tuple replay | Separate schoolbook field/group implementation:92 development and60 primary tuples, [receipt](polynomial-replay.json). Same producer; not signed independent admission. |
| Evidence export/custody | [Strict export check](checks/export-check.txt), [member manifest](raw-tree-manifest.json), [extraction/hash check](checks/archive-custody.json). No remote custody asserted. |
| Full root library integration | [Exit124](checks/root-tests.exit) at600seconds compiling dependencies, before tests executed. Remains open. |
| Warmed relation-collector throughput | Not measured. Uniform oracle penalties disclosed; cold savings cannot establish warmed throughput. |
| Independent scientific admission | Pending signed verifier; no admitted leaderboard row added. |

All result/test statements in the table are unsigned producer evidence. [pcc:unverified]

## Reproduction

Numerical source `8d3905a93a5197ee0c19117f4052468bf6846c47`; protocol publication `8453eb6263d6a9e50f21e657228c877b7520487a` before primary data; protocol SHA256 `8d1b3ce420e754021767778ca5151e5188e37b0239af6fca52754a7cccfb3186`.

The [locked workspace](native/Cargo.toml) imports actual production numerical modules. Build baseline with `cargo build --locked --release --bins --features legacy-halftrace --manifest-path research/trace_zero_halftrace_elimination_20261123/native/Cargo.toml`, save that executable, then build default features for the candidate. Both builds use the same harness/compiler/non-field source. The baseline selects the exact [old field source](baseline_f2m_snapshot.rs); candidate selects production. Set `TRACE_BASELINE_EXE` to the separately saved baseline. Runtime arm guards reject incorrect labels. [Source inventory](checks/source-inputs.json) and [final identity check](checks/source-final.json) retain all observed hashes.

[Preflight script](checks/run-preflight.sh) and [primary script](checks/run-primary.sh) preserve exact original commands and resource guards. For a new reproduction use a new output directory, benchmark record and start epoch, retaining the original immutable run. Rust1.100.0; Callgrind3.24.0; CPU0;2GiB child address space;600seconds combined calibration/pilot/freeze/primary; no paid resources or retries. Both phased/unphased calibrations are below0.1%. Whole cold includes parsing, setup, cache construction, search, replay, output and exit. Compact field counters exclude general F2m initialization and do not price this change. [pcc:unverified]

[RAW-QUERIES.tar.xz](RAW-QUERIES.tar.xz) contains every original correctness, calibration, pilot and primary file. [FROZEN-BINARIES.tar.xz](FROZEN-BINARIES.tar.xz) contains both exact executables. See [manifest](raw-tree-manifest.json) for every member digest. Artifact-store publication is a separate custody gate. Baseline executable SHA256 `6038501c5a5b8c860e0dc6a4d70101355717d8d2c26a94ba5dab440e3f6b0557`; candidate `504a9c6a6ae668a0488b1e24efab7f443331c083f84f3aaee3186af24e35cef7`.

## Sources and next lead

KB unavailable: wrapper127 and no loaded MCP. Read actual cache/compact-field sources and baseline profile. [FLINT Lemma5](https://flintlib.org/doc/algorithms/multivariate/modp-factoring.html) supplies Artin–Schreier image/kernel and linearity. Normalized inversion is established linear algebra; no novelty claim. Next measure repeated compact-map construction/cache access under a separately frozen warmed-oracle boundary, addressing the observed oracle penalty. Existing earlier-study ratios are not compounded.


Artifact transport: source-freeze [run45247559161](https://github.com/isogeny-labs/crypto/actions/runs/45247559161) failed its original authenticated HeadObject operation with403 Forbidden. [Exact log](checks/artifact-autofix-source-freeze.txt), [continuing infrastructure record](../../problems/ecc2k-adjoint-artifact-store-upload.md). A403 establishes neither object absence nor custody; no bypass or identity change was attempted. [pcc:unverified]


Historical leaderboard regeneration was attempted with unchanged gates and exact restored inputs. It refuses pre-existing E27 validator tolerances; E30/E32 also fail the current tolerance contract. [Observed failures](checks/leaderboard-existing-gate-failure.json), [continuing problem](../../problems/ic-leaderboard-historical-tolerances.md). No leaderboard output or historical evidence was changed, and no new row was admitted. [pcc:unverified]
