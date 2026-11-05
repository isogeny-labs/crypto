#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Native radix presentation controls
Development component · T-1008 · 30 October 2026

The native Rust controls checked 29 partitions of bit widths 1 through 6, with 1,042 exhaustive mixed-radix encoding checks and 1,098 explicit multiplication-matrix checks. The final executable passed four invalid-input rejection tests. Fresh execution and execution from the S3-restored executable reproduced controls.json byte for byte.

== Construction
Regrouping Boolean selectors into radix digits changes the polynomial presentation while preserving the candidate x interval. A radix-r digit satisfies the product of (d-a) for a=0,...,r-1. Its highest form is d^r. For prime m2 the candidate quotient is a tensor product of two such selector algebras, modulo the product of the squared linear coordinate forms. The production census will preserve the original prime point base and lift convention.

The controls enumerate bounded-exponent monomials and expand every ordered variable pair in multiplication by L². Gaussian elimination computes the ranks independently of the Hilbert-coefficient convolution. Over F101, the characteristic exceeds every tested socle degree, and all applicable maximal-rank assertions passed. Across F2 and F3, 39 maps were nonmaximal; these are expected negative-control records, not discarded failures.

#image("rank-controls.svg", width: 100%)
#text(size: 8pt)[Counts grouped from the 1,098 recorded rows in validation-radix-development/controls.json. Fixed exhaustive controls; no statistical uncertainty or timing comparison. Run r2, with identical fresh and S3-restored repetitions.]

== Coverage and retained data
Each row records bit partition, digit radices, field, socle and target degrees, source/target dimensions, Gaussian rank, maximal rank and whether the large-characteristic hypothesis holds. Tests reject empty, zero, oversized-part and oversized-total inputs. Commands: cargo test --release --bin prime_radix_controls, then prime_radix_controls FRESH_OUTPUT. The standalone study Cargo manifest is used. Runs occurred as local development checks; no measured performance claim is made.

Both r1 and final r2 outputs, executables and logs are retained. R2 adds input validation and four rejection tests; r1 source was reconstructed by reversing those changes and is explicitly labelled reconstructed. The accepted r2 has exact source and binary snapshots. All final matrix/domain controls passed; no unexpected failure occurred. The width27 production census has not run.

== Mathematical dependency and next step
Phuong–Tran, Theorem 3.1 (arxiv.org/abs/2211.13548), supplies the large-characteristic monomial-complete-intersection rank theorem. Production must check its characteristic condition and bind the exact original m2 source and point manifest. Then enumerate all digit partitions of 27, retaining exact Hilbert arrays for small socle degrees and explicit lower-bound records for the larger ones. No affine FFD/LFD or higher-m regularity conclusion is taken from these development controls.

== Custody
Raw archive: `s3://setec-astronomy-artifacts/runs/n83-radix-development-20261030/raw.tar.gz`. The archive was downloaded and compared byte for byte, and its restored native executable regenerated the accepted controls. Exact hashes and byte counts are in validation-radix-development/custody.json; RADIX_DEVELOPMENT_FORMAT.json describes the records. The centered-panel production job is separate and unchanged.
