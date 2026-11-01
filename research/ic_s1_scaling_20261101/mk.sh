#!/bin/bash
B=/Volumes/SSD990-2/s1-build/s1b-20261101; cd $B; export TMPDIR=$B/tmp
l=$1; mkdir -p bins/first/l$l
python3 /Volumes/SSD990-2/s1-build/wdsat_cell.py build anf/l$l --src /Volumes/SSD990-2/s1-build/wdsat_src --out bins/first/l$l --pad 5 --first > bins_l$l.log 2>&1 || echo FAIL $l
