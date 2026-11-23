#!/usr/bin/env python3
"""Print one instance of lfd_small.make_instance as a line for emin.c:
   n k poly a6 z v_1 ... v_k
Usage: python3 emit_instance.py N SEED [K]
"""
import sys
from lfd_small import make_instance

n, seed = int(sys.argv[1]), int(sys.argv[2])
k = int(sys.argv[3]) if len(sys.argv) > 3 else None
inst = make_instance(n, seed, k)
print(n, inst['k'], inst['K'].poly, inst['a6'], inst['z'], *inst['V'])
