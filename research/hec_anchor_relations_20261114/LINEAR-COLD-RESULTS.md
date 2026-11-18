# Exact linear-denominator kernel: complete cold results

The full specialized rational solver improves its exact-work generic reference
by1.46..3.02x in fixture median elapsed times. Primary fixture ratios against
the fastest of14existing same-target arms are0.70..1.43x. E6correctness passes;
the pre-registered E7>=10xpilot criterion fails. This is an implementation/policy
result on a six-curve panel, not a universal limitation of rational interpolation.
Producer L0observations, pending independent replay/validation [pcc:unverified].

## Algebra and exact work

For d2, M=g+4, signed weights ui=sigma_i*yi/A'(xi), write

    H=sum_i ui*A/(x-xi), S0=sum_i ui, S1=sum_i ui*xi,
    b=S0*x-S1, a=b*H-S0^2*A.

At the root r of nonconstant b, a(r)=-S0^2*A(r). Thus common factors can be
identified by anchor probes b(xi), without generic polynomial GCD. All probe
values update by additions/subtractions in Gray order, stored at integer-sign
indices to preserve the old consumption sequence. Zero/constant denominators
are explicit cases. After common-factor/anchor removal, the monic quotient
recurrence uses only upper norm coefficients instead of a full norm division.

All2688curve/sign cases agree exactly with existing subset a,b and generic
primitive a,b,Q,masks;118common-factor and13constant-denominator cases are
covered, plus forced controls. All12previous complete-solver cases preserve
every deterministic count and scalar, with raw/projected group gates. Three
evaluated certificates in LINEAR-IDENTITIES.json cover the numerator equality:
IDC1h0ee3a91b91e99243, IDC1h6852f423d9534983, IDC1h3bbc5c0aa477bd60.
They are producer-issued/rechecked32+16finite-field evaluations, proof=null.
Primary prior art is the rational interpolation framework of D'Andrea/Krick/
Szanto, https://arxiv.org/html/1211.6895v3 . No novelty claim follows.

## Complete cold comparison

Frozen algebra protocole38d00ce0e, cold protocolce6ebaa97b, scientific source
ed0fb2c5f9822cdee862b7a8c70c38a5bf193ad3. Pre-run S3checkpoint at09:34UTC
contains protocols/source. Source commit is fixed; the record's dirty flag
records the older shared resume edit, not an uncommitted scientific kernel.
Native runner/collector pin build, full arguments and fixture SHA. New collection
and statistics use Rust/native jq. Raw archive SHA256:

    cbe13374eb5f9a7f06cb6955efefee357acfa7d6f68686ed65a96b635aceb08e

Six reused dense curves, family0, at g2p1009/16381,g3p127/1009,g4p31/127.
Three new target seed indices5/6/7 per fixture,18distinct targets; no fresh
curve holdout is claimed. Every target has16fresh-child timed arms:12native/
table controls, compact, generic rational, linear rational and linear A/A.
Arm order is rotated/reversed by the frozen schedule. Each child has cold
tables/catalogue/anchors/descents, all batch construction including unused
outputs, normalization/orientation/dedup, forest/solve and final group check.
Common fixture validation/loading and known-answer construction are excluded
equally. Extra harness verification is separate. No cross-target amortization.

| g,p | Best existing / linear median (range) | Generic / linear median | A/A max |
|---|---|---:|---:|
| 2,1009 | 1.4341 (0.3718..1.4623) | 2.0988 | 2.0899 |
| 2,16381 | 0.7504 (0.6822..0.8114) | 1.4557 | 1.5691 |
| 3,127 | 0.8390 (0.6504..0.9426) | 2.1562 | 2.1599 |
| 3,1009 | 0.6960 (0.6729..0.6989) | 2.5965 | 1.8460 |
| 4,31 | 0.9046 (0.7951..0.9646) | 2.3880 | 2.5462 |
| 4,127 | 0.7551 (0.5885..1.0060) | 3.0202 | 1.5384 |

All values are three-target observed elapsed-time ratios. Primary baseline is
the per-target minimum of14existing methods, including compact and generic;
A/Ais excluded. Compact is fastest in14targets, tableFast/branches1in3and
tableHybrid/branches1in1. Do not multiply ratios from separate runs or divide
aggregate phase medians to construct a speedup. Full target-level values,
count equality, effective anchors and constructed/consumed outputs remain in
linear-cold-evidence/summary.json and cells.jsonl.

All18full group gates and288timed cells exit0with correct scalars. Gates check
17064raw principal and600projected anchor identities. Generic/linear/A-A
target/count signatures agree on all18targets. No experimental failure, cap,
timeout or missing cell. Native runner's changed-input and timeout controls
are intentionally failing diagnostics, kept separately from the panel.
Total driver wall8.18s; summed owned child wall7.976461035s. Local ARM L0,
sequential fresh processes, no isolation assertion or statistical confidence.
A/Aspread up to2.55limits interpretation of small positive ratios.

## Decision and continuation

E6passes; E7fails on every fixture. The exact arithmetic specialization is a
useful implementation result. Its full pipeline does not support the10xclaim.
Median per-solve relation fractions remain66.4..78.3%; setup/forest/solve
optimizations alone do not address the main remaining cost. Pursue a structural
change in useful-relation generation/selection, with prior-art and a complete
cost case first. Avoid another equivalent arithmetic rewrite of this family.
The full mathematical>=10xobjective remains active.

An exploratory post-run cost diagnostic holds every non-harvesting cost of
compact fixed and sets its observed harvesting time to zero. Per target it
computes best_existing_ns/(compact_total_ns-compact_relation_ns), before any
aggregation. The maximum is3.6210831112x; fixture medians are1.5332,1.9921,
1.9464,2.0042,3.2589,3.1133. Thus a harvesting-only same-work replacement does
not supply a10xcost case under these assumptions. This is not a universal bound:
changing the relation distribution may also change downstream work. A successor
needs a joint cost case for useful harvesting and bookkeeping/recombination/
setup/algebra. Source: harvest_bound.jq; complete cases HARVEST-ONLY-BOUND.json.
This derived diagnostic is separate from the pre-registered E6/E7tests and
does not alter their raw archive, metrics or criteria.

Known related successor literature found during packaging: *Improved sieving
on algebraic curves*, https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf,
Section2.2/2.3. Its Sarkar–Singh/Joux–Vitse point sieve buckets
lambda=(y-v(x))/u(x), and the double-large-prime version uses at least g-1
small-base hits before a residual quadratic split test. This is established
prior art, not a newly discovered pencil method. No new sieve has been run.
Next: cost-screen that batch relation search against the existing compact
baseline, charging small-base construction and all bucket/forest/rank work.

## Reproduction and artifacts

Build the heldout example at sourceed0fb2c5f982with HEC_SOURCE set to that full
commit, retaining its old fifteen-arm interface and adding --linear-cold.

    HEC_SOURCE=ed0fb2c5f9822cdee862b7a8c70c38a5bf193ad3 cargo build --release --manifest-path crates/hec-native-ic/Cargo.toml --example heldout
    heldout --linear-cold NEW_ROOT ORIGINAL_HELDOUT_FIXTURE_ROOT
    HEC_SOURCE=ed0fb2c5f9822cdee862b7a8c70c38a5bf193ad3 rustc --edition=2021 -O linear_collect.rs -o linear-collect
    linear-collect NEW_ROOT > cells.jsonl
    jq -s -f linear_summary.jq cells.jsonl > summary.json

Archive paths/receipts encode the actual original root; independent fresh replay
needs its own receipts/record, not rewriting this archive. Report source and
SVG are native Rust/Typst, with both PDF pages inspected. Generic benchmark
registry/progress metadata is separate from the native scientific path.
The closed record is20261115T0930Z-hec-linear-cold. Raw S3write was denied
locally; the existing credentialed CI upload path handles the Git-retained
archive. Do not call custody complete until that object's HEAD/CI check passes.
No admitted benchmark row or leaderboard/default is promoted.
