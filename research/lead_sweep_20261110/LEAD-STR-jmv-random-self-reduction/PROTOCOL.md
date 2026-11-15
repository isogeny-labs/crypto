# LEAD-STR-jmv-random-self-reduction

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_known (last reviewed 2026-10-24). Family: structure.
- Title: Jao-Miller-Venkatesan isogeny-expander random self-reduction within a level.
- Next-step excerpt (truncated): "Revisit when: A use of the self-reduction as a subroutine of an attack (not a hardness proof) is ..."
- Recorded obstruction (atlas_survey.md line 54): "vacuous at q=2^256 for delta<=0.5".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 52-66, 321-323 only);
  atlas_tally.json matched but was not read. The atlas YAML is encrypted here; the mechanism is taken from my own knowledge of JMV (2005).

## 2. Hypothesis
Used as an attack subroutine, the JMV self-reduction (walk L steps in an isogeny-class Cayley graph from the target curve, ask a delta-density oracle there) succeeds per attempt with probability delta*(1 +/- 0.15) when the generator set is an expander (k ~ log2 h random generators, L = 4 log2 h), for both random and target-adjacent oracle sets; the expected cost per DLP is therefore L/delta oracle-plus-walk units, so the reduction adds only a factor L and gives no gain over the oracle's own cost. Without expansion (k=1) the hit rate is not delta.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio = P_hit(L)/delta, estimated from 20000 walks per cell, h in {16411, 65537, 262147}, delta=0.05, sets "random" and "interval centred on the start".
- Prediction: |ratio-1| <= 0.15 in every expander cell. Outcome consistent => the recorded closure stands (reduction is a neutral L-factor wrapper). Outcome falsified (control OK, expander cells outside band) => the toy model of the closure is wrong; reopen the toy-size mixing question.
- Positive control: k=1 generator (+-1), interval set around start; must give ratio > 3 (detector sees non-mixing).
- Negative control / null: random subset oracle S (no structure to exploit) on the expander; plus a second-seed random cell.
- A/A arm: each cell is split into two independent halves with different RNG streams; |ratio_1-ratio_2| must be <= 0.30, else verdict inconclusive.
- Verdict: inconclusive if the positive control or A/A fails; else consistent or falsified by the band above.

## 4. Method
Class group modelled as cyclic Z/h (ordinary-curve level Cayley graph). Generators: k=ceil(log2 h) uniformly random elements of Z/h (k=1 and g=1 for control), steps +-g chosen uniformly, L=4*ceil(log2 h), start 0 = target curve. Fixed seeds 1000*i+{7,17,99,55}, halves +1/+2. Pure Python, about 20-60 s, a few MB RAM, one core. Run: `python3 -I run.py`; `--smoke` uses h=1021, 400 walks.
It does not model actual isogeny computation cost.

## 5. Classification: runnable_local
A stdlib toy decides the mixing statistic cheaply. Reason it is not desk_only: the closure's content (neutral L/delta overhead) can be measured at toy size.

## 6. What a run cannot establish
- Nothing about real isogeny graphs: they are Cayley graphs only up to level structure and the class group need not be cyclic; real generators are small primes with costly Velu steps.
- Nothing at q=2^256: it does not re-derive the recorded vacuity (needs delta > mixing error plus a per-curve oracle that does not exist); toy h is 2^14-2^18.
- It shows mixing, not that any oracle with density delta exists; the ECDLP is generic in a prime-order group (cf. closed_obstructed isogeny-class invariance leads).
- Results are labelled [pcc:unverified]; one fixed seed set only.
