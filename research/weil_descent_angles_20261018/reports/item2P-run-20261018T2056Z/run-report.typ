#set page(paper: "a4", flipped: true, margin: (x: 1.2cm, y: 1.2cm), numbering: "1 / 1")
#set text(size: 8pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.4pt)
#show table: set text(size: 5.6pt)
#show raw: set text(size: 6.6pt)

= Run report: item2P-run-20261018T2056Z

Descriptive record of one recorded run of examples/pkm\_coverage.rs (Part P) under
`research/weil_descent_angles_20261018/PROTOCOL.md` (Addendum A and
Amendments B–G and Addendum H, each committed before the run it governs). It lists what
ran, on what host, and what the tool recorded, file by file. Interpretation
is in the study's `REPORT.md`. Costs are counted operations
(F#sub[p] multiplications and inversions, group operations, native
operations), never wall-clock; the only times are the tool's own seconds.
Isolation: none earned (a shared cloud container, or a RunPod CPU pod whose lscpu is in host.txt); this
affects only the seconds, since every recorded value is an exact count or
a counted operation total.

== env.txt
```
run: item2P, 20261018T2056Z
protocol commits: 55ebe41b6 (Addendum A), Amendment B (Poisson residual, frozen before this run); tool commit: fe0469c07
rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v139 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.10GHz, 15 GB
command: rustc --edition 2021 -O examples/pkm_coverage.rs -o tool && PKM_THREADS=2 ./tool research/weil_descent_angles_20261018/results/item2P-run-20261018T2056Z
tool sha256: 85bda43eecd9e476bf74c4cc9d9ad86d444d42be7a9e9169b6c0b2dbcf08a254
threads: 2 (sharing the host with the item-4 census and the other coverage run; outputs are exact counts, no timing is evidence)
smoke runs before this run (not evidence): DC_SCALE=0.1 / PKM_SCALE=0.15, single thread.
```
== tool\_log.txt
```
jobs: 144
P kummer m=2 t_off=-1: primes 24 pi family 0.2158 null 0.2088 positive 0.0022 | resid family -0.00024 null +0.00000 positive -0.2138 | paired resid diff -0.00024 (z -1.10)
P kummer m=2 t_off=+0: primes 24 pi family 0.5793 null 0.5920 positive 0.0044 | resid family +0.00034 null +0.00075 positive -0.5746 | paired resid diff -0.00041 (z -1.40)
P kummer m=2 t_off=+1: primes 24 pi family 0.9501 null 0.9501 positive 0.0087 | resid family +0.00013 null +0.00037 positive -0.9412 | paired resid diff -0.00024 (z -1.84)
P kummer m=3 t_off=-1: primes 24 pi family 0.1168 null 0.1208 positive 0.0013 | resid family -0.00731 null -0.00685 positive -0.1228 | paired resid diff -0.00046 (z -0.43)
P kummer m=3 t_off=+0: primes 24 pi family 0.5884 null 0.5873 positive 0.0029 | resid family -0.00919 null -0.00962 positive -0.5947 | paired resid diff +0.00043 (z 0.26)
P kummer m=3 t_off=+1: primes 24 pi family 0.9910 null 0.9892 positive 0.0058 | resid family -0.00030 null -0.00073 positive -0.9855 | paired resid diff +0.00043 (z 1.36)
P dickson_split m=2 t_off=-1: primes 24 pi family 0.2356 null 0.2291 positive 0.0029 | resid family -0.00040 null -0.00049 positive -0.2331 | paired resid diff +0.00008 (z 0.34)
P dickson_split m=2 t_off=+0: primes 24 pi family 0.6455 null 0.6464 positive 0.0058 | resid family +0.00065 null +0.00051 positive -0.6391 | paired resid diff +0.00015 (z 0.39)
P dickson_split m=2 t_off=+1: primes 23 pi family 0.9740 null 0.9740 positive 0.0116 | resid family +0.00027 null +0.00032 positive -0.9621 | paired resid diff -0.00004 (z -0.34)
P dickson_split m=3 t_off=-1: primes 24 pi family 0.1194 null 0.1315 positive 0.0014 | resid family -0.00782 null -0.00729 positive -0.1259 | paired resid diff -0.00053 (z -0.85)
P dickson_split m=3 t_off=+0: primes 24 pi family 0.5945 null 0.6123 positive 0.0030 | resid family -0.00929 null -0.01165 positive -0.6009 | paired resid diff +0.00236 (z 1.71)
P dickson_split m=3 t_off=+1: primes 24 pi family 0.9812 null 0.9823 positive 0.0058 | resid family -0.00109 null -0.00101 positive -0.9765 | paired resid diff -0.00008 (z -0.22)
P dickson_torus m=2 t_off=-1: primes 24 pi family 0.2231 null 0.2258 positive 0.0021 | resid family -0.00034 null -0.00021 positive -0.2214 | paired resid diff -0.00013 (z -0.53)
P dickson_torus m=2 t_off=+0: primes 23 pi family 0.6174 null 0.6194 positive 0.0041 | resid family +0.00037 null +0.00042 positive -0.6130 | paired resid diff -0.00005 (z -0.23)
P dickson_torus m=2 t_off=+1: primes 24 pi family 0.9636 null 0.9638 positive 0.0084 | resid family +0.00026 null +0.00031 positive -0.9550 | paired resid diff -0.00005 (z -0.45)
P dickson_torus m=3 t_off=-1: primes 24 pi family 0.1449 null 0.1185 positive 0.0017 | resid family -0.00698 null -0.00684 positive -0.1502 | paired resid diff -0.00014 (z -0.16)
P dickson_torus m=3 t_off=+0: primes 24 pi family 0.6038 null 0.6119 positive 0.0035 | resid family -0.00931 null -0.00831 positive -0.6096 | paired resid diff -0.00100 (z -0.48)
P dickson_torus m=3 t_off=+1: primes 24 pi family 0.9921 null 0.9891 positive 0.0071 | resid family -0.00044 null -0.00061 positive -0.9855 | paired resid diff +0.00018 (z 0.71)
total 257.2s
```
== checks.txt
```
P kummer m=2 t_off=-1: primes 24 pi family 0.2158 null 0.2088 positive 0.0022 | resid family -0.00024 null +0.00000 positive -0.2138 | paired resid diff -0.00024 (z -1.10)
P kummer m=2 t_off=+0: primes 24 pi family 0.5793 null 0.5920 positive 0.0044 | resid family +0.00034 null +0.00075 positive -0.5746 | paired resid diff -0.00041 (z -1.40)
P kummer m=2 t_off=+1: primes 24 pi family 0.9501 null 0.9501 positive 0.0087 | resid family +0.00013 null +0.00037 positive -0.9412 | paired resid diff -0.00024 (z -1.84)
P kummer m=3 t_off=-1: primes 24 pi family 0.1168 null 0.1208 positive 0.0013 | resid family -0.00731 null -0.00685 positive -0.1228 | paired resid diff -0.00046 (z -0.43)
P kummer m=3 t_off=+0: primes 24 pi family 0.5884 null 0.5873 positive 0.0029 | resid family -0.00919 null -0.00962 positive -0.5947 | paired resid diff +0.00043 (z 0.26)
P kummer m=3 t_off=+1: primes 24 pi family 0.9910 null 0.9892 positive 0.0058 | resid family -0.00030 null -0.00073 positive -0.9855 | paired resid diff +0.00043 (z 1.36)
P dickson_split m=2 t_off=-1: primes 24 pi family 0.2356 null 0.2291 positive 0.0029 | resid family -0.00040 null -0.00049 positive -0.2331 | paired resid diff +0.00008 (z 0.34)
P dickson_split m=2 t_off=+0: primes 24 pi family 0.6455 null 0.6464 positive 0.0058 | resid family +0.00065 null +0.00051 positive -0.6391 | paired resid diff +0.00015 (z 0.39)
P dickson_split m=2 t_off=+1: primes 23 pi family 0.9740 null 0.9740 positive 0.0116 | resid family +0.00027 null +0.00032 positive -0.9621 | paired resid diff -0.00004 (z -0.34)
P dickson_split m=3 t_off=-1: primes 24 pi family 0.1194 null 0.1315 positive 0.0014 | resid family -0.00782 null -0.00729 positive -0.1259 | paired resid diff -0.00053 (z -0.85)
P dickson_split m=3 t_off=+0: primes 24 pi family 0.5945 null 0.6123 positive 0.0030 | resid family -0.00929 null -0.01165 positive -0.6009 | paired resid diff +0.00236 (z 1.71)
P dickson_split m=3 t_off=+1: primes 24 pi family 0.9812 null 0.9823 positive 0.0058 | resid family -0.00109 null -0.00101 positive -0.9765 | paired resid diff -0.00008 (z -0.22)
P dickson_torus m=2 t_off=-1: primes 24 pi family 0.2231 null 0.2258 positive 0.0021 | resid family -0.00034 null -0.00021 positive -0.2214 | paired resid diff -0.00013 (z -0.53)
P dickson_torus m=2 t_off=+0: primes 23 pi family 0.6174 null 0.6194 positive 0.0041 | resid family +0.00037 null +0.00042 positive -0.6130 | paired resid diff -0.00005 (z -0.23)
P dickson_torus m=2 t_off=+1: primes 24 pi family 0.9636 null 0.9638 positive 0.0084 | resid family +0.00026 null +0.00031 positive -0.9550 | paired resid diff -0.00005 (z -0.45)
P dickson_torus m=3 t_off=-1: primes 24 pi family 0.1449 null 0.1185 positive 0.0017 | resid family -0.00698 null -0.00684 positive -0.1502 | paired resid diff -0.00014 (z -0.16)
P dickson_torus m=3 t_off=+0: primes 24 pi family 0.6038 null 0.6119 positive 0.0035 | resid family -0.00931 null -0.00831 positive -0.6096 | paired resid diff -0.00100 (z -0.48)
P dickson_torus m=3 t_off=+1: primes 24 pi family 0.9921 null 0.9891 positive 0.0071 | resid family -0.00044 null -0.00061 positive -0.9855 | paired resid diff +0.00018 (z 0.71)
```
== pkm\_coverage.csv (first 12 of 1290 rows)
#table(columns: 16, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*family*], [*m*], [*p*], [*r*], [*log2r*], [*t*], [*t\_offset*], [*arm*], [*V*], [*F*], [*heuristic*], [*lambda*], [*poisson*], [*distinct*], [*multisets*], [*pi*],
[kummer], [2], [65537], [65579], [16.001], [8], [-1], [kummer], [256], [264], [0.499672], [0.533402], [0.413394], [27109], [34980], [0.413379],
[kummer], [2], [65537], [65579], [16.001], [8], [-1], [null], [256], [278], [0.499672], [0.591363], [0.446428], [29231], [38781], [0.445737],
[kummer], [2], [65537], [65579], [16.001], [8], [-1], [positive], [256], [264], [0.499672], [0.533402], [0.413394], [529], [34980], [0.008067],
[kummer], [2], [65537], [65579], [16.001], [9], [0], [kummer], [512], [488], [1.998689], [1.819424], [0.837881], [54915], [119316], [0.837387],
[kummer], [2], [65537], [65579], [16.001], [9], [0], [null], [512], [518], [1.998689], [2.049757], [0.871234], [57397], [134421], [0.875234],
[kummer], [2], [65537], [65579], [16.001], [9], [0], [positive], [512], [488], [1.998689], [1.819424], [0.837881], [977], [119316], [0.014898],
[kummer], [2], [65537], [65579], [16.001], [10], [1], [kummer], [1024], [1040], [7.994754], [8.254472], [0.999740], [65555], [541320], [0.999634],
[kummer], [2], [65537], [65579], [16.001], [10], [1], [null], [1024], [1012], [7.994754], [7.816191], [0.999597], [65565], [512578], [0.999787],
[kummer], [2], [65537], [65579], [16.001], [10], [1], [positive], [1024], [1040], [7.994754], [8.254472], [0.999740], [2081], [541320], [0.031733],
[kummer], [2], [70657], [71089], [16.117], [8], [-1], [kummer], [256], [274], [0.460943], [0.529969], [0.411377], [29141], [37675], [0.409923],
[kummer], [2], [70657], [71089], [16.117], [8], [-1], [null], [256], [256], [0.460943], [0.462744], [0.370446], [26359], [32896], [0.370789],
[kummer], [2], [70657], [71089], [16.117], [8], [-1], [positive], [256], [274], [0.460943], [0.529969], [0.411377], [549], [37675], [0.007723]
)
== pkm\_summary.csv (first 12 of 18 rows)
#table(columns: 13, align: left, stroke: 0.4pt + rgb("#d9d8d4"),
[*family*], [*m*], [*t\_offset*], [*primes*], [*mean\_pi\_family*], [*mean\_pi\_null*], [*mean\_pi\_positive*], [*resid\_family*], [*resid\_null*], [*resid\_positive*], [*resid\_diff*], [*sd\_resid\_diff*], [*z\_resid\_diff*],
[kummer], [2], [-1], [24], [0.215779], [0.208832], [0.002236], [-0.000240], [0.000002], [-0.213782], [-0.000242], [0.001083], [-1.10],
[kummer], [2], [0], [24], [0.579288], [0.592036], [0.004368], [0.000344], [0.000753], [-0.574576], [-0.000409], [0.001430], [-1.40],
[kummer], [2], [1], [24], [0.950070], [0.950146], [0.008726], [0.000132], [0.000373], [-0.941212], [-0.000240], [0.000640], [-1.84],
[kummer], [3], [-1], [24], [0.116842], [0.120809], [0.001339], [-0.007311], [-0.006851], [-0.122813], [-0.000460], [0.005279], [-0.43],
[kummer], [3], [0], [24], [0.588405], [0.587274], [0.002926], [-0.009194], [-0.009619], [-0.594672], [0.000425], [0.008148], [0.26],
[kummer], [3], [1], [24], [0.990996], [0.989248], [0.005817], [-0.000298], [-0.000727], [-0.985477], [0.000429], [0.001547], [1.36],
[dickson\_split], [2], [-1], [24], [0.235561], [0.229063], [0.002877], [-0.000403], [-0.000487], [-0.233087], [0.000084], [0.001232], [0.34],
[dickson\_split], [2], [0], [24], [0.645538], [0.646389], [0.005778], [0.000655], [0.000505], [-0.639105], [0.000149], [0.001890], [0.39],
[dickson\_split], [2], [1], [23], [0.974048], [0.973990], [0.011647], [0.000273], [0.000316], [-0.962128], [-0.000043], [0.000609], [-0.34],
[dickson\_split], [3], [-1], [24], [0.119446], [0.131493], [0.001405], [-0.007817], [-0.007290], [-0.125858], [-0.000527], [0.003037], [-0.85],
[dickson\_split], [3], [0], [24], [0.594514], [0.612283], [0.002950], [-0.009292], [-0.011650], [-0.600856], [0.002358], [0.006771], [1.71],
[dickson\_split], [3], [1], [24], [0.981202], [0.982349], [0.005770], [-0.001093], [-0.001012], [-0.976526], [-0.000082], [0.001782], [-0.22]
)

== Graph
#figure(image("../../figures/item2_pkm.svg", width: 100%), caption: [Part P: z of the paired Poisson residual, tower factor base vs random x-set, per cell.])
