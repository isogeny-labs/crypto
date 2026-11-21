# H116 regular-unit substitution source gate

No regular-substitution result was generated before this gate.
[pcc:unverified]

```text
1af64543e8df2fa9b8338c5f0c3673ca4522eb52a2c209712fc0194a381e43bb  REGULAR_SUBSTITUTION_PROTOCOL.json
484f839b802f6a6b6b5687d11d53649b5d54dfefcebcd82f3555cde7e20da11b  h116_regular_substitution_control.py
2b123a2af2b0c9e4adb8e55b6a12e4412ee99fb875798d7c602b41f178a3cb0c  h116_regular_substitution_verify.py
```

The producer implements truncated multiplication, inversion and substitution.
The verifier imports none of those routines; it checks retained indices and
coefficients against the three frozen first-variation formulas. The all-or-
nothing result includes 20 zero-correction, constant-outer and nonunit-outer
control triplets. [pcc:unverified]
