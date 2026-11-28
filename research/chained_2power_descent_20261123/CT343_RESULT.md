# CT343: exact even-order cyclic toy-DLP scalar lifting over F103

**2026-11-23 UTC; producer/coordinator-only exact finite arithmetic, [pcc:unverified].** This run follows merged CT339–CT342 and the [preregistered CT343 protocol](CT343_PROTOCOL.md), committed at `36fd8942f1b1ee02f9726c53234e581c612e9004` **before** CT343 implementation and science execution. Its scope is distinct from the original Jacobian norm-point reuse and cold solver measurement reservations.

## New explicit public subgroup and result

On `C/F103: y²=∏_{a=1}^6 (x-a)`, write `D=(10,15)-W1` with `W1=(1,0)` and define
- `H23=[100]D`, order 23; normalized Mumford `U=[74,42,1], V=[85,61]`.
- `H25=[368]D`, order 25; normalized Mumford `U=[11,48,1], V=[60,35]`.
- `K=W6-W1`, order 2; `U=[41,1], V=[]`; actual Cantor arithmetic checks `Φ(K)=O`.
- `G46=H23+K`, order 46, with `Φ(G46)=((51,25),O)` of order 23.
- `G1150=H23+H25+K`, order 1150, with `Φ(G1150)=((51,25),(51,58))` of order 575.

The preregistered deterministic order-25 witness search took its **first** eligible source point `(10,15)`; no fallback cohort or target-dependent choice occurred. These concrete generators and their prime-factor order tests are in the frozen result JSON.

| Cyclic source subgroup | Image subgroup order | Public toy DLP targets exactly recovered | False source lifts rejected |
|---|---:|---:|---:|
| `<G46>`, order 46 | 23 | **46/46** | 46/46 |
| `<G1150>`, order 1150 | 575 | **1,150/1,150** | 1,150/1,150 |
| **Total** | — | **1,196/1,196** | **1,196/1,196** |

For each `Q=[k]G`, a fresh elliptic-product BSGS receives only `Φ(G)`, `Φ(Q)` and the image order `m`, yielding `r=k (mod m)`. The two source-side candidates `r` and `r+m` were **both checked with actual genus-two Cantor scalar multiplication** against the public Q; exactly one reproduces Q. The log recovery therefore succeeds even when an order-two kernel makes the forward rational map noninjective. The emitted source oracle retains k only for the final known-answer check and for constructing public Q.

Example on order 1150: `Q=[575]G` maps to the product identity, so the destination solver returns `r=0`; the source check rejects candidate 0 and accepts 575. This is precisely the correctness ambiguity missed by a naive “isogeny preserves logs” claim. The general fact is `ker(Φ|<G>)` is killed by 2 when `ΨΦ=[2]`, hence has size at most two in a cyclic subgroup. The experiment supplies a concrete nontrivial-kernel witness.

## Execution, controls and resource accounting

- **Frozen source:** [ct343_even_cyclic.py](ct343_even_cyclic.py), SHA-256 `a49724dc7cff4ea66b0f1be5e2dec783e1c8aa44b0d0408c14702b23be9626f7`, Git blob `81cfe6131b5e75836bb9efadb3c308ae946ec26d`.
- **Frozen raw:** [CT343_RESULT.json](CT343_RESULT.json), SHA-256 `a91edcb81ec7d5a277335329affa2ff7349a4ae075ca237551a273e057edee12`, Git blob `50fe53915b0b2c69434309aa0727ba64fb01239b`.
- **Pinned unchanged base evaluator:** CT339 source SHA-256 `c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695`, archived on main as [ct339_f103_toy_dlp.py.gz.b64](ct339_f103_toy_dlp.py.gz.b64). The new script refuses execution if the extracted CT339 source hash is different.
- **Exact local commands:** `python3 ct343_even_cyclic.py > ct343_normal.json`; `python3 -O ct343_even_cyclic.py > ct343_opt.json`; `cmp ct343_normal.json ct343_opt.json`. Both successful streams were identical to frozen JSON. Required mutants `--drop-kernel`, `--poison-target`, `--skip-source` all exited nonzero in normal and optimized Python. The former violates order 46, the second has no mapped odd-subgroup log after a nonzero rational 2-torsion addition, and the last falsely accepts a second source scalar lift.
- **Separate-language destination check:** [ct343_target_crosscheck.cjs](ct343_target_crosscheck.cjs), SHA-256 `4f7d8419c2068844243232f81cb547d1603ca82c16487078f66ff42917fa29f3`, Git blob `5eb94aa98dd09ed3588f9f5a36c4c001b48717a3`. It recomputes both elliptic cubic equations, generator membership, exact image orders and all 1,196 mapped cyclic scalar repetitions **without importing the Python implementation**. [Output](CT343_TARGET_CROSSCHECK.json), SHA-256 `a5d94a65ec7d8824dc01beeadbc25358306cabbde3a1440172ce72b85415c30e`, Git blob `af076372020ac4d08c1c4c5e70db434e71409f3c`. This validates the destination arithmetic only, **not** the Cantor source arithmetic independently.
- **Primitive counters across the entire two-cohort producer run (not comparable solver timing):** 369,978 elliptic additions, 46,489 Jacobian additions, 1,199 forward map calls. Counts include setup, fresh BSGS tables, source construction and verification under CT339 instrumentation, and cannot be compared with optimized rho/BSGS/IC without matched pricing and a direct-source baseline. The first run ended with a serialization `TypeError: 'K' object is not subscriptable`; only result formatting was corrected, with the failure preserved here. A combined mutant shell command exceeded its timeout after two accepted negative controls; the third was run separately and also rejected in both interpreter modes. None of these failed attempts is a successful science result.

### Reproduce from merged CT339 on main

```sh
cd research/chained_2power_descent_20261123
base64 -d ct339_f103_toy_dlp.py.gz.b64 | gzip -d > ct339.py
printf '%s  %s\n' c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695 ct339.py | sha256sum -c -
python3 ct343_even_cyclic.py > /tmp/ct343.json
python3 -O ct343_even_cyclic.py > /tmp/ct343-opt.json
cmp /tmp/ct343.json /tmp/ct343-opt.json
cmp /tmp/ct343.json CT343_RESULT.json
node ct343_target_crosscheck.cjs CT343_RESULT.json > /tmp/ct343-node.json
cmp /tmp/ct343-node.json CT343_TARGET_CROSSCHECK.json
for flag in drop-kernel poison-target skip-source; do
  if python3 ct343_even_cyclic.py --"$flag"; then echo "unexpected accept $flag"; exit 1; fi
  if python3 -O ct343_even_cyclic.py --"$flag"; then echo "unexpected optimized accept $flag"; exit 1; fi
done
```

## Sources, limitations and handoff

Internal: CT339–CT342 original reports and frozen sources, merged [#5045](https://github.com/isogeny-labs/crypto/pull/5045) and [#5060](https://github.com/isogeny-labs/crypto/pull/5060), root [#3877](https://github.com/isogeny-labs/crypto/pull/3877). External primary: [Magma's Richelot documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1615) explicitly treats degree-four Jacobian isogenies with `(Z/2)^2` kernels and possible elliptic product codomains; [Diem (2006)](https://arxiv.org/abs/math/0312413) establishes degree-two genus-two-to-elliptic-cover geometry. These are standard mechanisms; **no novelty claim**.

Actual roles: one coordinator conducted mathematical construction, Python implementation, producer self-checks, and separately written Node target arithmetic. Native child workers and Rust compiler were not exposed, so there is **no independently staffed validation**, no Rust Cantor replay, no signed PCC standing and no formality claim. The ~2MB shared `docs/session-logs/2026-11-23.md` cannot be safely read through this connector and has a known concurrent conflict history; task-local [CT343_SESSION_LOG.md](CT343_SESSION_LOG.md) and [HANDOFF.md](HANDOFF.md) keep new work without rewriting the shared file. Research-KB and Conductor unavailable; fallback sources above.

The constructed curve is **deliberately bielliptic**. The result establishes a correct finite toy-DLP reduction for cyclic subgroups of its Jacobian, not a faster complete ordinary prime-field elliptic-curve discrete logarithm solver. Original 695/701 authenticated fixture, physical `C169/C13a/C13b/N2_actual/D` prices, paid EKS STOP, locked vault, matching optimized rho/BSGS/IC and independent signed replay remain reserved to their owners. No paid compute, benchmark, merge, CI rerun, or new automation ran.

**Next:** a source-pinned *separately owned* Rust/Cantor reimplementation of the 2-primary map kernel and all public lifts, then independent signed PCC. Only the original cost owner should execute fully charged same-host candidate/baseline comparisons under approved resources.
