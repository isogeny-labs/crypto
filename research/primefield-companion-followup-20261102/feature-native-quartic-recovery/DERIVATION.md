# Conditional recovery from quartic signature means

2026-11-05. Bounded native development checks, extending merged PR3491.

For an odd prime r, let F be the previous polynomial for the mean of each length-q quadratic-character signature fiber. Suppose every nonsingleton fiber is a pair {a,b}. For each pair choose an extra offset e with chi(a+e)=-chi(b+e) nonzero. With mu=(a+b)/2 and beta=(a-mu)/chi(a+e), F(x)+beta chi(x+e) equals x at both members of that pair. On singleton fibers use F(x) unchanged. Matching a cached signature selects the correction; no unknown scalar is an input to evaluation. [pcc:unverified]

The public compiler enumerates the coefficient field, constructs the mean polynomial, and finds pair separators. Runtime Program retains polynomial coefficients, eight sign patterns, offsets and correction constants; it does not retain pair endpoint labels. These rules encode sparse residual information and their construction, storage and signature comparisons must be charged. Public field enumeration and dense polynomial synthesis remain setup costs. This construction does not establish compressed setup. [pcc:unverified]

## Native source integration

Source preparation is reusable across public targets. A consuming cursor retains the geometry tail and two homogeneous seed pairs. skip_sign advances the geometry and seed recurrence exactly as advance does, but omits the residual character consumer. The next evaluated sign therefore uses valid intervening state. Skipped signs are represented by None, distinct from the character value zero. The maximum extension budget is consumed by either operation. [pcc:unverified]

At r479, q14, the complete field-value check requests offset14 for12 inputs and offset15 for4 inputs. The other463 inputs request no extra sign. Thus the policy uses14*479+16=6722 sign values. For the four offset15 cases it omits the character at14, while still executing two geometry and seed steps. Counts describe the policy and its branches, not a timing comparison. [pcc:unverified]

Twenty known-answer public-point fixtures on E/F509, y^2=x^3+2x+10, P=(3,68), r479, include all16 exceptional scalars and0,1,2,478. One prepared source is reused. Test-only point generation receives labels; recovery receives only the public point. Recorded output contains16 extra signs,20 geometry steps and4 skipped signs. Each extension converts24 points, giving480 converted points across this fixture set. The output direct_phases column counts extensions only; initial-window phase work and reusable preparation are outside that column. Initial and extension character-consumer calls are also checked by test-only counters. [pcc:unverified]

## Validation and limits

The native suite records60 passing tests, seven compiler warnings and22 recursive source/data locks. It includes all479 coefficient-field inputs, all20 public-point fixtures, old eight fixture identities, cursor budget/error controls, and inherited geometry/seed/polynomial tests. Four skipped-character cases compare retained seeds against a complete16-sign window and assert no character-consumer call occurs on the skipped step. [pcc:unverified]

The separate check.rs imports no producer modules. It builds quadratic characters from a square bitmap, reconstructs every signature fiber, evaluates the465-term polynomial and eight correction rules on all479 inputs, and independently generates public points by repeated addition. It checks the20 recovery rows against those points and the route policy. A changed correction coefficient and a changed recovered scalar are both rejected. Metadata agreement alone is not execution proof; native test counters cover the skip branch locally. This checker was authored in the producing session and has no independent-review standing. [pcc:unverified]

The full479-input check is algebraic; public source integration covers20 targets, not all479. Original4563-case cohort and prior artifacts remain unchanged. No cloud job or comparative timing panel ran. Next bind complete costs for compilation, preparation, initial characters, correction selection, extension geometry and residual evaluation; broader scaling needs a smaller public representation or cheaper sign source.

## Sources checked

Read feature-native-quartic-collapse/DERIVATION-LABELED.md and feature-native-adaptive-extension/routed/source.rs. NIST DLMF27.9 (https://dlmf.nist.gov/27.9), retrieved2026-11-05, fixes the quadratic-character convention. The correction identity follows directly from the displayed equations; no novelty claim is made. Sources checked (KB): MCP tools unavailable; scripts/research-kb status failed because no executable was installed. Used the named repository evidence as fallback.
