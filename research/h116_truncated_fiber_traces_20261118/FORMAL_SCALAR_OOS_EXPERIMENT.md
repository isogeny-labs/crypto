# H116 formal-scalar out-of-sample experiment

Protocol: `H116-FORMAL-SCALAR-OOS-001`.

Status: completed excluded control panel. The precommitted five-cell prediction
is false because one deterministic selector cell had no qualifying alternative
curve model. All four cells for which the frozen rule selected a model passed
every candidate and independent-verifier control. [pcc:unverified]

## Outcome

The unchanged formal-scalar source with SHA-256
`914c1909ffdd5e6b87a01f4c2da9de8faca0187da490cb56af323e105e64b6b4`
matched full degree-`r` Vélu maps through normalized x-map coefficient ten on
four new exact curve models. [pcc:unverified]

| p | r | d | selected `(A,B)` | selected j | result |
| ---: | ---: | ---: | --- | ---: | --- |
| 107 | 127 | 3 | none | none | selection failure after all 11,449 models |
| 211 | 197 | 4 | `(2,24)` | 98 | coefficients 0--10 equal |
| 241 | 257 | 4 | `(2,4)` | 61 | coefficients 0--10 equal |
| 431 | 397 | 3 | `(1,225)` | 126 | coefficients 0--10 equal |
| 587 | 547 | 3 | `(1,342)` | 424 | coefficients 0--10 equal |

The p=107 order-127 isogeny class supplied by the frozen cohort has no second
j-invariant among short-Weierstrass models over `F_107`. The cell was retained,
not replaced, and no map was constructed for it. This is a panel-selection
obstruction, not a mismatch of the candidate on a constructed map.
[pcc:unverified]

The four candidate/full-map prefixes are:

```text
p=211: [1,0,37,0,80,203,142,56,59,204,197]
p=241: [1,0,55,0,88,201,1,121,62,10,189]
p=431: [1,0,384,134,321,105,87,327,398,194,392]
p=587: [1,0,99,101,299,474,13,160,550,506,319]
```

Coefficients 0--8 also equal the independently implemented Bostan recurrence.
Coefficients 9--10 are new full-map comparisons not determined by that frozen
nine-coefficient recurrence control. [pcc:unverified]

## Separation and controls

The protocol and native oracle source were committed before execution. The v1
oracle retained the p=107 selection failure but terminated before attempting
the remaining cells. The v2 continuation did not rerun that cell; it only
serialized the retained failure and attempted cells two through five under the
unchanged rules. Both source gates and the first failure are retained.

The comparison program independently repeated deterministic model selection,
checked the frozen candidate/cohort hashes, and computed all candidate prefixes
before opening the oracle JSON. For each completed cell:

- full-map codomain scaling, cleared identity and normalization passed;
- a numerator-coefficient mutation failed the cleared identity;
- the candidate equaled the full-map quotient through coefficient ten;
- the candidate equaled the recurrence through coefficient eight;
- wrong scalar and omitted target scaling disagreed with the candidate/oracle;
- the independent verifier recomputed the selector, quotient, recurrence,
  first-order residual, cleared identity, scaling and mutation.

The independent verifier returned four matches in every completed control and
rejected a separate candidate coefficient-ten mutation. [pcc:unverified]

## Exact operation counts

The formal-scalar candidate used exactly 64,426 additions, 67,584
multiplications and 16 inversions on every completed curve at precision eleven.
The native full-map oracle grew from 3,560,372/3,130,760 base additions/
multiplications at `r=197` to 14,693,223/12,219,039 at `r=547`, plus extension
operations. These rows have different semantic obligations and implementations;
the counts are not a speed ratio. [pcc:unverified]

| r | candidate add | candidate mul | candidate inv | oracle base add | oracle base mul | oracle ext add | oracle ext mul | oracle ext inv |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 197 | 64,426 | 67,584 | 16 | 3,560,372 | 3,130,760 | 107,403 | 116,767 | 195 |
| 257 | 64,426 | 67,584 | 16 | 5,946,452 | 5,216,360 | 182,523 | 194,512 | 255 |
| 397 | 64,426 | 67,584 | 16 | 7,799,826 | 6,495,417 | 434,803 | 450,212 | 395 |
| 547 | 64,426 | 67,584 | 16 | 14,693,223 | 12,219,039 | 824,728 | 847,077 | 545 |

## Reproduction and evidence

```text
rustc --edition=2021 -O -D warnings h116_formal_scalar_oos_oracle.rs \
  -o /tmp/h116_formal_scalar_oos_oracle_v2
/tmp/h116_formal_scalar_oos_oracle_v2 > FORMAL_SCALAR_OOS_ORACLE.json
python3 h116_formal_scalar_oos.py COHORT_V2.json FORMAL_SCALAR_OOS_ORACLE.json \
  > FORMAL_SCALAR_OOS_RESULT.json
python3 h116_formal_scalar_oos_verify.py FORMAL_SCALAR_OOS_RESULT.json \
  FORMAL_SCALAR_OOS_ORACLE.json COHORT_V2.json
```

Evidence SHA-256 values:

```text
8fb0dad351eb1b7948e42f553b6e9ff7f33ae91527e4bd09d8566ed767fe4eaf  FORMAL_SCALAR_OOS_ORACLE.json
bf89357661d122371585a008cede2c521063d238292ff5c6defd1f33d526134d  FORMAL_SCALAR_OOS_RESULT.json
a84761b550a4118c7c2ec793cce71d534118151a20c10ee4e144ab8b44546fc4  h116_formal_scalar_oos.py
8823c6af2e9f1af45ab098d0469448520a42a0d28c68022f4f09ca288b637b33  h116_formal_scalar_oos_verify.py
```

The descriptive report, editable plot source, recorded CSV, SVG, optimized PNG
and inspected PDF are under
[`reports/run-20261118T0510Z/`](reports/run-20261118T0510Z/).

## Evidence boundary and next proof step

Together with the p=31 control, the same fixed candidate now matches five full
maps across five `(p,r,t)` classes and degrees 37 through 547. This is stronger
construction evidence at precision eleven, but it is still finite evidence.
The generic implication remains conditional on the elliptic facts
`V+pi=[t]`, `pi(z)=z^p`, formal-group subtraction, and the transfer of
`z^p`-divisibility through the normalized x-coordinate ratio.
[pcc:unverified]

The next proof target is a generic power-series lemma: if two normalized local
parameters agree modulo `z^p`, then their x-coordinate ratios agree modulo
`z^(p-1)`. That lemma can be formalized independently of the elliptic
endomorphism instance and composed later with the already checked degree-22
cutoff. [pcc:unverified]
