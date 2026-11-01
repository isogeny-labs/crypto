# Independent review of native matrix verifier replacement

Fresh offline compilation of the published Rust source reproduced all 105 ordered matrix/root rows, the 21-model set, twelve source pins and all 133 fixed external bindings. Ten producer controls and eight additional native reviewer boundary tests passed. Two actual CLI corruption tests rejected a substituted walk source and an altered binding manifest with exit 1 and accepted=false.

## Source and arithmetic fidelity

Published commit 09e64fca5552e185b24359be7283660d1604d392 is byte-bound to the copied source and Cargo files. The verifier compiles binding SHA256 8f6c33e483a6ceffe8009004470afd7699c4ccd330327804d496e7ea95966772. It checks the manifest digest before interpreting its 133 unique relative paths, then requires each file digest; it repeats these checks after computation. This is exact custody of that frozen snapshot, not general authentication of arbitrary manifests or runtime binaries.

The model set is taken from the bound historical summary and required equal to the theta/generic key sets with 21 members. Ordered action arrays require five matrices and saved scalars each. Prime p≤2^32, canonical entries, nonzero canonical scalars and distinct canonical roots are checked. Matrix squares use explicit row-column u128 dot products. Four canonical products sum below2^66, so the widened arithmetic is sufficient. The normalization uses signed i128 subtraction modulo p and extended-Euclid inversion, independently differing from the original producer's Fermat-power inversion. Nonzero canonical delta and primality make the inverse domain valid. Every normalized root and scalar agrees with the producer at the same ordered index.

All twelve historical pin paths have unique entries and locations and matching bytes. The literal nonempty linear/loader sections are equal and present in both bound full sources. Historical cached and supplied uncached walk revisions remain distinct. Full native row-fold semantics, collector binary/input provenance and geometric action meaning are not inferred.

## Independent boundary tests

Eight Rust test declarations cover a nontrivial Euclid inverse and negative root difference; inverses at prime 4294967291; zero/noncoprime inverse rejection; near-modulus diagonal matrix; a nontrivial canonical Hadamard matrix with large widened dot sums; noncanonical scalar/negative/boolean entries; composite and over-bound moduli; zero and shear matrices. These are fixed software arithmetic controls, not curve/point enumeration or a new scientific sample.

The two native CLI corruptions operate on private copied inputs. Altering walk-source.rs fails with bound input mismatch; altering the external manifest fails at its compiled digest. The original subject and all historical evidence remain unchanged. The final replay equals the initial replay exactly, and published-source hashes match before and after.

## Language correction and reviewer attestation

The preceding root review's recompute.py/control path violated the repository's native-language rule and remains historical evidence. This replacement's executed verification path is Rust, with shell-only build/run orchestration. No Python checker, curve generator, solver or cloud operation was invoked during this review. Existing render-only scripts were not used for checking.

The reviewer authored the earlier native matrix producer and has historical familiarity, but did not author this replacement verifier. Producer PLAN/REPORT/source were read before testing; this is source-first independent execution and control design, not blind derivation. The result validates the replacement's stated fixed-coefficient and custody scope. Native compiler/dependency trust remains; no formal proof of this Rust verifier is claimed.
