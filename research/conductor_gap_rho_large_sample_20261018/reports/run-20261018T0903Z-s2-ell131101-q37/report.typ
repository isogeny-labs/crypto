#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: s2-ell131101-q37
Session `ECBS1h4ad8566d376f`, status `complete`, 12288 records; spec `ECS1hf5756920d2b0` sha256 `f5756920d2b0c450…`; binary sha256 `ee0af723cf6c1064…`, git commit `42ab2c57490a1eb4ee791e3b8d35563fce61b0e4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell=131101, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets.
Arms: rho-neg = rho.negation (reference), rho-plain = rho.plain (baseline), rho-neg-aa = rho.negation (control). Workloads: 4 curves × 1024 `planted_shared` targets, target seed 2026101802, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 12288}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/s2-ell131101-q37/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [crater], [icv1-fp37-t277-91fd6e29], [1024], [0.9140], [0.0147], [0.048 / 2.758],
[rho-neg], [crater-twin(u=3)], [icv1-fp37-t277-9df09dab], [1024], [0.9501], [0.0155], [0.037 / 3.185],
[rho-neg], [floor1], [icv1-fp37-t277-090dc414], [1024], [0.9186], [0.0152], [0.029 / 3.087],
[rho-neg], [floor1-twin(u=3)], [icv1-fp37-t277-18ee6dc2], [1024], [0.9222], [0.0156], [0.045 / 3.206],
[rho-neg-aa], [crater], [icv1-fp37-t277-91fd6e29], [1024], [0.9140], [0.0147], [0.048 / 2.758],
[rho-neg-aa], [crater-twin(u=3)], [icv1-fp37-t277-9df09dab], [1024], [0.9501], [0.0155], [0.037 / 3.185],
[rho-neg-aa], [floor1], [icv1-fp37-t277-090dc414], [1024], [0.9186], [0.0152], [0.029 / 3.087],
[rho-neg-aa], [floor1-twin(u=3)], [icv1-fp37-t277-18ee6dc2], [1024], [0.9222], [0.0156], [0.045 / 3.206],
[rho-plain], [crater], [icv1-fp37-t277-91fd6e29], [1024], [1.3217], [0.0215], [0.050 / 3.616],
[rho-plain], [crater-twin(u=3)], [icv1-fp37-t277-9df09dab], [1024], [1.3087], [0.0215], [0.015 / 3.492],
[rho-plain], [floor1], [icv1-fp37-t277-090dc414], [1024], [1.3131], [0.0210], [0.060 / 3.966],
[rho-plain], [floor1-twin(u=3)], [icv1-fp37-t277-18ee6dc2], [1024], [1.3043], [0.0209], [0.060 / 4.555],
)