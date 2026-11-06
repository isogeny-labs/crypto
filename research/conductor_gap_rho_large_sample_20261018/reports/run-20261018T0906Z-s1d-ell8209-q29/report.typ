#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: s1d-ell8209-q29
Session `ECBS1h30a76d209f47`, status `complete`, 18432 records; spec `ECS1h2a3535d11b86` sha256 `2a3535d11b86aa4a…`; binary sha256 `ee0af723cf6c1064…`, git commit `42ab2c57490a1eb4ee791e3b8d35563fce61b0e4`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell=8209, D=-19, hard kernel: crater vs 3 floors vs isomorphic twins, 1024 shared targets.
Arms: rho-neg = rho.negation (reference), rho-plain = rho.plain (baseline), rho-neg-aa = rho.negation (control). Workloads: 6 curves × 1024 `planted_shared` targets, target seed 2026101805, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 18432}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/s1d-ell8209-q29/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [crater], [icv1-fp29-t47-14392892], [1024], [0.9693], [0.0155], [0.061 / 2.966],
[rho-neg], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [1.0005], [0.0159], [0.092 / 2.784],
[rho-neg], [floor1], [icv1-fp29-t47-1f934956], [1024], [0.9515], [0.0152], [0.063 / 3.388],
[rho-neg], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [0.9716], [0.0155], [0.051 / 2.626],
[rho-neg], [floor2], [icv1-fp29-t47-522e6830], [1024], [0.9821], [0.0158], [0.080 / 2.774],
[rho-neg], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [0.9634], [0.0154], [0.072 / 3.033],
[rho-neg-aa], [crater], [icv1-fp29-t47-14392892], [1024], [0.9693], [0.0155], [0.061 / 2.966],
[rho-neg-aa], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [1.0005], [0.0159], [0.092 / 2.784],
[rho-neg-aa], [floor1], [icv1-fp29-t47-1f934956], [1024], [0.9515], [0.0152], [0.063 / 3.388],
[rho-neg-aa], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [0.9716], [0.0155], [0.051 / 2.626],
[rho-neg-aa], [floor2], [icv1-fp29-t47-522e6830], [1024], [0.9821], [0.0158], [0.080 / 2.774],
[rho-neg-aa], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [0.9634], [0.0154], [0.072 / 3.033],
[rho-plain], [crater], [icv1-fp29-t47-14392892], [1024], [1.3505], [0.0212], [0.058 / 3.701],
[rho-plain], [crater-twin(u=3)], [icv1-fp29-t47-31294f58], [1024], [1.3260], [0.0210], [0.115 / 4.019],
[rho-plain], [floor1], [icv1-fp29-t47-1f934956], [1024], [1.3140], [0.0203], [0.083 / 3.424],
[rho-plain], [floor1-twin(u=3)], [icv1-fp29-t47-3ccb5527], [1024], [1.3188], [0.0205], [0.089 / 3.700],
[rho-plain], [floor2], [icv1-fp29-t47-522e6830], [1024], [1.3251], [0.0200], [0.079 / 3.749],
[rho-plain], [floor3], [icv1-fp29-t47-4a644ab5], [1024], [1.3063], [0.0207], [0.047 / 3.687],
)