# Candidate-carrier point-count filter, frozen before replay

Source `verify_carriers.sage` SHA-256 `03f64d80d79f971fe51fbae48b28796aac2fe96870424d46f3a0bfb76496a2f9`.

Independently count points of the two known hyperelliptic carrier models with f masks 9735 and 17371 over F3^r for r=1,...,5. For each x, count one y if f(x)=0, two if f(x) is a nonzero square, and zero otherwise, plus the rational point at infinity. Derive the first five coefficients of the genus-five L-polynomial from Newton sums and complete it by the functional equation. Expected point counts [4,10,28,82,314], expected L-polynomial coefficients [1,0,0,0,0,14,0,0,0,0,243]. Any discrepancy fails the filter construction.

This replay confirms the exact isogeny-class target for a later nonhyperelliptic carrier search. It does not produce such a carrier, an elliptic cover, a subgroup map, or a DLP cost. A candidate nonhyperelliptic genus-five curve must have these five point counts and a separately verified smooth model. Only then search maps to the source E over F3^5 and assess any GHS transfer.

Prior work checked before search: Kudo–Harashita, *Representation of non-special curves of genus 5 as plane sextic curves and its application to finding curves with many rational points* (2023), https://arxiv.org/pdf/2102.07270, Section 4.2 uses an F9-point threshold of at least 32; the target class has 10. The LMFDB class `5.3.a_a_a_a_o` records the target polynomial and a Jacobian but does not supply a nonhyperelliptic model. These observations establish a scoped search gap, not publication priority or vulnerability.
