#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm), numbering: "1 / 1")
#set text(size: 8.4pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.6pt)
#show table: set text(size: 6.9pt)
#show raw: set text(size: 7.2pt)

= Run report: teske\_variants\_field\_types, run 20261018T0630Z

Descriptive record of one run of `examples/teske_variants_field_types.rs`
(protocol `research/teske_variants_field_types_20261018/PROTOCOL.md`, frozen
at commit `62b4502c7`). It lists what ran, on what host, and the values the
tool recorded, with their files. Interpretation is in `REPORT.md` of the
study. Nothing here is a running time of an attack; the only timings are
the tool's own wall-clock seconds per part, from `tool_log.txt`.

== Command, host and isolation
```
run: 20261018T0630Z
protocol commit: 62b4502c7
working tree: 62b4502c7 plus the uncommitted tool (committed with the results)
rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v139 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.80GHz, 15 GB
command: rustc --edition 2021 -O examples/teske_variants_field_types.rs -o tvft && ./tvft research/teske_variants_field_types_20261018/results/run-20261018T0630Z A B C
tool sha256: 437ebf9244ad1b8b42242531fea097588b4b310b326515c80110a35ae96c1435
threads: 4 (std::thread::scope)

note 1: the tool file was reformatted with rustfmt after this run started (whitespace only; sha256 of the formatted file at that point 4dd68f202b057ffd...); Part A output of the formatted binary was compared with these files and is identical. The committed file further replaces the census root search (see the next run directory), which does not touch Parts A and B.
note 2: the session worker process was restarted while Part C was at q = 13 and this tool process was killed. Parts A and B are complete here (29 part lines in tool_log.txt, every (q, n) of the protocol); the census files hold q = 3, 5, 7, 11 and are superseded by the next run directory, whose q <= 11 census files are byte-identical to these.
```
Isolation: none earned (a shared cloud container, 4 threads, no pinning);
the tool is exact, so isolation affects only the wall-clock figures below.
== Tool log (wall-clock per part)
```
part A done
elapsed 0.0s
part B q=3 n=2 L=6 types=5 enum=0.0s traces=0.0s all_j=true
part B q=5 n=2 L=6 types=5 enum=0.0s traces=0.0s all_j=true
part B q=7 n=2 L=6 types=5 enum=0.0s traces=0.0s all_j=true
part B q=11 n=2 L=6 types=5 enum=0.4s traces=0.0s all_j=true
part B q=13 n=2 L=6 types=5 enum=1.2s traces=0.0s all_j=true
part B q=3 n=3 L=6 types=7 enum=0.0s traces=0.0s all_j=true
part B q=5 n=3 L=6 types=7 enum=0.0s traces=0.0s all_j=true
part B q=7 n=3 L=6 types=7 enum=0.0s traces=0.0s all_j=true
part B q=11 n=3 L=6 types=7 enum=0.8s traces=0.0s all_j=true
part B q=13 n=3 L=6 types=7 enum=2.5s traces=0.1s all_j=true
part B q=3 n=4 L=4 types=3 enum=0.0s traces=0.0s all_j=true
part B q=5 n=4 L=4 types=3 enum=0.0s traces=0.0s all_j=true
part B q=7 n=4 L=4 types=3 enum=0.0s traces=0.0s all_j=true
part B q=11 n=4 L=4 types=3 enum=0.3s traces=1.4s all_j=true
part B q=13 n=4 L=4 types=3 enum=0.9s traces=3.3s all_j=true
part B q=3 n=5 L=5 types=1 enum=0.0s traces=0.0s all_j=true
part B q=5 n=5 L=5 types=1 enum=0.0s traces=0.0s all_j=true
part B q=7 n=5 L=5 types=1 enum=0.3s traces=1.1s all_j=true
part B q=11 n=5 L=5 types=1 enum=6.8s traces=111.8s all_j=true
part B q=13 n=5 L=5 types=1 enum=19.3s traces=645.0s all_j=true
part B q=3 n=6 L=6 types=2 enum=0.0s traces=0.0s all_j=true
part B q=5 n=6 L=6 types=2 enum=0.0s traces=1.0s all_j=true
part B q=7 n=6 L=6 types=2 enum=0.1s traces=58.8s all_j=true
part B q=11 n=6 L=6 types=2 enum=1.0s traces=26.8s all_j=false
part B q=13 n=6 L=6 types=2 enum=2.6s traces=182.6s all_j=false
part B q=3 n=7 L=7 types=1 enum=0.0s traces=0.0s all_j=true
part B q=5 n=7 L=7 types=1 enum=0.0s traces=25.6s all_j=true
part B q=7 n=7 L=7 types=1 enum=0.4s traces=19.8s all_j=false
elapsed 1114.5s
```
== Part A: binary fields (`binary_types.csv`, `binary_gghs.csv`, `binary_summary.csv`, `binary_bits.csv`)

Rows in `binary_types.csv`: 766; in `binary_gghs.csv`: 541; in `binary_bits.csv`: 164. Every value is an exact count or a formula on the cyclotomic cosets of 2 modulo n; the per-class expectations are the GHS 2002 §5 formula. Selected rows of the summary (smallest genus per variant; δ = n − deg F; margin a = δ − n/2):

#table(columns: 9, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*n*], [*ord*], [*cosets*], [*type I (deg,g,δ)*], [*a\_I*], [*type II (deg,g,δ)*], [*a\_II*], [*gGHS (r,s,g,δ)*], [*a\_gGHS*],
[3], [2], [2], [(2, 3, 1)], [-0.5], [(3, 4, 0)], [-1.5], [(2, 2, 3, 0)], [-1.5],
[5], [4], [2], [(4, 15, 1)], [-1.5], [(5, 16, 0)], [-2.5], [(1, 4, 15, 1)], [-1.5],
[7], [3], [3], [(3, 7, 4)], [0.5], [(4, 8, 3)], [-0.5], [(3, 3, 7, 2)], [-1.5],
[9], [6], [3], [(6, 63, 3)], [-1.5], [(7, 64, 2)], [-2.5], [(1, 6, 63, 3)], [-1.5],
[15], [4], [5], [(4, 15, 11)], [3.5], [(5, 16, 10)], [2.5], [(4, 4, 15, 8)], [0.5],
[17], [8], [3], [(8, 255, 9)], [0.5], [(9, 256, 8)], [-0.5], [(8, 8, 255, 2)], [-6.5],
[21], [6], [6], [(5, 31, 16)], [5.5], [(6, 32, 15)], [4.5], [(2, 3, 21, 17)], [6.5],
[23], [11], [3], [(11, 2047, 12)], [0.5], [(12, 2048, 11)], [-0.5], [(11, 11, 2047, 2)], [-9.5],
[31], [5], [7], [(5, 31, 26)], [10.5], [(6, 32, 25)], [9.5], [(5, 5, 31, 22)], [6.5],
[33], [10], [5], [(10, 1023, 23)], [6.5], [(11, 1024, 22)], [5.5], [(10, 10, 1023, 14)], [-2.5],
[35], [12], [6], [(7, 127, 28)], [10.5], [(8, 128, 27)], [9.5], [(3, 4, 105, 29)], [11.5],
[45], [12], [8], [(10, 1023, 35)], [12.5], [(11, 1024, 34)], [11.5], [(4, 6, 945, 36)], [13.5],
[63], [6], [13], [(6, 63, 57)], [25.5], [(7, 64, 56)], [24.5], [(6, 6, 63, 52)], [20.5],
[93], [10], [14], [(7, 127, 86)], [39.5], [(8, 128, 85)], [38.5], [(2, 5, 93, 87)], [40.5]
)

Bit table rows for N = 155 and 161 (all variants except `generic`):

#table(columns: 12, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*N*], [*n*], [*l*], [*variant*], [*g*], [*δ*], [*rho*], [*owner*], [*walk*], [*margin*], [*log2 \#weak*], [*per class*],
[155], [5], [31], [GHS-I], [15], [1], [77.5], [57.9], [31.0], [-46.5], [125.0], [1.000e0],
[155], [5], [31], [GHS-II], [16], [0], [77.5], [58.1], [0.0], [-77.5], [156.0], [1.000e0],
[155], [5], [31], [gGHS], [15], [1], [77.5], [57.9], [31.0], [-46.5], [126.0], [1.000e0],
[155], [31], [5], [GHS-I], [31], [26], [77.5], [9.7], [77.5], [52.5], [28.6], [6.078e-18],
[155], [31], [5], [GHS-II], [32], [25], [77.5], [9.7], [77.5], [47.5], [33.6], [1.945e-16],
[155], [31], [5], [gGHS], [31], [22], [77.5], [9.7], [77.5], [32.5], [48.6], [6.373e-12],
[161], [7], [23], [GHS-I], [7], [4], [80.5], [39.4], [80.5], [11.5], [71.0], [4.289e-6],
[161], [7], [23], [GHS-II], [8], [3], [80.5], [40.2], [69.0], [-11.5], [94.0], [1.000e0],
[161], [7], [23], [gGHS], [7], [2], [80.5], [39.4], [46.0], [-34.5], [117.0], [1.000e0],
[161], [23], [7], [GHS-I], [2047], [12], [80.5], [14.0], [80.5], [3.5], [79.0], [1.098e-3],
[161], [23], [7], [GHS-II], [2048], [11], [80.5], [14.0], [77.0], [-3.5], [86.0], [1.405e-1],
[161], [23], [7], [gGHS], [2047], [2], [80.5], [14.0], [14.0], [-66.5], [149.0], [1.000e0]
)

== Part B: odd characteristic (`oddchar_types.csv`, `oddchar_classes.csv`, `oddchar_checks.txt`)

Rows (one per (q, n, branch type)): 93. instances\_B is the number of weak branch sets (pairs (E, x)); weak\_j the number of distinct j-invariants; ratio = weak\_j / q^(r−3).

#table(columns: 13, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*q*], [*n*], [*L*], [*orbits*], [*r*], [*m*], [*g*], [*instances B*], [*weak j*], [*log\_q*], [*pred.*], [*ratio*], [*all-j*],
[3], [2], [6], [2+1+1+1], [5], [2], [2], [24], [1], [0.0000], [2], [0.1111], [true],
[3], [2], [6], [2+2+1], [5], [2], [2], [48], [1], [0.0000], [2], [0.1111], [true],
[3], [2], [6], [2+2+1+1], [6], [2], [3], [72], [2], [0.6309], [3], [0.0741], [true],
[3], [2], [6], [2+2+2], [6], [2], [3], [12], [1], [0.0000], [3], [0.0370], [true],
[3], [2], [6], [3+2], [5], [2], [2], [48], [2], [0.6309], [2], [0.2222], [true],
[5], [2], [6], [2+1+1+1], [5], [2], [2], [400], [4], [0.8614], [2], [0.1600], [true],
[5], [2], [6], [2+2+1], [5], [2], [2], [1080], [4], [0.8614], [2], [0.1600], [true],
[5], [2], [6], [2+2+1+1], [6], [2], [3], [2700], [5], [1.0000], [3], [0.0400], [true],
[5], [2], [6], [2+2+2], [6], [2], [3], [1440], [5], [1.0000], [3], [0.0400], [true],
[5], [2], [6], [3+2], [5], [2], [2], [800], [7], [1.2091], [2], [0.2800], [true],
[7], [2], [6], [2+1+1+1], [5], [2], [2], [2352], [7], [1.0000], [2], [0.1429], [true],
[7], [2], [6], [2+2+1], [5], [2], [2], [6720], [9], [1.1292], [2], [0.1837], [true],
[7], [2], [6], [2+2+1+1], [6], [2], [3], [23520], [9], [1.1292], [3], [0.0262], [true],
[7], [2], [6], [2+2+2], [6], [2], [3], [15960], [9], [1.1292], [3], [0.0262], [true],
[7], [2], [6], [3+2], [5], [2], [2], [4704], [14], [1.3562], [2], [0.2857], [true],
[11], [2], [6], [2+1+1+1], [5], [2], [2], [24200], [19], [1.2279], [2], [0.1570], [true],
[11], [2], [6], [2+2+1], [5], [2], [2], [71280], [20], [1.2493], [2], [0.1653], [true],
[11], [2], [6], [2+2+1+1], [6], [2], [3], [392040], [21], [1.2697], [3], [0.0158], [true],
[11], [2], [6], [2+2+2], [6], [2], [3], [314820], [21], [1.2697], [3], [0.0158], [true],
[11], [2], [6], [3+2], [5], [2], [2], [48400], [37], [1.5059], [2], [0.3058], [true],
[13], [2], [6], [2+1+1+1], [5], [2], [2], [56784], [26], [1.2702], [2], [0.1538], [true],
[13], [2], [6], [2+2+1], [5], [2], [2], [168168], [29], [1.3128], [2], [0.1716], [true],
[13], [2], [6], [2+2+1+1], [6], [2], [3], [1093092], [29], [1.3128], [3], [0.0132], [true],
[13], [2], [6], [2+2+2], [6], [2], [3], [912912], [29], [1.3128], [3], [0.0132], [true],
[13], [2], [6], [3+2], [5], [2], [2], [113568], [52], [1.5405], [2], [0.3077], [true],
[3], [3], [6], [3+1+1], [5], [3], [3], [144], [3], [1.0000], [2], [0.3333], [true],
[3], [3], [6], [3+1+1+1], [6], [3], [5], [96], [4], [1.2619], [3], [0.1481], [true],
[3], [3], [6], [3+2], [5], [3], [3], [72], [3], [1.0000], [2], [0.3333], [true],
[3], [3], [6], [3+2+1], [6], [3], [5], [288], [12], [2.2619], [3], [0.4444], [true],
[3], [3], [6], [3+3], [6], [2], [3], [252], [4], [1.2619], [3], [0.1481], [true],
[3], [3], [6], [3+3], [6], [3], [5], [168], [4], [1.2619], [3], [0.1481], [true],
[3], [3], [6], [6], [6], [2], [3], [348], [5], [1.4650], [3], [0.1852], [true],
[5], [3], [6], [3+1+1], [5], [3], [3], [1800], [12], [1.5440], [2], [0.4800], [true],
[5], [3], [6], [3+1+1+1], [6], [3], [5], [2400], [20], [1.8614], [3], [0.1600], [true],
[5], [3], [6], [3+2], [5], [3], [3], [1200], [10], [1.4307], [2], [0.4000], [true],
[5], [3], [6], [3+2+1], [6], [3], [5], [7200], [60], [2.5440], [3], [0.4800], [true],
[5], [3], [6], [3+3], [6], [2], [3], [7020], [21], [1.8917], [3], [0.1680], [true],
[5], [3], [6], [3+3], [6], [3], [5], [4680], [21], [1.8917], [3], [0.1680], [true],
[5], [3], [6], [6], [6], [2], [3], [7740], [21], [1.8917], [3], [0.1680], [true],
[7], [3], [6], [3+1+1], [5], [3], [3], [9408], [25], [1.6542], [2], [0.5102], [true],
[7], [3], [6], [3+1+1+1], [6], [3], [5], [18816], [56], [2.0686], [3], [0.1633], [true],
[7], [3], [6], [3+2], [5], [3], [3], [7056], [21], [1.5646], [2], [0.4286], [true],
[7], [3], [6], [3+2+1], [6], [3], [5], [56448], [168], [2.6332], [3], [0.4898], [true],
[7], [3], [6], [3+3], [6], [2], [3], [55944], [58], [2.0867], [3], [0.1691], [true],
[7], [3], [6], [3+3], [6], [3], [5], [37296], [57], [2.0777], [3], [0.1662], [true],
[7], [3], [6], [6], [6], [2], [3], [58632], [58], [2.0867], [3], [0.1691], [true],
[11], [3], [6], [3+1+1], [5], [3], [3], [87120], [60], [1.7075], [2], [0.4959], [true],
[11], [3], [6], [3+1+1+1], [6], [3], [5], [290400], [220], [2.2493], [3], [0.1653], [true],
[11], [3], [6], [3+2], [5], [3], [3], [72600], [55], [1.6712], [2], [0.4545], [true],
[11], [3], [6], [3+2+1], [6], [3], [5], [871200], [660], [2.7075], [3], [0.4959], [true],
[11], [3], [6], [3+3], [6], [2], [3], [869220], [221], [2.2512], [3], [0.1660], [true],
[11], [3], [6], [3+3], [6], [3], [5], [579480], [222], [2.2531], [3], [0.1668], [true],
[11], [3], [6], [6], [6], [2], [3], [885060], [221], [2.2512], [3], [0.1660], [true],
[13], [3], [6], [3+1+1], [5], [3], [3], [198744], [82], [1.7181], [2], [0.4852], [true],
[13], [3], [6], [3+1+1+1], [6], [3], [5], [794976], [364], [2.2991], [3], [0.1657], [true],
[13], [3], [6], [3+2], [5], [3], [3], [170352], [78], [1.6986], [2], [0.4615], [true],
[13], [3], [6], [3+2+1], [6], [3], [5], [2384928], [1092], [2.7274], [3], [0.4970], [true],
[13], [3], [6], [3+3], [6], [2], [3], [2381652], [363], [2.2981], [3], [0.1652], [true],
[13], [3], [6], [3+3], [6], [3], [5], [1587768], [366], [2.3013], [3], [0.1666], [true],
[13], [3], [6], [6], [6], [2], [3], [2412228], [366], [2.3013], [3], [0.1666], [true],
[3], [4], [4], [4+1], [5], [4], [5], [288], [7], [1.7712], [2], [0.7778], [true],
[3], [4], [4], [4+1+1], [6], [3], [5], [432], [13], [2.3347], [3], [0.4815], [true],
[3], [4], [4], [4+2], [6], [3], [5], [216], [9], [2.0000], [3], [0.3333], [true],
[5], [4], [4], [4+1], [5], [4], [5], [3600], [27], [2.0478], [2], [1.0800], [true],
[5], [4], [4], [4+1+1], [6], [3], [5], [9000], [63], [2.5743], [3], [0.5040], [true],
[5], [4], [4], [4+2], [6], [3], [5], [6000], [50], [2.4307], [3], [0.4000], [true],
[7], [4], [4], [4+1], [5], [4], [5], [18816], [49], [2.0000], [2], [1.0000], [true],
[7], [4], [4], [4+1+1], [6], [3], [5], [65856], [171], [2.6423], [3], [0.4985], [true],
[7], [4], [4], [4+2], [6], [3], [5], [49392], [147], [2.5646], [3], [0.4286], [true],
[11], [4], [4], [4+1], [5], [4], [5], [174240], [126], [2.0169], [2], [1.0413], [true],
[11], [4], [4], [4+1+1], [6], [3], [5], [958320], [686], [2.7236], [3], [0.5154], [true],
[11], [4], [4], [4+2], [6], [3], [5], [798600], [605], [2.6712], [3], [0.4545], [true],
[13], [4], [4], [4+1], [5], [4], [5], [397488], [172], [2.0069], [2], [1.0178], [true],
[13], [4], [4], [4+1+1], [6], [3], [5], [2583672], [1110], [2.7338], [3], [0.5052], [true],
[13], [4], [4], [4+2], [6], [3], [5], [2214576], [1014], [2.6986], [3], [0.4615], [true],
[3], [5], [5], [5], [5], [4], [5], [240], [10], [2.0959], [2], [1.1111], [true],
[5], [5], [5], [5], [5], [4], [5], [3120], [26], [2.0244], [2], [1.0400], [true],
[7], [5], [5], [5], [5], [4], [5], [16800], [50], [2.0104], [2], [1.0204], [true],
[11], [5], [5], [5], [5], [4], [5], [161040], [121], [2.0000], [2], [1.0000], [true],
[13], [5], [5], [5], [5], [4], [5], [371280], [170], [2.0023], [2], [1.0059], [true],
[3], [6], [6], [3+2+1], [6], [4], [9], [576], [21], [2.7712], [3], [0.7778], [true],
[3], [6], [6], [6], [6], [4], [9], [696], [23], [2.8540], [3], [0.8519], [true],
[5], [6], [6], [3+2+1], [6], [4], [9], [14400], [117], [2.9589], [3], [0.9360], [true],
[5], [6], [6], [6], [6], [4], [9], [15480], [121], [2.9798], [3], [0.9680], [true],
[7], [6], [6], [3+2+1], [6], [4], [9], [112896], [336], [2.9894], [3], [0.9796], [true],
[7], [6], [6], [6], [6], [4], [9], [117264], [334], [2.9863], [3], [0.9738], [true],
[11], [6], [6], [3+2+1], [6], [4], [9], [1742400], [1320], [2.9965], [3], [0.9917], [false],
[11], [6], [6], [6], [6], [4], [9], [1770120], [1323], [2.9975], [3], [0.9940], [false],
[13], [6], [6], [3+2+1], [6], [4], [9], [4769856], [2184], [2.9977], [3], [0.9941], [false],
[13], [6], [6], [6], [6], [4], [9], [4824456], [2181], [2.9972], [3], [0.9927], [false],
[3], [7], [7], [7], [7], [3], [7], [4368], [147], [4.5425], [4], [1.8148], [true],
[5], [7], [7], [7], [7], [3], [7], [156240], [1253], [4.4322], [4], [2.0048], [true],
[7], [7], [7], [7], [7], [3], [7], [1647072], [4782], [4.3541], [4], [1.9917], [false]
)

Class statistics (rows: 88; only where the all-j trace table was computed): classes is the number of traces realised over F\_(q^n); covered are those with a weak member of the type; share is the fraction of all curves (both twists of every j) in covered classes; walk = curves in covered classes / weak curves in them.

#table(columns: 12, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*q*], [*n*], [*orbits*], [*r*], [*m*], [*g*], [*classes*], [*covered*], [*share*], [*mean weak*], [*max weak*], [*walk*],
[3], [2], [2+1+1+1], [5], [2], [2], [10], [2], [0.3333], [1.000], [1], [3.00],
[3], [2], [2+2+1], [5], [2], [2], [10], [2], [0.3333], [1.000], [1], [3.00],
[3], [2], [2+2+1+1], [6], [2], [3], [10], [4], [0.4444], [1.000], [1], [2.00],
[3], [2], [2+2+2], [6], [2], [3], [10], [2], [0.3333], [1.000], [1], [3.00],
[3], [2], [3+2], [5], [2], [2], [10], [2], [0.2222], [2.000], [2], [1.00],
[5], [2], [2+1+1+1], [5], [2], [2], [18], [6], [0.4400], [1.333], [2], [2.75],
[5], [2], [2+2+1], [5], [2], [2], [18], [4], [0.4000], [2.000], [2], [2.50],
[5], [2], [2+2+1+1], [6], [2], [3], [18], [6], [0.4400], [1.667], [2], [2.20],
[5], [2], [2+2+2], [6], [2], [3], [18], [6], [0.4400], [1.667], [2], [2.20],
[5], [2], [3+2], [5], [2], [2], [18], [8], [0.3200], [1.750], [2], [1.14],
[7], [2], [2+1+1+1], [5], [2], [2], [26], [6], [0.4082], [2.333], [3], [2.86],
[7], [2], [2+2+1], [5], [2], [2], [26], [8], [0.4286], [2.250], [4], [2.33],
[7], [2], [2+2+1+1], [6], [2], [3], [26], [8], [0.4286], [2.250], [4], [2.33],
[7], [2], [2+2+2], [6], [2], [3], [26], [8], [0.4286], [2.250], [4], [2.33],
[7], [2], [3+2], [5], [2], [2], [26], [10], [0.3061], [2.800], [4], [1.07],
[11], [2], [2+1+1+1], [5], [2], [2], [42], [12], [0.4215], [3.167], [6], [2.68],
[11], [2], [2+2+1], [5], [2], [2], [42], [12], [0.4215], [3.333], [6], [2.55],
[11], [2], [2+2+1+1], [6], [2], [3], [42], [12], [0.4215], [3.500], [6], [2.43],
[11], [2], [2+2+2], [6], [2], [3], [42], [12], [0.4215], [3.500], [6], [2.43],
[11], [2], [3+2], [5], [2], [2], [42], [20], [0.3388], [3.700], [8], [1.11],
[13], [2], [2+1+1+1], [5], [2], [2], [50], [14], [0.4201], [3.714], [7], [2.73],
[13], [2], [2+2+1], [5], [2], [2], [50], [14], [0.4201], [4.143], [8], [2.45],
[13], [2], [2+2+1+1], [6], [2], [3], [50], [14], [0.4201], [4.143], [8], [2.45],
[13], [2], [2+2+2], [6], [2], [3], [50], [14], [0.4201], [4.143], [8], [2.45],
[13], [2], [3+2], [5], [2], [2], [50], [22], [0.3195], [4.727], [8], [1.04],
[3], [3], [3+1+1], [5], [3], [3], [15], [2], [0.2222], [3.000], [3], [2.00],
[3], [3], [3+1+1+1], [6], [3], [5], [15], [4], [0.3704], [2.000], [3], [2.50],
[3], [3], [3+2], [5], [3], [3], [15], [2], [0.2222], [3.000], [3], [2.00],
[3], [3], [3+2+1], [6], [3], [5], [15], [6], [0.5926], [4.000], [6], [1.33],
[3], [3], [3+3], [6], [2], [3], [15], [4], [0.3704], [2.000], [3], [2.50],
[3], [3], [3+3], [6], [3], [5], [15], [4], [0.3704], [2.000], [3], [2.50],
[3], [3], [6], [6], [2], [3], [15], [5], [0.4074], [2.000], [3], [2.20],
[5], [3], [3+1+1], [5], [3], [3], [37], [6], [0.2560], [4.000], [6], [2.67],
[5], [3], [3+1+1+1], [6], [3], [5], [37], [8], [0.4000], [5.000], [6], [2.50],
[5], [3], [3+2], [5], [3], [3], [37], [8], [0.3360], [2.500], [3], [4.20],
[5], [3], [3+2+1], [6], [3], [5], [37], [16], [0.6400], [7.500], [12], [1.33],
[5], [3], [3+3], [6], [2], [3], [37], [10], [0.4160], [4.200], [6], [2.48],
[5], [3], [3+3], [6], [3], [5], [37], [10], [0.4160], [4.200], [6], [2.48],
[5], [3], [6], [6], [2], [3], [37], [10], [0.4160], [4.200], [6], [2.48],
[7], [3], [3+1+1], [5], [3], [3], [63], [9], [0.2478], [5.556], [12], [3.40],
[7], [3], [3+1+1+1], [6], [3], [5], [63], [16], [0.4140], [7.000], [15], [2.54],
[7], [3], [3+2], [5], [3], [3], [63], [4], [0.1224], [10.500], [12], [2.00],
[7], [3], [3+2+1], [6], [3], [5], [63], [32], [0.6647], [10.500], [18], [1.36],
[7], [3], [3+3], [6], [2], [3], [63], [17], [0.4169], [6.824], [15], [2.47],
[7], [3], [3+3], [6], [3], [5], [63], [17], [0.4169], [6.706], [15], [2.51],
[7], [3], [6], [6], [2], [3], [63], [17], [0.4169], [6.824], [15], [2.47],
[11], [3], [3+1+1], [5], [3], [3], [133], [16], [0.2494], [7.500], [12], [5.53],
[11], [3], [3+1+1+1], [6], [3], [5], [133], [34], [0.4147], [12.941], [36], [2.51],
[11], [3], [3+2], [5], [3], [3], [133], [16], [0.2494], [6.875], [12], [6.04],
[11], [3], [3+2+1], [6], [3], [5], [133], [66], [0.6649], [20.000], [48], [1.34],
[11], [3], [3+3], [6], [2], [3], [133], [35], [0.4162], [12.629], [36], [2.51],
[11], [3], [3+3], [6], [3], [5], [133], [35], [0.4162], [12.686], [36], [2.50],
[11], [3], [6], [6], [2], [3], [133], [35], [0.4162], [12.629], [36], [2.51],
[13], [3], [3+1+1], [5], [3], [3], [173], [20], [0.2230], [8.200], [18], [5.98],
[13], [3], [3+1+1+1], [6], [3], [5], [173], [42], [0.4169], [17.333], [36], [2.52],
[13], [3], [3+2], [5], [3], [3], [173], [32], [0.3414], [4.875], [6], [9.62],
[13], [3], [3+2+1], [6], [3], [5], [173], [86], [0.6664], [25.395], [60], [1.34],
[13], [3], [3+3], [6], [2], [3], [173], [42], [0.4169], [17.286], [33], [2.52],
[13], [3], [3+3], [6], [3], [5], [173], [42], [0.4169], [17.429], [36], [2.50],
[13], [3], [6], [6], [2], [3], [173], [42], [0.4169], [17.429], [36], [2.50],
[3], [4], [4+1], [5], [4], [5], [26], [4], [0.2593], [3.500], [6], [3.00],
[3], [4], [4+1+1], [6], [3], [5], [26], [6], [0.4074], [4.333], [6], [2.54],
[3], [4], [4+2], [6], [3], [5], [26], [4], [0.2593], [4.500], [6], [2.33],
[5], [4], [4+1], [5], [4], [5], [82], [8], [0.1792], [6.750], [9], [4.15],
[5], [4], [4+1+1], [6], [3], [5], [82], [10], [0.2240], [12.600], [17], [2.22],
[5], [4], [4+2], [6], [3], [5], [82], [16], [0.3328], [6.250], [12], [4.16],
[7], [4], [4+1], [5], [4], [5], [170], [14], [0.1533], [7.000], [12], [7.51],
[7], [4], [4+1+1], [6], [3], [5], [170], [36], [0.3865], [9.500], [17], [5.43],
[7], [4], [4+2], [6], [3], [5], [170], [22], [0.2366], [13.364], [28], [3.86],
[11], [4], [4+1], [5], [4], [5], [442], [40], [0.1790], [6.300], [14], [20.80],
[11], [4], [4+1+1], [6], [3], [5], [442], [94], [0.3805], [14.596], [44], [8.12],
[11], [4], [4+2], [6], [3], [5], [442], [56], [0.2322], [21.607], [48], [5.62],
[13], [4], [4+1], [5], [4], [5], [626], [40], [0.1347], [8.600], [16], [22.36],
[13], [4], [4+1+1], [6], [3], [5], [626], [78], [0.2276], [28.462], [60], [5.86],
[13], [4], [4+2], [6], [3], [5], [626], [144], [0.4003], [14.083], [60], [11.27],
[3], [5], [5], [5], [4], [5], [43], [4], [0.1646], [5.000], [5], [4.00],
[5], [5], [5], [5], [4], [5], [179], [10], [0.1254], [5.200], [10], [15.08],
[7], [5], [5], [5], [4], [5], [445], [14], [0.0768], [7.143], [15], [25.80],
[11], [5], [5], [5], [4], [5], [1461], [36], [0.0608], [6.722], [20], [80.93],
[13], [5], [5], [5], [4], [5], [2251], [56], [0.0539], [6.071], [10], [117.76],
[3], [6], [3+2+1], [6], [4], [9], [74], [8], [0.2140], [5.250], [6], [7.43],
[3], [6], [6], [6], [4], [9], [74], [8], [0.1742], [5.750], [7], [5.52],
[5], [6], [3+2+1], [6], [4], [9], [402], [24], [0.1334], [9.750], [18], [17.81],
[5], [6], [6], [6], [4], [9], [402], [38], [0.1841], [6.368], [12], [23.77],
[7], [6], [3+2+1], [6], [4], [9], [1178], [64], [0.1195], [10.500], [30], [41.83],
[7], [6], [6], [6], [4], [9], [1178], [86], [0.1414], [7.767], [30], [49.82],
[3], [7], [7], [7], [3], [7], [125], [24], [0.3393], [12.250], [28], [5.05],
[5], [7], [7], [7], [3], [7], [897], [160], [0.3362], [15.662], [42], [20.96]
)

Checks (`oddchar_checks.txt`):
```
q=3 n=2 weak_j_outside_K=0
q=3 n=2 all_j_table=true j_listed=9 hasse_violations=0
q=3 n=2 check6_num_j=9 expected=9
q=5 n=2 weak_j_outside_K=0
q=5 n=2 all_j_table=true j_listed=25 hasse_violations=0
q=5 n=2 check6_num_j=25 expected=25
q=7 n=2 weak_j_outside_K=0
q=7 n=2 all_j_table=true j_listed=49 hasse_violations=0
q=7 n=2 check6_num_j=49 expected=49
q=11 n=2 weak_j_outside_K=0
q=11 n=2 all_j_table=true j_listed=121 hasse_violations=0
q=11 n=2 check6_num_j=121 expected=121
q=13 n=2 weak_j_outside_K=0
q=13 n=2 all_j_table=true j_listed=169 hasse_violations=0
q=13 n=2 check6_num_j=169 expected=169
q=3 n=3 weak_j_outside_K=0
q=3 n=3 all_j_table=true j_listed=27 hasse_violations=0
q=3 n=3 check6_num_j=27 expected=27
q=3 n=3 type=3+1+1 m=3 r=5 check5_curves=3 check5_violations=0
q=3 n=3 type=3+2 m=3 r=5 check5_curves=3 check5_violations=0
q=5 n=3 weak_j_outside_K=0
q=5 n=3 all_j_table=true j_listed=125 hasse_violations=0
q=5 n=3 check6_num_j=125 expected=125
q=5 n=3 type=3+1+1 m=3 r=5 check5_curves=12 check5_violations=0
q=5 n=3 type=3+2 m=3 r=5 check5_curves=10 check5_violations=0
q=7 n=3 weak_j_outside_K=0
q=7 n=3 all_j_table=true j_listed=343 hasse_violations=0
q=7 n=3 check6_num_j=343 expected=343
q=7 n=3 type=3+1+1 m=3 r=5 check5_curves=24 check5_violations=0
q=7 n=3 type=3+2 m=3 r=5 check5_curves=21 check5_violations=0
q=11 n=3 weak_j_outside_K=0
q=11 n=3 all_j_table=true j_listed=1331 hasse_violations=0
q=11 n=3 check6_num_j=1331 expected=1331
q=11 n=3 type=3+1+1 m=3 r=5 check5_curves=60 check5_violations=0
q=11 n=3 type=3+2 m=3 r=5 check5_curves=55 check5_violations=0
q=13 n=3 weak_j_outside_K=0
q=13 n=3 all_j_table=true j_listed=2197 hasse_violations=0
q=13 n=3 check6_num_j=2197 expected=2197
q=13 n=3 type=3+1+1 m=3 r=5 check5_curves=82 check5_violations=0
q=13 n=3 type=3+2 m=3 r=5 check5_curves=78 check5_violations=0
q=3 n=4 weak_j_outside_K=0
q=3 n=4 all_j_table=true j_listed=81 hasse_violations=0
q=3 n=4 check6_num_j=81 expected=81
q=5 n=4 weak_j_outside_K=0
q=5 n=4 all_j_table=true j_listed=625 hasse_violations=0
q=5 n=4 check6_num_j=625 expected=625
q=7 n=4 weak_j_outside_K=0
q=7 n=4 all_j_table=true j_listed=2401 hasse_violations=0
q=7 n=4 check6_num_j=2401 expected=2401
q=11 n=4 weak_j_outside_K=0
q=11 n=4 all_j_table=true j_listed=14641 hasse_violations=0
q=11 n=4 check6_num_j=14641 expected=14641
q=13 n=4 weak_j_outside_K=0
q=13 n=4 all_j_table=true j_listed=28561 hasse_violations=0
q=13 n=4 check6_num_j=28561 expected=28561
q=3 n=5 weak_j_outside_K=0
q=3 n=5 all_j_table=true j_listed=243 hasse_violations=0
q=3 n=5 check6_num_j=243 expected=243
q=5 n=5 weak_j_outside_K=0
q=5 n=5 all_j_table=true j_listed=3125 hasse_violations=0
q=5 n=5 check6_num_j=3125 expected=3125
q=7 n=5 weak_j_outside_K=0
q=7 n=5 all_j_table=true j_listed=16807 hasse_violations=0
q=7 n=5 check6_num_j=16807 expected=16807
q=11 n=5 weak_j_outside_K=0
q=11 n=5 all_j_table=true j_listed=161051 hasse_violations=0
q=11 n=5 check6_num_j=161051 expected=161051
q=13 n=5 weak_j_outside_K=0
q=13 n=5 all_j_table=true j_listed=371293 hasse_violations=0
q=13 n=5 check6_num_j=371293 expected=371293
q=3 n=6 weak_j_outside_K=0
q=3 n=6 all_j_table=true j_listed=729 hasse_violations=0
q=3 n=6 check6_num_j=729 expected=729
q=5 n=6 weak_j_outside_K=0
q=5 n=6 all_j_table=true j_listed=15625 hasse_violations=0
q=5 n=6 check6_num_j=15625 expected=15625
q=7 n=6 weak_j_outside_K=0
q=7 n=6 all_j_table=true j_listed=117649 hasse_violations=0
q=7 n=6 check6_num_j=117649 expected=117649
q=11 n=6 weak_j_outside_K=0
q=11 n=6 all_j_table=false j_listed=2637 hasse_violations=0
q=13 n=6 weak_j_outside_K=0
q=13 n=6 all_j_table=false j_listed=4365 hasse_violations=0
q=3 n=7 weak_j_outside_K=0
q=3 n=7 all_j_table=true j_listed=2187 hasse_violations=0
q=3 n=7 check6_num_j=2187 expected=2187
q=5 n=7 weak_j_outside_K=0
q=5 n=7 all_j_table=true j_listed=78125 hasse_violations=0
q=5 n=7 check6_num_j=78125 expected=78125
q=7 n=7 weak_j_outside_K=0
q=7 n=7 all_j_table=false j_listed=4782 hasse_violations=0
```

== Part C: census (`census.csv`, `census_summary.csv`, `census_patterns.csv`, `census_checks.txt`)

#table(columns: 15, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*q*], [*models*], [*squarefree*], [*Res-shaped models*], [*classes*], [*signature*], [*no signature*], [*roots outside F\_q^6*], [*\#E prime*], [*prime, no sig.*], [*t subfield-shaped*], [*of which sig.*], [*traces*], [*traces with sig.*], [*s*],
[3], [5832], [3888], [152], [21], [6], [15], [0], [0], [0], [7], [0], [9], [2], [0.0],
[5], [187500], [150000], [3088], [158], [20], [138], [0], [0], [0], [70], [8], [21], [10], [0.2],
[7], [3294172], [2823576], [48352], [538], [39], [499], [0], [0], [0], [92], [1], [37], [9], [5.5],
[11], [42517464], [38652240], [293440], [3173], [106], [3067], [0], [0], [0], [547], [6], [73], [16], [717.2]
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
[11], [2+6], [yes], [52]
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
```

== Graphs
Recorded values only, drawn by `figures/plot.py` from the CSVs of the two recorded runs (Parts A and B from run 20261018T0630Z, Part C from run 20261018T0845Z).

#figure(image("../../figures/oddchar_counts.svg", width: 100%), caption: [Weak j-invariants per branch type divided by q^(r−3); exact counts, no uncertainty.])
#figure(image("../../figures/oddchar_classes.svg", width: 100%), caption: [Share of curves in isogeny classes with a weak member, and weak members per covered class; exact, from the all-j trace tables.])
#figure(image("../../figures/census.svg", width: 70%), caption: [Res-shaped genus-3 hyperelliptic classes over F_q by GHS signature; exact census.])
#figure(image("../../figures/margins.svg", width: 100%), caption: [Margins a = δ − n/2 by n: binary (exact cosets) and odd characteristic (smallest-genus type).])
