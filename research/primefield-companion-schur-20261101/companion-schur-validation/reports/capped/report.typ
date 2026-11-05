#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Schur panel: original cap reached
Terminal incomplete-run report · 2026-11-01T22:42:11.515704+00:00

The single Schur invocation reached its original 3,600-second cap and returned
124. Its postprocessor recorded the same terminal result and stopped before
analysis/export. Both process handles are absent. No restart or budget
extension occurred. The frozen complete cohort remains 642 context rows.

#image("coverage.svg",width:100%)
Counts derive from every complete raw JSON line. The terminal reconciliation
retains full and prefix hashes, byte counts, row IDs and all observed branches.

*Retained evidence.* 42 completed context rows contain 37 checked
Schur constructions and 5 certified leading-block obstructions,
with 456 directional product checks and
1903 rejected corruption controls. These are observations
on the completed prefix. A/A and the synthetic control cohort did not complete.
The planned 96 invertible-full/blocked-leading controls are not claimed as
executed. The frozen analyzer was not forced onto this incomplete archive.

*Coverage and custody.* 600 planned rows lack a completion receipt;
the exact number of started but unfinished contexts is not recorded. All
815,403,674 raw bytes remain, including 0 trailing partial bytes.
Eleven frozen source locks and the public input lock reverified. Predata
f5cac20; protocol 7403509a477e478f03427db64fd4c864e8e62ea99dba3f5d84f8bcd249204480.
The full source, log, native descriptors and original deadline receipts are
preserved. Previous launch reports remain historical observations.

*Completed construction examples.* These are generator-construction costs;
source application work and supplied reference-inverse work remain separate.

- Source 18, r=359, cut=179: 37,001,952 field products, 103,032 native integer products, 295,907,904 packing bytes; 322 reference leading solves and 10,317,202 dense reference terms.
- Source 27, r=479: certified leading-block obstruction; no complete Schur generator.
- Source 36, r=683, cut=341: 163,410,762 field products, 239,112 native integer products, 1,306,507,968 packing bytes; 490 reference leading solves and 56,977,690 dense reference terms.

*Next technical step.* The per-column construction invokes the full source
operator 2s+6 times. The separate companion-fused candidate defers polynomial
coefficient reduction through extraction, wrapping and accumulation. Its
larger radix and extra native arithmetic must be checked and charged before
it can replace a source operator. That candidate remains unexecuted at this
report's preparation. No change is made to this capped instrument.

*Files.* terminal-reconciliation.json; producer.log; exit-code.txt;
postprocess-state.json; postprocessor-exit.txt; runs/cases.jsonl; source-lock.json;
input-lock.json; launch.json; protocol.json. Chart data, editable report source
and renderer accompany this PDF. No full-panel export or solver improvement
is inferred from the retained prefix.
