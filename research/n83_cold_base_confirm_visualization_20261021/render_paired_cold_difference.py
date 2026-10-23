#!/usr/bin/env python3
"""Render/check the paired K1103-K1107 cold-process graph."""
import argparse, json
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
SOURCE=ROOT/"research/n83_cold_base_confirm_20261021/results/analysis.json"
OUTPUT=HERE/"paired-cold-difference.svg"
def render(a):
    it=a["paired_interval"]; rows=a["cold_process_ms_by_point"]; diffs=it["differences_ms"]
    if a.get("status")!="COLD_WINNER_CONFIRMED_WITHIN_PAIR" or a.get("selection_admitted") is not True or len(rows)!=16 or len(diffs)!=16:
        raise ValueError("requires the admitted 16-point result")
    pts=[]
    for row,d in zip(rows,diffs):
        actual=row["cold_process_ms"]["k1103"]-row["cold_process_ms"]["k1107"]
        if abs(actual-d)>1e-6: raise ValueError("point differences disagree")
        pts.append((row["point_index"],actual/1000))
    mean,lo,hi=(it[k]/1000 for k in ("mean_ms","lower_95_ms","upper_95_ms"))
    if abs(mean-sum(v for _,v in pts)/16)>1e-9: raise ValueError("mean disagrees")
    W,H,L,R,T,B=1000,570,105,950,75,440
    def x(i): return L+(i-1)*(R-L)/15
    def y(v): return B-v*(B-T)/2.2
    s=['