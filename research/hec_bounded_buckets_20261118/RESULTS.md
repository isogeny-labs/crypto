# Bounding the storage used for pencil slopes

Each audited qualifying slope and its complete ordered point list matched the reference lists under both bounded implementations. All four arms solved and verified all 44 inputs. The three lifted arms had identical anchors, admissions, joins and total field reduction counts on each target.

| Genus | Fixtures | Current / fixed-array time | Fixed faster | Current / two-pass time | Two-pass faster | Original / two-pass time |
|---|---:|---:|---:|---:|---:|---:|
| 2 | 24 | 0.970–1.131× | 23 | 1.006–1.131× | 24 | 2.030–3.295× |
| 3 | 12 | 0.990–1.069× | 10 | 0.935–1.095× | 8 | 2.746–3.738× |
| 4 | 8 | 0.945–1.006× | 4 | 0.972–1.014× | 2 | 3.485–10.141× |

The second traversal of cached slopes paid for itself relative to the current vector buckets on all genus-two cases in this timing run, and on only two genus-four cases. The gain reflects data handling on an identical relation stream. Ratios near one are single-run median comparisons. No host-independent performance claim follows. The two-pass arm exceeded tenfold only on the previously identified genus-four p=31 cases 0 and 1. The fixed-array variant exceeded tenfold on case 1 alone. Genus two still needs a substantially different source of speedup to reach the tenfold goal.

RUN.txt identifies the remote freeze. COMPARISON.tsv contains fixture-level medians expressed as ratios. CONTROL.txt retains 176 counted calls and 2,112 individual wall observations.
