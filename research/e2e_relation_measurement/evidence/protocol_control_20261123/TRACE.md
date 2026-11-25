# One complete recorded relation path

This is a fixed public correctness control on `y²=x³+x+32` over `F_101`, with subgroup order 101, `P=(4,10)` and `Q=(82,27)=[17]P`. The scalar is part of the known-answer setup. The seed is 42, mode is independent, block size is 10 and both blocks use the full x interval. Its [raw result](result.json), [Singular output](stdout), [original audit](audit.json), and [later seed-aware audit](seed_replay_audit.json) are retained. All observations below concern this one control, not a benchmark comparison. [pcc:unverified]

The target draw produced `(a_R,b_R)=(81,14)` and `R=(51,90)=81P+14Q`. Of 23 candidates, 20 were accepted and three were rejected as duplicate x-coordinates; none were filtered or the identity. Target generation plus candidate acquisition called scalar multiplication 48 times and point addition 24 times. These are instrumented high-level calls, excluding their internal field arithmetic. [pcc:unverified]

One of the three recorded signed witnesses uses accepted-point indices 8 and 15, numbered from zero: [pcc:unverified]

| Input | Coefficients | Full point | Sign |
| --- | --- | --- | --- |
| P₈ | `(11,27)` | `(67,65)` | −1 |
| P₁₅ | `(57,75)` | `(75,2)` | −1 |

Full point arithmetic checks `-P₈-P₁₅=R`. Subtracting the target coefficients gives `A=53`, `B=86` modulo 101, so this record checks `53P+86Q=O`. The given scalar 17 satisfies that coefficient identity and `[17]P=Q`. All three witnesses have nonzero B and their supplied scalars pass the recorded point check. The later independent audit reproduced the recorded seeded stream using its local Python 3.13.1 runtime; the producer recorded Python 3.14.3. That match covers this stream only, not general cross-version compatibility. [pcc:unverified]

The input polynomial total degrees are `(4,10,10)` with `(9,11,11)` recorded nonzero terms. The exact first-zero degree of this generator top-form ideal is 11; it is not the backend solving degree. The returned basis has maximum output degree 2, which is another distinct quantity. The bracketed Singular trace reports product-criterion count 17 and chain-criterion count 82; those are not field-operation totals. Solver solving degree and solver field-operation counts were not exposed. [pcc:unverified]

The single process reported the following phase intervals. They are retained to show the instrumentation, not to establish a speed ranking. [pcc:unverified]

| Phase | Wall milliseconds |
| --- | ---: |
| Generation | 0.729042 |
| Polynomial construction | 1.694917 |
| Top-form diagnostic | 0.014542 |
| Gröbner basis | 1.830583 |
| Root extraction | 167.143208 |
| Relation verification | 0.473958 |
| Sum of listed phases | 171.886250 |
| Driver internal interval | 242.337917 |

The 70.451667 ms difference lies outside the named phases and includes setup and other driver work; the internal interval is not whole-process time. The process reported 133.723 ms CPU and process-lifetime peak RSS 252,461,056 bytes (240.765625 MiB). The later audit recorded 5.475 ms wall for its internal interval, excluding its interpreter startup and receipt writing; it was run separately after the control and is not included in the original driver interval. Root completeness, solver field-operation counts, matched resource controls and a complete single-target reference remain outside this record. [pcc:unverified]
