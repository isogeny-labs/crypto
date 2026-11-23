# Requirement ledger

| Requirement | Evidence | Status |
| --- | --- | --- |
| Arbitrary Mumford pushforward | residue-field factor/orbit implementation in `cover_transfer.rs` | implemented; replay required |
| Nonsplit support | evaluation in `k[t]/(g)` for irreducible degree 2 or 3 | implemented; replay required |
| Independent norm cross-check | all 486 GF(8) table classes compared at table construction | implemented; replay required |
| Homomorphism | all 236,196 ordered GF(8) class pairs | test present; replay required |
| `phi_*phi^*=[3]` | typed fibers plus pinned n=83 general-norm replay | implemented; replay required |
| `e^2=[3]e` | arbitrary-class certificate and exhaustive GF(8) test | implemented; replay required |
| r-torsion image/kernel separation | normalized projector certificate | implemented; replay required |
| Prym Frobenius/order factors | exact GF(2), GF(4), GF(8) counts and recurrence | analytically closed; native replay required |
| Shared target prime | three exact integer gcds | rejected: every gcd is one |
| Shared elliptic isogeny factor | source trace versus traces `±2^42` | rejected by finite-field isogeny criterion |
| Full genus-3 index calculus | prior destination screen | rejected |
| Rational fiber-structured base at n=9,13,19,23 | cube-map injectivity for odd n | rejected: distinct size unchanged, collision rate zero |
| Higher-degree image-compatible relation calculus | strict cost gate | still open; no speedup claim |
| Formal identity | `Projector.lean` and identity certificate | source supplied; replay required |
| Illustrated report | DOT/SVG/PDF assets | supplied |
