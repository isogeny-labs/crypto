#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
= Fixed-domain radix presentation census
Runs n83-radix-census-20261030-r1/r2 · T-1008 · 30 October 2026

The native census covers all 3,010 nondecreasing partitions of 27. It records 35 exact Hilbert calculations and 2,975 rigorous lower bounds. The minimum exact first graded vanishing degree is 42, attained once by the original all-binary presentation. Every bounded record has a certified lower bound of at least 43. Thus no presentation in this specified same-domain family lowers the original degree-42 value, under the stated highest-form model and rank theorem.

== Same points, different equations
A partition part k gives a radix-2^k digit and a product-of-linear-factors digit constraint. Mixed-radix weights map the digit domain bijectively onto the original interval [0,2^27), below the field modulus. The actual prime point base, its manifest, curve and lift convention remain fixed. The native validator checks the entire original prime m2 natural source. Candidate equations are retained as an exact compact product-range specification; expanded enormous-degree equations are not claimed.

After weight normalization, the highest-form quotient is (A tensor A)/(L0² L1²), with A defined by digit-power monomials and selector-block socle degree D. For D at most 40, the characteristic exceeds D; the Phuong–Tran Theorem 3.1 supplies the maximal multiplication ranks, and the native instrument retains complete Hilbert/rank arrays. For D above 40, the nonzero component of bidegree (D,1) gives first vanishing degree at least D+2; exact degree stays null. The baseline result is reproduced from data, not used to select a winner.

#image("radix-degrees.svg", width: 100%)
#text(size: 8pt)[All 3,010 native rows; overlapping points remain separate records. Triangles are lower bounds, not exact degrees. Log axes show the complete range. No statistical interval or timing comparison applies.]

== Verification and execution
The local producer and fresh native replay both exited 0. After S3 restoration, all native payloads were reconstructed and byte-compared: 3,015 payloads verified. The whole-worker manifest also passes; exact counts/hashes are in evidence/radix-census-r2/custody.json. The corrected unified export passes protocol and file verification. Preproduction validation retained 1,098 explicit small matrix ranks, 1,042 encoding checks and 39 expected small-characteristic rank failures; final native and container suites each passed 15 tests.

R2 ran on the existing shared host with 120-second phase caps, 1 GiB address-space caps and a 300-second worker cap. Worker timestamps are 22:02:03–22:03:44 UTC; wall_valid is false and no performance result is inferred. Source: `1e5269c772dfff228fb07c391c03737466f63d8d`. Protocol v2: `994b14318ad32876c7c4a9ebb79a9e3e6fcaed539cd1d518d138195f67dc354d`.

== Preserved first attempt and accounting correction
R1 was submitted once to EKS at 21:51:04 UTC and reached DeadlineExceeded at 22:01:04 UTC without an observed worker start or arithmetic output. Its empty logs/listings and terminal receipt remain. Frozen v1 also incorrectly encoded 3,010 total records as per-arm instances, yielding 6,020 schema slots. Its intended-coverage export retains the four accounting-check failures. V2 corrected this before arithmetic data: two execution phases, each processing the identical 3,010 records. No extra trials were invented, and no candidate was omitted.

== Artifacts and next construction
Raw prefix: `s3://setec-astronomy-artifacts/runs/n83-radix-census-20261030-r2/raw/`. See summary.json, exact-cases.json, observations.json and custody.json under evidence/radix-census-r2; RADIX_CENSUS_FORMAT.json specifies fields and replay. Original r1 remains under evidence/radix-census-r1. The source theorem is `https://arxiv.org/abs/2211.13548`; the tensor-image derivation is in HILBERT_DESIGN.md.

This calculation concerns prime m2 original-top vanishing degree. It does not compute affine FFD/LFD or higher-m regularity. The next base-quality construction must change the candidate-domain geometry or another feature outside the tested nonzero-reweighting and mixed-radix families; the current point dataset stays available unchanged.
