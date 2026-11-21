# Amendment26: certified homogeneous-rank collection stop

The existing fixed-cohort2LP collector is the matched reference. Opt-in
collection_rank_stride128 checks at whole1024Bbatch boundaries, first after
at least small_base-1accepted original inputs, then after128additional
accepted inputs. At each check, reverify original Jacobian equations,
filter LPs, replay every homogeneous certificate, and compute exact rank
with the counted modular RREF. Charge every check in LinearAlgebra, retain
its full certificates/rank/counters/source prefix, and stop collection at
rank>=small_base-1. This is a collection-readiness heuristic; unchanged
affine descent, target determination and actual elliptic replay remain
mandatory. No fallback or relaxed correctness condition. [pcc:unverified]

Sources checked(KB): exact and negative incremental-LP queries unavailable;
retained validation/rank-stop-kb-search{1,2}.log. Read Joux–Vitse primary
§3.3/§6 and repository29/30evidence. Extra relations can improve structured
elimination; early stopping may instead lose useful sparsity or pay too
much for rank checks. No novelty or generic performance claim.
Primary: https://www-fourier.ujf-grenoble.fr/~viva/research/articles/CoverDecomp.pdf
[pcc:unverified]

Conditional on passing native checks and clean source/spec publication,
one fresh public weak-family instance p1009seed311,m9,75%smallbase and
1018081uniqueBmaximum. Sequential originalIC, fixed2LP, rank-stopped2LP
on the same public input and binary. The generator/held-out answer stay
outside workers; nested public schema rejects answers and Spec.d=0.
Each method returns its actual scalar, checked natively/onE and against
the held-out answer. One validation/zero warmups, CPU0/8GiB/one claimed
thread,60secondsfixture/900perworker/2800whole,no retries. Failed/censored
outcomes are retained without budget increases or unchanged reruns.
[pcc:unverified]

The original copied Rust qualifier binds fixtures/flow/status/source and
all three actual scalars; checks both final affine systems, every
checkpoint combination and independent batch rank, rank-stop/stride
criteria, counter containment and exact primitive repricing. Require the
rank-stopped original inputs to be an exact prefix of the fixed collector,
and their common descent prefix to agree. Record B/source/rank/checkpoint
work and observed phase subtotals side by side; these are incomplete
prices. No cost ratio, A/A interval, full ranking, rho parity, independent
signed verdict or canonical-schema admission. Complete totals remain
null, comparative flags false. Never compose31costs with29/30subtotals.
[pcc:unverified]

Preserve exact raw/certificates/summary/flow/argv/status/progress, copied
driver/qualifier/wrapper/source, input/preworker/prepost hashes and host
facts. Raw/logs above64KiB stay exact in local custody, hash-bound in Git;
do not truncate them. Render the report and inspect every final PDF page;
retain exact PDF/SVG/150dpiPNG browsing copies and local originals.
[pcc:unverified]

The original copied Rust qualifier also writes one self-described raw export
containing all workers, fixture, runtime protocol, flow, analysis and BLAKE3
pins to the exact original files. The canonical experiment-protocol skill
and both schemas in the sibling repository are VLT1-encrypted and unreadable
here; access receipt is retained. This native raw-export format makes no
experiment.export/v1 conformance or canonical-schema admission claim. No
Python scientific exporter/checker is used. [pcc:unverified]
