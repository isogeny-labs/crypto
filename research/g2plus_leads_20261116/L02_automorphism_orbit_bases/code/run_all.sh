#!/bin/bash
cd /home/user/crypto/research/g2plus_leads_20261116/L02_automorphism_orbit_bases
R=code/runcmd.sh
for k in 10 11 12 13 14; do for arm in A0 A B; do o=raw/ic_F5_b${k}_${arm}.jsonl; $R $o python3 code/run_ic.py --instances raw/instances_F5.jsonl --bits $k --arm $arm --seeds 1,2,3 --out $o; done; done
for k in 8 9 10; do for arm in A0 A B; do o=raw/ic_F7_b${k}_${arm}.jsonl; $R $o python3 code/run_ic.py --instances raw/instances_F7.jsonl --bits $k --arm $arm --seeds 1,2,3 --out $o; done; done
for k in 10 11 12 13 14; do for arm in A B; do o=raw/ic_C5_b${k}_${arm}.jsonl; $R $o python3 code/run_ic.py --instances raw/instances_C5.jsonl --bits $k --arm $arm --seeds 1,2,3 --out $o; done; done
for k in 8 9 10; do for arm in A B; do o=raw/ic_C7_b${k}_${arm}.jsonl; $R $o python3 code/run_ic.py --instances raw/instances_C7.jsonl --bits $k --arm $arm --seeds 1,2,3 --out $o; done; done
for k in 10 11 12 13 14; do for arm in Rneg Raut; do o=raw/rho_F5_b${k}_${arm}.jsonl; $R $o python3 code/run_rho.py --instances raw/instances_F5.jsonl --bits $k --arm $arm --seeds 1-24 --out $o; done; done
for k in 8 9 10; do for arm in Rneg Raut; do o=raw/rho_F7_b${k}_${arm}.jsonl; $R $o python3 code/run_rho.py --instances raw/instances_F7.jsonl --bits $k --arm $arm --seeds 1-12 --out $o; done; done
for k in 10 11 12 13 14; do for arm in Rneg Raut; do o=raw/rho_C5_b${k}_${arm}.jsonl; $R $o python3 code/run_rho.py --instances raw/instances_C5.jsonl --bits $k --arm $arm --seeds 1-6 --out $o; done; done
for k in 8 9 10; do for arm in Rneg Raut; do o=raw/rho_C7_b${k}_${arm}.jsonl; $R $o python3 code/run_rho.py --instances raw/instances_C7.jsonl --bits $k --arm $arm --seeds 1-6 --out $o; done; done
echo DONE > raw/.alldone
