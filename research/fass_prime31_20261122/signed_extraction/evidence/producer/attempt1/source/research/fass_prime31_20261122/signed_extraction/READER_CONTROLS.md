# Fixed signed-extraction reader controls

This panel is specified before native execution. It uses the same reader core
as ordinary saved-object checking. Every mutation starts from a fresh in-memory
copy of the original complete 42-case directory; producer files remain unchanged.
The reader independently binds all 132 raw case identities and the exact 42
persisted input/output pairs. Raw records bind both byte lengths and SHA-256
digests. Bare mode still binds the complete expected fixture registry and
mathematical outputs, without trusting a producer's kind or success label.

## Semantic controls (43, each in two reading modes)

Each of the following must be rejected both with raw byte binding and without
raw records. A missing required control fixture is a failure, not a skip.

1. Change the input case identity.
2. Change the input prime.
3. Change curve coefficient A.
4. Change curve coefficient B.
5. Change selector degree d.
6. Substitute the other valid target ordinate.
7. Flip the F restriction policy.
8. Remove the configured cyclic subgroup.
9. Replace its generator by its inverse, retaining the same generated group.
10. Replace its exact order by a nonminimal annihilating order.
11. Remove an entire input/output pair.
12. Add an extra input/output pair.
13. Remove one input sidecar only.
14. Replace a known case's filenames by an unknown case's filenames.
15. Change the output case identity.
16. Omit a selector root.
17. Omit an F root.
18. Omit a G root.
19. Change a point ordinate in the signed base.
20. Duplicate a rational two-torsion base point.
21. Omit a point from the exact cyclic subgroup inventory.
22. Change a coefficient of the quotient H.
23. Change the quotient support count.
24. Omit a left collision bucket.
25. Omit an eliminated witness root from a left bucket.
26. Change the left witness gcd.
27. Omit a right collision bucket.
28. Omit an eliminated witness root from a right bucket.
29. Change the right witness gcd.
30. Omit the noncancelling (P,P,P) triple in the order-four control.
31. Add an invalid ordered triple.
32. Duplicate an ordered triple.
33. Change a finite prefix's chart to the identity-prefix chart.
34. Change its finite prefix abscissa.
35. Change its left-bucket backpointer.
36. Change its right-bucket backpointer.
37. Omit a permutation-group witness backpointer.
38. Change an integer row coefficient.
39. Change a row's target coefficient.
40. Omit an integer-row witness backpointer.
41. Remove all identity-prefix triples.
42. Remove all affine triples on the selector-target cancellation coordinate mask.
43. Collapse signed collisions by keeping only the first finite signed triple
    per pair of left/right coordinate-bucket indices; retain identity-prefix
    triples. The known order-four control must have a removed collision.

The ordinary mutation fixture is `order4-d1-minus-all`: P=(1,14) on
y^2=x^3+x over F97, target -P=(1,83). Here 2P=(0,0). Both (P,P,P) and
(-P,-P,P) have finite prefix abscissa zero and the same coordinate buckets;
(-P,P,-P) has an identity prefix. This fixture makes controls 30, 41, 42 and
43 exercise the distinct omitted branches without selecting a fixture using
new native output. Subgroup mutations use `sub5-d96-plus-liftable`; the
two-torsion duplicate uses `rank97-d2-x1-all`.

## Raw-only controls (5)

The following must be rejected with raw binding enabled:

1. Add harmless whitespace to an input sidecar without updating its raw binding.
2. Change its recorded SHA-256.
3. Change its recorded byte length.
4. Remove its recorded SHA-256 entry.
5. Remove its recorded byte-length entry.

The first 43 controls also exercise certificate byte binding when their
mutated object is an output, and sidecar binding when it is an input.

## Positive controls (3)

Check the original complete directory with raw records; check it in bare
mode; check the whitespace-only sidecar change in bare mode. The latter is
semantically identical and must pass when exact byte binding was not requested.

Acceptance requires 86 semantic-mode rejections, five raw-only rejections,
and all three positive checks: **91 rejections and three positives**. All
reader arithmetic uses the independent affine oracle and input-binding code;
none of these calls invokes the producer extractor, norm constructor or
quotient solver. Arithmetic work counters are not promoted by this panel to
independently priced cost measurements. [pcc:unverified]
