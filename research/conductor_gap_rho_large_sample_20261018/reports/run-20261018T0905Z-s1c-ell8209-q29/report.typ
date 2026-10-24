#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: s1c-ell8209-q29
Session `ECBS1h093ad8046e45`, status `complete`, 18432 records; spec `ECS1he534671911c5` sha256 `e534671911c54642…`; binary sha256 `ee0af723cf6c1064…`, git commit `42ab2c57490a1eb4ee791e3b8d35563fce61b0e4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell=8209, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets.
Arms: rho-neg = rho.negation (reference), rho-plain = rho.plain (baseline), rho-neg-aa = rho.negation (control). Workloads: 6 curves × 1024 `planted_shared` targets, target seed 2026101804, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 18432}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/s1c-ell8209-q29/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [crater], [icv1-fp29-t47-14392892], [1024], [0.9671], [0.0152], [0.082 / 3.107],
[rho-neg], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [0.9639], [0.0157], [0.060 / 3.280],
[rho-neg], [floor1], [icv1-fp29-t47-1f934956], [1024], [0.9727], [0.0154], [0.066 / 2.640],
[rho-neg], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [0.9360], [0.0150], [0.060 / 2.544],
[rho-neg], [floor2], [icv1-fp29-t47-522e6830], [1024], [0.9798], [0.0149], [0.051 / 2.837],
[rho-neg], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [0.9715], [0.0149], [0.083 / 2.871],
[rho-neg-aa], [crater], [icv1-fp29-t47-14392892], [1024], [0.9671], [0.0152], [0.082 / 3.107],
[rho-neg-aa], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [0.9639], [0.0157], [0.060 / 3.280],
[rho-neg-aa], [floor1], [icv1-fp29-t47-1f934956], [1024], [0.9727], [0.0154], [0.066 / 2.640],
[rho-neg-aa], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [0.9360], [0.0150], [0.060 / 2.544],
[rho-neg-aa], [floor2], [icv1-fp29-t47-522e6830], [1024], [0.9798], [0.0149], [0.051 / 2.837],
[rho-neg-aa], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [0.9715], [0.0149], [0.083 / 2.871],
[rho-plain], [crater], [icv1-fp29-t47-14392892], [1024], [1.3775], [0.0211], [0.056 / 3.933],
[rho-plain], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [1.3027], [0.0212], [0.086 / 4.118],
[rho-plain], [floor1], [icv1-fp29-t47-1f934956], [1024], [1.3182], [0.0207], [0.072 / 4.152],
[rho-plain], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [1.3390], [0.0210], [0.074 / 3.972],
[rho-plain], [floor2], [icv1-fp29-t47-522e6830], [1024], [1.3242], [0.0216], [0.081 / 3.795],
[rho-plain], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [1.3692], [0.0211], [0.026 / 3.982],
)