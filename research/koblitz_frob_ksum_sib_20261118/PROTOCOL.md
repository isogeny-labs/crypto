---
title: "Protocol: Necklace-reduced x-only sibling k-sum tables with free setup collisions on Koblitz and binary curves"
date: 2026-11-18
status: registered and committed before any study measurement
---

# Protocol: Necklace-reduced x-only sibling k-sum tables with free setup collisions on Koblitz and binary curves [pcc:unverified]

**Track: generic and Frobenius-orbit decomposition (bounded by $\tilde{\Omega}(\sqrt{r / n})$ on Koblitz curves via the $2n$-fold signed Frobenius symmetry and $\tilde{\Omega}(\sqrt{r})$ on generic binary curves via negation; cannot beat Pollard rho asymptotically in exponent).** Nothing in the shipped candidates claims a sub-$\sqrt{r}$ algebraic exponent. Section 4 also records an attempted algebraic-track pilot (`pdp3-koblitz` leaf Gray-code quadratic enumeration) that produced a null result (`+0.8%` / `-1.3%` word XORs) and was not shipped. [pcc:unverified]

## 1. Question and Mechanism

Cold single-target `ic.pipeline` index calculus on Koblitz curves (`LEADERBOARD.md`, `ecbench_generator_orbit_20261022` and `koblitz_ksum_fold_study_20261115`) previously faced two bottlenecks:

1. **Pair-table saturation (`mitm-frobenius-layout-counted:m=2`, `gen-layout`):** $c$ Frobenius orbits hold at most $\approx c^2 n$ pair-sum orbits while requiring $c - 1$ unknown-orbit relations, forcing total cost $\Theta(r^{1/3})$ rather than $\Theta(\sqrt{r / n})$. At 39 bits (`icv1-f2m41-tm2308219-7f48b14a`), `gen-layout` (`columns=90`) spent $S(w=0) = 2.27$, $S(w=1) = 6.88$, and $\approx 444\text{ ms}$ per cold solve ($18\times$ `rho.signed_frobenius_strong`).
2. **Unbounded Frobenius $k$-sum build (`ksum-frobenius:m=3`):** building all $\binom{|F|/2}{k} 2^{k-1}$ tuples without rotational deduplication, sibling inversion sharing, dead-end pruning, or an entry cap visited every Frobenius rotation $k$ times, paid a full affine addition per leaf sign, and over-built small-order curves (`S = 8.32` on `icv1-f2m29-tm40309-30c52b96`, a $5.3\times$ regression).

This study tests five composable, strictly counted changes (all opt-in on `ksum-frobenius` and `ksum` so historical replay sentinels remain bit-identical):

- **`BinaryGroup::sibling_pair` (`src/cryptanalysis/ic_boundary.rs`):** computes both $x(P + J)$ and $x(P - J)$ on $y^2 + x y = x^3 + a x^2 + b$ over $\mathbb{F}_{2^n}$ from one shared inverse $(x_P \oplus x_J)^{-1}$, charged as `1 GroupAdd + 1 sibling_second_points` (matching exact `bitop/v1` gate counts). This also enables `ksum:m=k,sib=1` on generic binary curves.
- **`layout=1` (`FrobeniusKSumTable`):** indexes Frobenius orbit members directly on `compact-orbit-scan` bases (zero build-time Frobenius maps or base-hash lookups) and enforces a cyclic minimum-gap necklace bound inside the leading orbit (`rot_1 <= n / 2`, `prev_rot + rot_1 <= rot_s <= n - rot_1`), eliminating rotational duplicates while retaining 100% of distinct orbit keys.
- **`prune=1` and batched leaf `sib=1` (`FrobeniusKSumTable`):** skips partial sums with too few remaining abscissae to complete $m$ summands, and forms leaf sign pairs $x(S \pm P_j)$ using Montgomery batch inversion (`Gf2::batch_inv` in chunks of up to 256 leaves), halving setup `GroupAdd` per stored orbit entry to $\approx 0.52$.
- **`free=1` (`FrobeniusKSumOracle`):** harvests colliding Frobenius-orbit keys encountered during table construction as group-checked target-free orbit relations, with a counted $\mathbb{Z}[\tau]$-identity pre-filter modulo $r$ so intra-orbit $\tau$-NAF synonyms never spend group additions.
- **`scale=X` (`FrobeniusKSumOracle`):** caps stored unique orbit entries at the scale-free analytical balance point $E = \max(1, \mathrm{round}(X \sqrt{r / n}))$. Because a subgroup of prime order $r$ has $(r - 1) / (2n)$ signed Frobenius orbits, $E = X \sqrt{r / n}$ yields a scale-invariant expected self-collision count $\mathbb{E}[\text{collisions}] \approx n E^2 / r = X^2$ (pinning a second orbit column in setup with probability $1 - e^{-X^2}$ across all $r$ and $n$) and total expected group cost
  $$\mathbb{E}[S(w = 0)] \approx \frac{1}{\sqrt{n}}\left(0.52 X + \frac{1 + e^{-X^2}}{2 X}\right) \approx \frac{1.17}{\sqrt{n}} \quad (X = 1.35),$$
  within a small constant factor of the Pollard rho Frobenius-orbit constant $\sqrt{\pi / (4n)} \approx 0.886 / \sqrt{n}$.

## 2. Cohorts, Curves, and Arms

All curves are registered ICV1 slugs (`docs/curves/registry.json`). Target seeds (`2026111801..2026111809`) and measurement seeds (`8101..8109`) are held out and disjoint from all sizing/diagnostic pilot seeds (`999001..999005`). Every spec sets `target_kind = "public"`, `order = "alternate"`, `warmup = 1`, `rounds = 2` (even round count so `alternate` AB/BA pairs are balanced), and `isolation_required = "L0"`.

### Cohort K (Primary): Cold Single-Target Koblitz Ladder (15.4 to 44.3 bits)

| session | curve slug | model | $\log_2 r$ | public targets $\times$ rounds | incumbent `gen-layout` columns ($C$) |
|:--|:--|:--|--:|:--|--:|
| `K1-n29` | `icv1-f2m29-tm40309-30c52b96` | $K_1 / \mathbb{F}_{2^{29}}$ | 15.37 | $32 \times 2$ | `2` (and `4`) |
| `K0-n23` | `icv1-f2m23-t5197-69e76b73` | $K_0 / \mathbb{F}_{2^{23}}$ | 21.00 | $32 \times 2$ | `6` |
| `K1-n23` | `icv1-f2m23-tm5197-1f85e9e1` | $K_1 / \mathbb{F}_{2^{23}}$ | 22.00 | $32 \times 2$ | `8` |
| `K0-n37` | `icv1-f2m37-tm534059-32aad96b` | $K_0 / \mathbb{F}_{2^{37}}$ | 27.78 | $32 \times 2$ | `13`, `16` |
| `K0-n41` | `icv1-f2m41-tm2308219-7f48b14a` | $K_0 / \mathbb{F}_{2^{41}}$ | 39.00 | $24 \times 2$ | `90`, `158` |
| `K0-n53` | `icv1-f2m53-tm56619371-dac20a85` | $K_0 / \mathbb{F}_{2^{53}}$ | 44.26 | $16 \times 2$ | `220` |

**Arms in Cohort K:**
- `rho-strong` (`reference`): `rho.signed_frobenius_strong`
- `rho-strong-aa` (`control`): `rho.signed_frobenius_strong` (paired A/A control)
- `inc-gen-layout-c<C>` (`baseline`): `ic.pipeline` with `compact-orbit-scan:columns=<C>,raw_x_cap=100000000,generator_orbit=1`, `oracle = mitm-frobenius-layout-counted:m=2`, `targets = stride-generator` (includes both published `LEADERBOARD.md` column counts `16`, `90` and analytical cube-root optima $C^* = \mathrm{round}((r / (2 n^3))^{1/3}) = 13, 158$)
- `inc-kf3` (`baseline` on `K1-n29`, `K0-n23`, `K1-n23`, `K0-n37`): `compact-orbit-scan:columns=1,raw_x_cap=100000000,generator_orbit=1`, `oracle = ksum-frobenius:m=3`, `targets = stride-generator`
- **`cand-kf4-c2-free` (Primary universal candidate on all 6 curves, zero per-curve tuning):** `compact-orbit-scan:columns=2,raw_x_cap=100000000,generator_orbit=1`, `oracle = ksum-frobenius:m=4,layout=1,prune=1,sib=1,free=1,scale=1.35`, `targets = stride-generator`
- **`cand-kf5-c1` (Secondary zero-linalg single-orbit candidate on `K1-n29`, `K0-n23`, `K1-n23`, `K0-n37`):** `compact-orbit-scan:columns=1,raw_x_cap=100000000,generator_orbit=1`, `oracle = ksum-frobenius:m=5,layout=1,prune=1,sib=1,scale=1.0`, `targets = stride-generator`

### Cohort B (Secondary): Table B Primary Random-Binary Curves (`BinaryGroup::sibling_pair`)

| session | curve slug | $\log_2 r$ | public targets $\times$ rounds | baseline `ksum:m=4|5,prune=1` sizes | candidate `ksum:m=5,prune=1,sib=1,free=1` size |
|:--|:--|--:|:--|:--|:--|
| `B21` | `icv1-f2m21-t2591-de54d72e` | 20.00 | $32 \times 2$ | `ksum4-prune` (`N=10`), `ksum5-prune` (`N=9`) | `ksum5-sib-free` (`N=10`) |
| `B24` | `icv1-f2m24-tm3047-2d0c0e8c` | 21.00 | $32 \times 2$ | `ksum4-prune` (`N=12`), `ksum5-prune` (`N=9`) | `ksum5-sib-free` (`N=10`) |
| `B27` | `icv1-f2m27-t12543-569dca8b` | 24.41 | $32 \times 2$ | `ksum4-prune` (`N=16`), `ksum5-prune` (`N=12`) | `ksum5-sib-free` (`N=12`) |

**Arms in Cohort B:** `rho-neg` (`reference`: `rho.negation`), `rho-neg-aa` (`control`), `inc-ksum4-prune` (`baseline`, `ksum:m=4,prune=1`), `inc-ksum5-prune` (`baseline`, `ksum:m=5,prune=1`, the 2026-11-15 `ic_ksum_prune_20261115` champion), and `cand-ksum5-sib-free` (`candidate`, `ksum:m=5,prune=1,sib=1,free=1`).

## 3. Registered Acceptance and Falsification Criteria (§5a Pricing Band & `bitop/v1`)

For each curve, let $I(w)$ denote the **strictest baseline IC arm** (minimum mean $S(w)$ at weight $w$ among all `inc-*` arms in that session). We evaluate paired target-by-round cost ratios $R_w = \mathrm{cost}(\text{candidate}) / \mathrm{cost}(I(w))$ across all three `AGENTS.md` §5a weights $w \in \{0, 0.1, 1.0\}$ via `ecbench compare --weight w --resamples 4000 --seed 20261118`, plus `ecbench band --ranks`, `ecbench work` (`bitop/v1`), and `ecbench verify --replay-all`.

1. **Verification & completeness gate (mandatory):** 100% of runs end with `outcome.status` equal to `"verified"` (`[k]G == Q` checked inside the harness), 100% of measured records reproduce identically under `ecbench verify --replay-all`, and `ecbench work` reports `unpriced == []`, `known_uncounted == []`, `complete == true` on every candidate record. [pcc:unverified]
2. **Primary Koblitz criterion (H1):** `cand-kf4-c2-free` achieves upper 95% confidence bound of $R_w < 1.0$ across **all three weights** $w \in \{0, 0.1, 1.0\}$ (`cand-kf4-c2-free < strictest incumbent` in `ecbench band --ranks`) on the B1 scaling anchors `icv1-f2m41-tm2308219-7f48b14a` (predicted $R_0 \in [0.08, 0.22]$, $>5\times$ speedup) and `icv1-f2m53-tm56619371-dac20a85` (predicted $R_0 < 0.20$), as well as `icv1-f2m37-tm534059-32aad96b` and `icv1-f2m29-tm40309-30c52b96` (eliminating the prior $5.3\times$ `kf3` regression).
3. **Secondary generic-binary criterion (H2):** `cand-ksum5-sib-free` achieves upper 95% confidence bound of $R_w < 1.0$ across **all three weights** $w \in \{0, 0.1, 1.0\}$ against the strictest `ksum:m=4|5,prune=1` incumbent on at least two of the three primary Table B curves (`B21`, `B24`, `B27`; predicted $R_0 \in [0.70, 0.85]$).
4. **Falsification:** A hypothesis fails on any curve where `ecbench band --ranks` reports `unresolved inside the band` or reverses order against the strictest incumbent, or where the 95% upper bound of $R_w \ge 1.0$ at any $w \in \{0, 0.1, 1.0\}$.

## 4. Disclosed Pre-Registration Pilots

Five small sizing/mechanism pilots were executed in `/tmp/ecbench-pilot` on seeds `999001..999005` (3–6 targets per curve, never reused here):
- Algebraic-track negative pilot (`seed 999001`): `pdp3-koblitz:m=3,engine=inherited-f4-qtail` (leaf Gray-code quadratic solver at $k \le 16$) on `icv1-f2m17-tm101-00378d4e` (`dim=6`) cut Macaulay calls `9,979 -> 6,159` (`-38%`) and hash probes `21.4M -> 16.7M` (`-22%`), but Gray-code exhaustive evaluation of $k = 11$ leaves (`7.66M` assignments) cost `15.3M` word XORs vs Macaulay degree-3 refutation's `~13.7M`, leaving total word XORs flat (`154.4M -> 156.0M`, `S: 1,419 -> 1,431`; on `icv1-f2m19-t797-b6cf2467`, `S: 5,274 -> 5,204`, `-1.3%`). Root cause: ~90% of `inherited-f4` word XORs on chained $S_3$ systems occur in the $2^1 + \dots + 2^5 = 62$ shallow degree-3 Macaulay rebuilds before $x_3$ is specialized. Rejected and reverted from `src/`.
- Frobenius $k$-sum mechanism and analytical-scale pilots (`seeds 999001..999003`) and generic-binary `sib=1` pilot (`seed 999005`): confirmed exact key-set preservation and motivated the scale-free $E = \mathrm{round}(X \sqrt{r / n})$ rule at $X = 1.35$ (`columns=2, m=4, free=1`) and $X = 1.0$ (`columns=1, m=5`).

## 5. Execution & Artifacts

Specs are generated deterministically by `node research/koblitz_frob_ksum_sib_20261118/make_specs.mjs`, checked via `./target/release/ecbench plan --spec`, and committed together with all `src/` changes **before** running any official session (`L0` counted measurement, sequential single-process runs on an otherwise idle VM, followed by `ecbench verify --replay-all` on every session). Because counted `S(w)` and `bitop/v1` gate counts are deterministic hardware-independent counts (verified by replay), primary claims rest on counted `S(w)` across $w \in [0, 1]$ and `bitop/v1`; L0 median wall times on this host are reported solely as secondary context, and no `LEADERBOARD.md` or `benchmarks/README.md` table rows are modified without a second-host replay certificate. [pcc:unverified]
