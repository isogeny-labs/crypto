# Frozen second five-rational-node configuration

`search_case2.py`, SHA-256 `e9aef41158ed7fee738435717730951fc9dcc127ce820803f887d500939217b2`, is the same exact matrix/normalization filter as the first source except for the fifth node `(1:2:1)` in place of `(0:1:1)` and a distinct schema. This is the second PGL3(F3) position case with five F3-rational intended nodes listed in Kudo–Harashita Section 4.1. The first four nodes are `(0:0:1),(0:1:0),(1:0:0),(1:1:0)`.

A 100,000-vector exploratory preflight checked execution and was not counted evidence. Freeze this source before a full enumeration of all `(3^13-1)/2 = 797161` projective coefficient classes. Apply the same ordinary-node and normalization point-count filters [4,10,28,82,314], with a direct F3/F9 evaluation control. A zero result excludes this fixed configuration with five ordinary nodes only. A hit requires independent geometry and L-polynomial replay.

Counted command: `sage -python search_case2.py > FULL_CASE2.json 2> FULL_CASE2.stderr`. Preserve all stage counts and raw output.
