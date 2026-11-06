#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: l6r-pairs04-07
Session `ECBS1h7018cc2bac70`, status `complete`, 192 records; spec `ECS1hb5946b29ca2c` sha256 `b5946b29ca2cd508…`; binary sha256 `ee0af723cf6c1064…`, git commit `20dba5f4c750ae4888415cf92cf395a2c94f6be0`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 4..7 of cm_orbits: crater vs floor vs crater twins, 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 12 curves × 16 `planted_shared` targets, target seed 2026101836, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 192}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/l6r-pairs04-07/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair4-crater], [icv1-fp53-t33684497-ecd1e0df], [16], [1.2074], [0.1536], [0.316 / 2.817],
[rho-neg], [pair4-crater-twin(u=3)], [icv1-fp53-t33684497-434b27f3], [16], [0.9339], [0.1192], [0.375 / 1.849],
[rho-neg], [pair4-floor], [icv1-fp53-t33684497-3523039e], [16], [0.8200], [0.1313], [0.098 / 1.835],
[rho-neg], [pair5-crater], [icv1-fp53-t33705281-dcfbcce7], [16], [1.4260], [0.1288], [0.587 / 2.546],
[rho-neg], [pair5-crater-twin(u=3)], [icv1-fp53-t33705281-769fbfc4], [16], [1.0373], [0.1118], [0.347 / 1.840],
[rho-neg], [pair5-floor], [icv1-fp53-t33705281-298d41bc], [16], [1.0434], [0.1328], [0.141 / 1.998],
[rho-neg], [pair6-crater], [icv1-fp53-t33756437-6aab8717], [16], [0.8930], [0.0838], [0.179 / 1.391],
[rho-neg], [pair6-crater-twin(u=3)], [icv1-fp53-t33756437-601c2640], [16], [1.0879], [0.1199], [0.291 / 2.333],
[rho-neg], [pair6-floor], [icv1-fp53-t33756437-405cd167], [16], [1.0422], [0.1345], [0.253 / 2.320],
[rho-neg], [pair7-crater], [icv1-fp53-t33785081-1ffbd22f], [16], [1.0336], [0.1504], [0.075 / 2.028],
[rho-neg], [pair7-crater-twin(u=3)], [icv1-fp53-t33785081-48525048], [16], [0.9557], [0.1139], [0.402 / 1.930],
[rho-neg], [pair7-floor], [icv1-fp53-t33785081-aac0c67d], [16], [0.8434], [0.0867], [0.242 / 1.587],
)