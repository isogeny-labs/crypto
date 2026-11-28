# Many-coset component, version 1

PCC-20261116-ac25ef pins the original ten-endpoint source, configuration,
toolchain and dependency manifest. PCC-20261116-1aecf2 separately pins the
new polynomial-degree endpoint and its complete imported proof context.
Gate-v1 audits ten declarations; gate-v2 audits eleven; both have zero
failing/trusted-native axioms. All signed standing is unverified.

PCC-20261116-cc48bc pins native control/qualifier bytes, the33-input source
manifest and51784-byte source-v5-post-call archive, with native test and
strict Clippy logs. The example's source digest matches its compiled-in
pre-call digest. The locally retained executable is
/workspace/curve25519-native-binaries/many-coset-control-v1, hash
60f09a1c86f935cae27477682a48d56ef590f048a70e8092859f393d5c54c5dc.
The local binary is not an independently signed execution receipt.

PCC-20261116-69427b pins the report Markdown, inspected two-page PDF,
LaTeX source and diagram source/SVG. Its point/decomposition/family contracts
remain explicit. Full original ROUTE_CRITERIA.md and ROUTE_CRITERIA_V3.md
are in the companion consumer repository and remain unchanged.

Do not edit pinned source, reports, qualifier/control JSON, claims or archives.
Later code uses a new source/qualifier version. Later corrections preserve
this version and add an erratum/new record. Mutable README and PR prose link
new versions rather than rewriting old evidence.

Reproduction uses the standalone torus-energy manifest with the
many_coset_audit example; this is fixed correctness arithmetic, not a
benchmark. Full commands and actual logs are under development/.
One example test was added; the prior unchanged eight-case library,
identity and scaling qualification is retained with its own source snapshot.
No broad root-library or old solver rerun occurred.
