# Route 3 — exact commands

Binary (verified before every script; `sha256` printed in each store header):

```
BIN=/Volumes/SSD990-2/.codex/worktrees/crypto-pkm-map-sweep-20261123/target/release/ic
shasum -a 256 "$BIN"
# 1ae5170f9b3362f75a636909e1f5d01fb0ead0fd4886627d6012e2b9de9d8d08
```

All commands run from `/Volumes/SSD990/llm/tmp/opencode/subexp_routes/route3`.
Nothing in the git repository was modified.

## 1. m=2 reference panel (13 bits, 16 repeats x 1000 trials, 5 s, 1 rho run)

```
python3 run_panel.py --bin "$BIN" --seed 20261185 --bits 13 --m 2 \
  --repeats 16 --max-trials 1000 --budget 5 --degree 8 \
  --store m2_m3_13bit_20261185.jsonl --group m2_ref
```

which issues, in order interval, candidate, candidate, interval:

```
$BIN bench --bits 13 --family generic --seed 20261185 --repeats 16 \
  --targets walk --factor-base prime-abscissa:below=32 \
  --oracle "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain" \
  --max-trials 1000 --rho-runs 1 --solver-budget-seconds 5 --json

$BIN bench --bits 13 --family generic --seed 20261185 --repeats 16 \
  --targets walk --factor-base "prime-isogeny:t=5,seed=5,aux=ordinary" \
  --oracle "pkm-semaev:m=2,engine=f4,degree=8,mode=chain" \
  --max-trials 1000 --rho-runs 1 --solver-budget-seconds 5 --json
```

Preflights (`--oracle subtract --max-trials 1`) confirm both arms build an
18-abscissa base on the same instance `icv1-fp13-tm13-9e2d11f6`.

## 2. m=3 F4 panel, same instance and base (equal size 18)

```
python3 run_panel.py --bin "$BIN" --seed 20261185 --bits 13 --m 3 \
  --repeats 2 --max-trials 2 --budget 5 --degree 8 \
  --store m2_m3_13bit_20261185.jsonl --group m3_f4_censored
```

Trial counts are 2 x 2, not 16 x 1000: at 5 s every m=3 call is censored, so a
16 x 1000 panel would run ~4.7 h per call and still report only censoring. The
reduction is stated, not hidden; the m=3 numbers are charged work *without
resolution*.

## 3. m=3 F4 at the next T down (T=4, 11 abscissae) — resolves

```
$BIN bench --bits 13 --family generic --seed 20261185 --repeats 2 \
  --targets walk --factor-base "prime-isogeny:t=4,seed=5,aux=ordinary" \
  --oracle "pkm-semaev:m=3,engine=f4,degree=8,mode=chain" \
  --max-trials 2 --rho-runs 0 --solver-budget-seconds 5 --json > m3_t4_b5.json

# same with --solver-budget-seconds 30 > m3_t4_b30.json   (identical ops: deterministic)
```

`mode=full` was also tried and is unusable: the S_4 system is 15 vars / 16 eqs
with a degree-12 leading equation, and the F4 degree bound 8 declines it in
0.02 s (0 ops).

## 4. Binary's own exact m=2 pair-sum yield on a 13-bit prime curve

```
$BIN prime --curve toy:13,1 --size 18 --trials 20000 --controls 8 --json \
  > prime_toy13_s18.json
```

## 5. Exact sumset / pruning counts (pure integer EC arithmetic, no timing)

```
python3 sumset_sweep.py      # T=4..7 sumset sizes and yields -> sumset_sweep.json
python3 structure_probe.py   # generic base + subgroup-coset control -> structure_probe.json
```

## 6. m-general cost model

```
python3 cost_model.py        # -> cost_model.json (exponent table, three engines)
```

## 7. Store

```
python3 make_store.py        # append-only index -> route3_store.jsonl
```
