# Frozen nontrivial isogeny coordinate control for proposal 16

Frozen 2026-11-24 (America/Los_Angeles), draft PR #5165, before this producer run. Proposal #16 posits that walks on supersingular isogeny graphs yield multiplicative relations among target-point coordinates. This bounded check executes one certified, nonidentity quotient edge and tests both the actual point homomorphism and a literal coordinate-product identity. It does **not** execute a random walk or provide a general endomorphism-based relation collector. The existing identity-edge coordinate counterexample is in [EC leads protocol](EC_LEADS_001_034_PROTOCOL.md).

Fixed public input: `p=1019`, `E:y²=x³+x` over `F_p`, expected supersingular trace zero and `#E=p+1=1020`; rational order-two kernel `K=(0,0)`; PARI/GP `ellisogeny(E,K)` gives the quotient model and rational map `(f/h²,g/h³)`. Select the first liftable `x≥1` whose deterministic square-root point has order >12 and whose `P,2P,3P` avoid the kernel and infinity. Set `Q=2P`, `R=P+Q`. Guard source/codomain cardinalities, point membership, exact map additivity `φ(P)+φ(Q)=φ(R)`, and kernel-coset identity `φ(P+K)=φ(P)`. Report the three signed source and image points and compare `x(φ(P))x(φ(Q))` with `x(φ(R))`. No point or target is changed after observing output. The same GP source acts as a verified coordinate-control producer, not a same-target solver benchmark.

The user's renewed instruction to run public `E(F_p)` leads and earlier explicit local bypass authorize this bounded local run while `scripts/checkpoint.sh gate` remains failed (`.resume/RESUME.md` missing; earlier S3 `InvalidAccessKeyId`). This is a local-only exception, not a passed checkpoint. No external target or cloud resource is used. Stop on GP stderr, missing completion sentinel, map/group mismatch, or nonzero shell exit; preserve any failure without retry. Frozen source `EC_ISOGENY_016_GP.gp` SHA-256 is `fcf038bda5b366a0b9aea626c8139fa846f5ccebf632d540ca3f705fe0d9f7e7`; PARI/GP is 2.17.3 on arm64 macOS. Invoke once with:

```sh
gp -q < research/prime_field_100_ideas_20261123/EC_ISOGENY_016_GP.gp > research/prime_field_100_ideas_20261123/EC_ISOGENY_016.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_ISOGENY_016.stderr.txt
```

Write the shell status immediately to `EC_ISOGENY_016.exit.txt`. Results and hashes follow only after observation. `[pcc:unverified]`

## One-shot local result

The frozen source ran once with GP 2.17.3. Shell exit was `0`, stderr was empty, and the completion sentinel appeared. The script guarded `#E=#E'=1020` and the order-two kernel. Its deterministic first admissible point was `P=(4,219)` of order 85; `2P=(589,923)`, `3P=(571,635)`. The quotient map sent these to `(259,269)`, `(871,879)`, and `(687,390)`, respectively. All coordinates are modulo 1019. Exact `elladd` verified `φ(P)+φ(2P)=φ(3P)`, and `φ(P+K)=φ(P)` verified the kernel coset. However, `259·871 ≡ 390 (mod 1019)` differs from `x(φ(3P))=687`.

| Preserved artifact | SHA-256 |
|---|---|
| [Source](EC_ISOGENY_016_GP.gp) | `fcf038bda5b366a0b9aea626c8139fa846f5ccebf632d540ca3f705fe0d9f7e7` |
| [Stdout](EC_ISOGENY_016.stdout.txt) | `78dca12e77dc6a69b67158f4bba027f6f2491cda1c685beb9c0842cf7ba901db` |
| [Stderr](EC_ISOGENY_016.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Exit record](EC_ISOGENY_016.exit.txt) | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |

This is a direct one-edge supersingular isogeny transport/control and a counterexample to the literal image-abscissa product rule for these points. It does not assess a composed endomorphism path, any different algebraic coordinate identity, relation yield, or a complete index-calculus cost. `[pcc:unverified]`
