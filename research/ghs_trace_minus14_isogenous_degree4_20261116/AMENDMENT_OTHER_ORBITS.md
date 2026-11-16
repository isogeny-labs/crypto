# Pre-run extension to the remaining trace -14 source orbits

The earlier `class_count.json` lists twenty trace -14 normal-form source
equations `v²=u³+u²+b` over `K=F3[a]/(a^5+2a+1)`. Direct Frobenius cubing
partitions their b masks into four five-element orbits:

```text
11 17 71 147 238
47 187 202 81 117
38 172 223 53 241
80 153 190 156 217
```

The first two are paired by rational degree-two isogenies; the last two
are also paired (38 maps to 217, and 80 maps to 172). The prior corrected
degree-four screen covered mask 11 and the present first arm covered 202,
so masks 38 and 80 are the remaining source-orbit representatives.

Before executing them, `screen_other.sage` is frozen to use the same twelve
carrier masks, four normalized degree-four denominator classes, CRT root
assignments, and full-equation check as `screen.sage`. It accepts only b
masks 38 and 80 and writes distinct `other_<b>_<carrier>.json` receipts.
Each cell should enumerate 59,697 assignments. A zero-hit result for all
24 new cells, together with the first two representatives, transports by
Frobenius to all 240 exact source/carrier equation pairs in the recorded
slice; it would still be a finite degree-four obstruction, not a GHS
vulnerability classification.
