#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
= Prime m=2: exact original-top regularity
Observations only · n83-prime-m2-hilbert-20261028-r2

Six retained input identities, six producer certificates and six fresh-process
replays passed. R2 had no failure, timeout, retry, resume or exclusion. R1 failed on its first
source-template check; five cases were not reached. R1 data and partial results
remain in r1-failed.tar.xz and experiment-hilbert-r1-failed.export.json.

#table(columns: (1fr,1fr,1fr), [Dataset], [Targets], [Exact degree],
[Original fixed base], [Planted, natural], [42],
[Paired baseline], [Planted, natural], [42],
[Paired spike40], [Planted, natural], [42])

The quotient by the original highest homogeneous forms first vanishes in
degree *42*. Its degree-41 coefficient is *5348880*;
its degree-42 coefficient is zero. Total quotient dimension: *9159376218609600*.
This is not a solving degree, last fall degree or affine final-basis degree.

#image("hilbert.svg", width: 100%)
#text(size: 8pt)[Recorded exact Hilbert coefficients; symlog includes zero.
All six source certificates identify this same function.]

== What was checked
The complete saved source matches the m2 summation polynomial and 54 selector
equations. Invertible coordinate weights reduce the highest-form quotient to
54 variable squares and the product of two squared 27-variable sums.
Wilson inclusion-matrix diagonals determine each multiplication rank; tensor
images determine every bidegree coefficient. All 784 bidegrees are retained.

The prime p=4835703278761981908090881 has a verified full Lucas certificate.
Its characteristic exceeds 27. Controls passed: 168
explicit rank checks, 63 explicit product-degree
checks and 44 small-characteristic rank deficiencies.
Eight development unit tests passed.

#pagebreak()
= Scope, proof and reproducibility

== Proof scope
Lean 4.34.1 kernel-checked the finite first-zero arithmetic with no axioms.
The subset-inclusion rank theorem, invertible substitution and the polynomial
quotient/tensor-image bridge remain written mathematics, not Lean proofs.
See HILBERT_DESIGN.md and HILBERT_PROOF_MAP.json. Fresh-process replay uses
the producer implementation; independent scientific review is pending.

The six rows are saved identities, not independent random samples. Natural
source files can coincide. Original and paired planted targets differ;
paired planted tuples come from the common low26 support. The same fixed
factor-base manifest is retained within each dataset across m2 through9.

== Run identity
UTC start: 2026-10-28T16:40:30Z\
UTC finish: 2026-10-28T16:40:33Z\
Backend: existing-host fallback after retained EKS shared-bucket denial.\
Limits: one thread, 2 GiB, 120 s each producer and replay; no timing claim.

#text(size: 8pt)[Source: 64e06657f1a2e6df003130fa7f4e9a034d44b826\
Protocol: 97da83f16a1a375a486546970b5b95c3313a63f838b366d07ff504209323a32b]

== Data and replay
The unchanged inputs, executable, every coefficient, rank diagonal, primality
residue, control, log and replay receipt are preserved. The deterministic
raw archive was downloaded from S3 into a fresh directory and all sealed
file hashes checked. The source snapshot is separately retained.

#text(size: 8pt)[Raw prefix: `s3://setec-astronomy-artifacts/runs/n83-prime-m2-hilbert-20261028-r2/local/`\
Local: /home/ec2-user/n83-candidate-recovery/hilbert-r2\
Repository: research/n83_candidate_pdp_20261028/evidence/hilbert/\
Export: experiment-hilbert.export.json · Layout: HILBERT_FORMAT_V2.json\
Archive: hilbert-archives/runs.tar.xz · Storage: evidence/hilbert-storage.json]

== Remaining work
Exact binary/Koblitz regularity and larger-m LFD remain open. Prime larger-m
top quotients are positive-dimensional in the retained presentation; no finite
vanishing degree is assigned. This batch identifies no elite base and makes
no ECDLP attack, timing or universal FFD/LFD claim. All prior raw data remain.

== Primary references
#text(size: 9pt)[Ghorbani et al., Inclusion Matrices and Chains, Corollary 3:
https://arxiv.org/pdf/0709.3144.\
Phuong and Tran, A new proof of Stanley's theorem on the strong Lefschetz
property, Theorem 2.7: https://arxiv.org/pdf/2211.13548.]
