# Wider scans with sparse outer places

Every arm solved and verified all 44 inputs. At the doubled scan width, restricting the dense prefix to W beat expanding it to 2W on 18/24 genus-two, 9/12 genus-three and 4/4 distinct-width genus-four inputs. The other four genus-four cases already scanned the entire catalogue. The improvement in this matched comparison rarely made the wider scan faster than the original W scan in genus two.

| Genus | Wider scan, fixed dense prefix | Targets faster than W scan, excluding unchanged widths | Original normalized / candidate wall, whole genus |
|---|---|---:|---:|
| 2 | 2W, discard | 1/24 | 1.682–2.726× |
| 2 | 3W, discard | 0/24 | 1.167–2.603× |
| 2 | 2W, retry | 1/24 | 1.485–3.036× |
| 3 | 2W, discard | 2/12 | 2.353–3.252× |
| 3 | 3W, discard | 0/12 | 1.912–2.803× |
| 3 | 2W, retry | 5/12 | 2.621–3.454× |
| 4 | 2W, discard | 4/4 | 4.716–12.768× |
| 4 | 3W, discard | 4/4 | 4.754–12.830× |
| 4 | 2W, retry | 4/4 | 4.791–12.777× |

The retry arm admitted 818 of its 820 initially unready finite rows in genus two and all 589 in genus three. Compared with discarding at the same width, retries improved median time on 9/24 genus-two and 12/12 genus-three cases. Genus four had no rows to retry. Near-complete recovery therefore does not imply a faster solver than the original narrow collector. The doubled-scan discard arm required fewer anchors than W on every changed-width input. Its remaining cost includes scanning and processing more rows per anchor.

The only tenfold cells across widened arms were the same two genus-four p=31 inputs, where all widths clip to the same catalogue. These are repeats of the existing threshold cases. No policy reached tenfold throughout. CONTROL.txt contains 264 counted calls and 3,168 cyclic wall observations; summarize.rb reproduces the fixture comparisons.
