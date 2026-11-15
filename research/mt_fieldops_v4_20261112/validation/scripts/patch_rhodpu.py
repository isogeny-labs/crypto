#!/usr/bin/env python3
"""VALIDATOR SENSITIVITY ARM (not pre-registered): adds arm `rhodpu` = rhodp with 1024 DISTINCT step values.
rhodp draws its 1024 steps A_i = B_j + B_k with j, k uniform over the 64-point bank, so about 39% of the
steps share their value with another index; a sign flip followed by a different index with the same value
is a fruitless 2-cycle that the look-ahead rule (same index only) does not catch, and the walk is then
abandoned after 20 * 2^d counted steps (1-34% of rhodp's steps by size and offset).  rhodpu draws distinct
unordered pairs j < k instead (same 1024 counted additions); nothing else changes.  It is applied on top of
patch_ic5rw.py's output.  usage: patch_rhodpu.py <in main.rs> <out main.rs>"""
import sys
src = open(sys.argv[1]).read()
a = src.index('        "rhodp" => {'); b = src.index('        "bsgs2" => {')
blk = src[a:b].replace('"rhodp" => {', '"rhodpu" => {', 1)
old = """            for _ in 0..R {
                let (i, j) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                arms.push((f.padd(bank[i].0, bank[j].0), (bank[i].1 + bank[j].1) % n));
            }"""
assert blk.count(old) == 1
blk = blk.replace(old, """            let mut used = std::collections::HashSet::new();
            while arms.len() < R {
                let (i, j) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                if i == j || !used.insert((i.min(j), i.max(j))) {
                    continue;
                }
                arms.push((f.padd(bank[i].0, bank[j].0), (bank[i].1 + bank[j].1) % n));
            }""")
src = src[:b] + blk + src[b:]
open(sys.argv[2], "w").write(src)
