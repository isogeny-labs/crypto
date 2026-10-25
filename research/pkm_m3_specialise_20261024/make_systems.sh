#!/bin/bash
# Builds systems/ from the committed unspecialised systems (frozen arms of PROTOCOL.md section 4).
set -e; cd "$(dirname "$0")"; mkdir -p systems
SRC=../ic_next10_20261014/03-pkm-degree/systems; P=pkm_kummer_p2013265921
sp() { python3 specialise.py $SRC/${P}_$1.ms systems/${P}_$1__$2.ms "${@:3}"; }
# control N=15 (t=5): head order 2^5, mid (k=2) order 2^3
C=m3_t5_x1289903598
cp $SRC/${P}_$C.ms systems/${P}_${C}__F0.ms
sp $C H1  y0_0=5:1
sp $C H1b y0_0=5:3
sp $C H2  y0_0=5:1 y1_0=5:3
sp $C M1  y0_2=3:1
sp $C M1b y0_2=3:3
sp $C M2  y0_2=3:1 y1_2=3:3
# control (b): m=4, N=20 (t=5), recorded D>=8
M4=m4_t5_x362838726
sp $M4 H1 y0_0=5:1
sp $M4 M1 y0_2=3:1
sp $M4 M2 y0_2=3:1 y1_2=3:3
# target N=21 (t=7): head 2^7, mid (k=3) order 2^4
T1=m3_t7_x704078485
cp $SRC/${P}_$T1.ms systems/${P}_${T1}__F0.ms
T=m3_t7_x221026520
sp $T H1 y0_0=7:1
sp $T H2 y0_0=7:1 y1_0=7:3
sp $T M1 y0_3=4:1
sp $T M2 y0_3=4:1 y1_3=4:3
# N=24 (t=8): mid (k=4) order 2^4
U=m3_t8_x1227422598
sp $U M2 y0_4=4:1 y1_4=4:3
sp $U H2 y0_0=8:1 y1_0=8:3
