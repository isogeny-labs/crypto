#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm), numbering: "1 / 1")
#set text(size: 8.4pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.6pt)
#show table: set text(size: 6.9pt)
#show raw: set text(size: 7.2pt)

= Run report: teske\_variants\_field\_types, run 20261018T0845Z

Descriptive record of one run of `examples/teske_variants_field_types.rs`
(protocol `research/teske_variants_field_types_20261018/PROTOCOL.md`, frozen
at commit `62b4502c7`). It lists what ran, on what host, and the values the
tool recorded, with their files. Interpretation is in `REPORT.md` of the
study. Nothing here is a running time of an attack; the only timings are
the tool's own wall-clock seconds per part, from `tool_log.txt`.

== Command, host and isolation
```
run: 20261018T0845Z (Part C only)
protocol commit: 62b4502c7
why a second run: run-20261018T0630Z completed Parts A and B and Part C for q <= 11, then the session's worker process was restarted and the tool was killed during q = 13 (see its NOTE.md). Before this run the census root search over F_{q^6} (a scan of the field) was replaced by Cantor-Zassenhaus splitting with polynomial arithmetic over the same table field; the census files for q <= 11 are byte-identical between the two tools (verified with cmp before this run), and this run supplies Part C for all q.
rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v139 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.80GHz, 15 GB
command: rustc --edition 2021 -O examples/teske_variants_field_types.rs -o tvft && ./tvft research/teske_variants_field_types_20261018/results/run-20261018T0845Z C
tool sha256: 98ff1e88eea94c083d694abae25375f3df924280ea860d9b655392c96f705a10 (rustfmt-formatted; the committed file)
threads: 4 (std::thread::scope)
```
Isolation: none earned (a shared cloud container, 4 threads, no pinning);
the tool is exact, so isolation affects only the wall-clock figures below.
== Tool log (wall-clock per part)
```
part C q=3 models=5832 res_shaped=152 classes=21 sig=6 nosig=15 na=0 enum=0.0s total=0.0s
part C q=5 models=187500 res_shaped=3088 classes=158 sig=20 nosig=138 na=0 enum=0.1s total=0.2s
part C q=7 models=3294172 res_shaped=48352 classes=538 sig=39 nosig=499 na=0 enum=1.8s total=3.1s
part C q=11 models=42517464 res_shaped=293440 classes=3173 sig=106 nosig=3067 na=0 enum=49.0s total=62.6s
part C q=13 models=135150652 res_shaped=682472 classes=5582 sig=160 nosig=5422 na=0 enum=195.1s total=231.5s
elapsed 297.6s
```
== Part A: not part of this run

== Part B: not part of this run

== Part C: census (`census.csv`, `census_summary.csv`, `census_patterns.csv`, `census_checks.txt`)

#table(columns: 15, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*q*], [*models*], [*squarefree*], [*Res-shaped models*], [*classes*], [*signature*], [*no signature*], [*roots outside F\_q^6*], [*\#E prime*], [*prime, no sig.*], [*t subfield-shaped*], [*of which sig.*], [*traces*], [*traces with sig.*], [*s*],
[3], [5832], [3888], [152], [21], [6], [15], [0], [0], [0], [7], [0], [9], [2], [0.0],
[5], [187500], [150000], [3088], [158], [20], [138], [0], [0], [0], [70], [8], [21], [10], [0.2],
[7], [3294172], [2823576], [48352], [538], [39], [499], [0], [0], [0], [92], [1], [37], [9], [3.1],
[11], [42517464], [38652240], [293440], [3173], [106], [3067], [0], [0], [0], [547], [6], [73], [16], [62.6],
[13], [135150652], [124754448], [682472], [5582], [160], [5422], [0], [0], [0], [1042], [28], [93], [36], [231.5]
)

Classes by Weierstrass cycle type and signature:

#table(columns: 4, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*q*], [*cycle type*], [*signature*], [*classes*],
[3], [1+1+3+3], [no], [2],
[3], [1+1+3+3], [yes], [4],
[3], [1+1+6], [no], [11],
[3], [2+6], [no], [2],
[3], [2+6], [yes], [2],
[5], [1+1+3+3], [no], [30],
[5], [1+1+3+3], [yes], [12],
[5], [1+1+6], [no], [76],
[5], [2+3+3], [no], [12],
[5], [2+6], [no], [20],
[5], [2+6], [yes], [8],
[7], [1+1+3+3], [no], [116],
[7], [1+1+3+3], [yes], [19],
[7], [1+1+6], [no], [211],
[7], [2+3+3], [no], [38],
[7], [2+6], [no], [134],
[7], [2+6], [yes], [20],
[11], [1+1+3+3], [no], [832],
[11], [1+1+3+3], [yes], [54],
[11], [1+1+6], [no], [1168],
[11], [2+3+3], [no], [264],
[11], [2+6], [no], [803],
[11], [2+6], [yes], [52],
[13], [1+1+3+3], [no], [1264],
[13], [1+1+3+3], [yes], [92],
[13], [1+1+6], [no], [2198],
[13], [2+3+3], [no], [528],
[13], [2+6], [no], [1432],
[13], [2+6], [yes], [68]
)

Checks (`census_checks.txt`):
```
q=3 models=5832 squarefree=3888 res_shaped_models=152 classes=21 neighbours_missing=0
q=3 check8_hasse_violations=0 t_inconsistent_in_class=0 check9_signature_numE_mod4_violations=0 check9_signature_cycle_violations=0
q=5 models=187500 squarefree=150000 res_shaped_models=3088 classes=158 neighbours_missing=0
q=5 check8_hasse_violations=0 t_inconsistent_in_class=0 check9_signature_numE_mod4_violations=0 check9_signature_cycle_violations=0
q=7 models=3294172 squarefree=2823576 res_shaped_models=48352 classes=538 neighbours_missing=0
q=7 check8_hasse_violations=0 t_inconsistent_in_class=0 check9_signature_numE_mod4_violations=0 check9_signature_cycle_violations=0
q=11 models=42517464 squarefree=38652240 res_shaped_models=293440 classes=3173 neighbours_missing=0
q=11 check8_hasse_violations=0 t_inconsistent_in_class=0 check9_signature_numE_mod4_violations=0 check9_signature_cycle_violations=0
q=13 models=135150652 squarefree=124754448 res_shaped_models=682472 classes=5582 neighbours_missing=0
q=13 check8_hasse_violations=0 t_inconsistent_in_class=0 check9_signature_numE_mod4_violations=0 check9_signature_cycle_violations=0
```

== Graphs
Recorded values only, drawn by `figures/plot.py` from the CSVs of the two recorded runs (Parts A and B from run 20261018T0630Z, Part C from run 20261018T0845Z).

#figure(image("../../figures/oddchar_counts.svg", width: 100%), caption: [Weak j-invariants per branch type divided by q^(r−3); exact counts, no uncertainty.])
#figure(image("../../figures/oddchar_classes.svg", width: 100%), caption: [Share of curves in isogeny classes with a weak member, and weak members per covered class; exact, from the all-j trace tables.])
#figure(image("../../figures/census.svg", width: 70%), caption: [Res-shaped genus-3 hyperelliptic classes over F_q by GHS signature; exact census.])
#figure(image("../../figures/margins.svg", width: 100%), caption: [Margins a = δ − n/2 by n: binary (exact cosets) and odd characteristic (smallest-genus type).])
