#set page(paper: "a4", margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 7pt)
= Run report: l8r-pairs16-23
Session `ECBS1he5973c1e51ff`, status `complete`, 256 records; spec `ECS1h7e0846278378` sha256 `7e0846278378c785…`; binary sha256 `ee0af723cf6c1064…`, git commit `20dba5f4c750ae4888415cf92cf395a2c94f6be0`; env class `ECBENV2ha8c679da3835`; host env class sha256 `a8c679da3835cd97…` (Intel Xeon 6975P-C, 32 logical CPUs, per `ecbench host`). Isolation levels earned: ['L0'] (shared host, no CPU reservation). Spec label: conductor gap ell~2^25, D=-19, 53-bit, pairs 16..23 of cm_orbits: crater vs floor, 16 shared targets.
Arms: rho-neg = rho.negation (reference). Workloads: 16 curves × 16 `planted_shared` targets, target seed 2026101838, measurement seed 20261018, rounds 1, warm-up 0.
Record outcomes: {'verified': 256}. Unit: S = charged group operations / sqrt(r) (`cost.s`, lower bound: canonicalisations uncharged). Wall time not read. Raw records: `sessions/l8r-pairs16-23/records.jsonl`.
#figure(image("per_curve.svg", width: 95%), caption: [Mean S per curve and arm with ±1.96 SE over targets (recorded values).])
#table(columns: 7, [arm], [curve], [slug], [n], [mean S], [SE], [min / max],
[rho-neg], [pair16-crater], [icv1-fp53-t34024341-e3c0702b], [16], [1.0566], [0.0820], [0.571 / 1.779],
[rho-neg], [pair16-floor], [icv1-fp53-t34024341-0eea4a4a], [16], [0.9841], [0.1254], [0.113 / 1.794],
[rho-neg], [pair17-crater], [icv1-fp53-t34040357-35c8b067], [16], [1.1923], [0.1746], [0.151 / 2.423],
[rho-neg], [pair17-floor], [icv1-fp53-t34040357-f237e2be], [16], [1.0425], [0.1398], [0.214 / 2.008],
[rho-neg], [pair18-crater], [icv1-fp53-t34042037-e54d747f], [16], [1.0593], [0.1336], [0.178 / 2.287],
[rho-neg], [pair18-floor], [icv1-fp53-t34042037-202608c9], [16], [1.3730], [0.1747], [0.175 / 2.367],
[rho-neg], [pair19-crater], [icv1-fp53-t34049261-62ff07a5], [16], [0.9551], [0.0994], [0.353 / 1.720],
[rho-neg], [pair19-floor], [icv1-fp53-t34049261-aad1a4df], [16], [0.8634], [0.1011], [0.240 / 1.551],
[rho-neg], [pair20-crater], [icv1-fp53-t34050177-eee03cfb], [16], [0.9434], [0.1033], [0.236 / 2.030],
[rho-neg], [pair20-floor], [icv1-fp53-t34050177-1fef7712], [16], [0.9393], [0.1397], [0.253 / 2.349],
[rho-neg], [pair21-crater], [icv1-fp53-t34058157-767b2a00], [16], [1.0225], [0.1670], [0.238 / 2.393],
[rho-neg], [pair21-floor], [icv1-fp53-t34058157-da7474d1], [16], [0.7667], [0.0983], [0.155 / 1.319],
[rho-neg], [pair22-crater], [icv1-fp53-t34093521-383bf2fa], [16], [1.3386], [0.2174], [0.406 / 3.289],
[rho-neg], [pair22-floor], [icv1-fp53-t34093521-8e2561e6], [16], [1.1124], [0.1038], [0.288 / 1.818],
[rho-neg], [pair23-crater], [icv1-fp53-t34106297-c2a974f9], [16], [1.1427], [0.1331], [0.397 / 2.250],
[rho-neg], [pair23-floor], [icv1-fp53-t34106297-9acec86d], [16], [0.9815], [0.1423], [0.182 / 2.397],
)