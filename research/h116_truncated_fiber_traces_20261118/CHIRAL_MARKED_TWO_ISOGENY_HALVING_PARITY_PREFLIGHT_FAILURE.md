# H116 marked two-isogeny halving preflight failure

The first pushed source gate, commit `d36a3fc69c`, froze protocol
`H116-CHIRAL-MARKED-TWO-ISOGENY-HALVING-PARITY-001` with an incorrect
total-feature assertion.  Its first permissible execution exited before
constructing rules, selecting a mask, or scoring any scalar labels:

```text
$ /tmp/h116_chiral_marked_two_isogeny_halving_parity --run-kummer CHIRAL_MARKED_TWO_ISOGENY_HALVING_PARITY_RESULT.json
h116 chiral branch residual parity error: marked two-isogeny feature abstention
```

The cause is structural: 88 of the 6,080 potential branch-pair quartics are
singular, matching the inherited trace and `I,J` panels' 5,992 admitted-model
census.  Protocol `-002` changes only this case from a fatal totality
assertion to the already frozen singular-model abstention convention.  The
observable on every smooth model, rule family, statistic, shuffle count and
controls are unchanged.  No result JSON was emitted by the failed gate.
[pcc:unverified]
