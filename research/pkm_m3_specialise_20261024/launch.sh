#!/bin/bash
# Two queues (PROTOCOL.md section 4). Run with nohup; progress.txt records each start/end.
cd "$(dirname "$0")"; P=pkm_kummer_p2013265921; C=${P}_m3_t5_x1289903598; M=${P}_m4_t5_x362838726; T0=${P}_m3_t7_x221026520
( CAP_KB=16777216 ./run_all.sh ${P}_m3_t7_x704078485__F0; ./run_all.sh ${T0}__H2 ${T0}__H1 ${T0}__M1 ${T0}__M2 ) &
( ./run_all.sh ${C}__F0 ${C}__H2 ${C}__H1 ${C}__H1b ${C}__M1 ${C}__M1b ${C}__M2 ${M}__H1 ${M}__M1 ${M}__M2 ${P}_m3_t8_x1227422598__M2 ${P}_m3_t8_x1227422598__H2 ) &
wait
