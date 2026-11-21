"""Independent exact-arithmetic replay of the frozen 27-shift rows."""
import gzip
import hashlib
import itertools
import json
from collections import Counter
from fractions import Fraction as Q
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
BASE = ROOT / "xedni_local_collision_crt_20261120/candidate-rows.jsonl.gz"

def sha(data):
    return hashlib.sha256(data).hexdigest()

def modq(z, p):
    assert z.denominator % p
    return z.numerator * pow(z.denominator, -1, p) % p

def curve(points, k):
    (x1,y1),(x2,y2) = [tuple(map(Q, P)) for P in points]
    a1,a2,a3 = [Q(47*z) for z in k]
    d1 = y1*y1+a1*x1*y1+a3*y1-x1*x1*x1-a2*x1*x1
    d2 = y2*y2+a1*x2*y2+a3*y2-x2*x2*x2-a2*x2*x2
    a4 = (d1-d2)/(x1-x2)
    return a1,a2,a3,a4,d1-a4*x1

def discriminant(a):
    a1,a2,a3,a4,a6 = a
    b2=a1*a1+4*a2
    b4=a1*a3+2*a4
    b6=a3*a3+4*a6
    b8=a1*a1*a6+4*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4
    return -b2*b2*b8-8*b4*b4*b4-27*b6*b6+9*b2*b4*b6

def relation(points, a):
    (x1,y1),(x2,y2) = [tuple(map(Q,P)) for P in points]
    a1,a2,a3,a4,a6 = a
    den=2*y2+a1*x2+a3
    if den == 0:
        return False
    slope=(3*x2*x2+2*a2*x2+a4-a1*y2)/den
    intercept=(-x2*x2*x2+a4*x2+2*a6-a3*y2)/den
    xd=slope*slope+a1*slope-a2-2*x2
    yd=-(slope+a1)*xd-intercept-a3
    return xd==x1 and yd==-y1-a1*x1-a3

def main():
    result=json.loads((HERE/"result.json").read_text())
    protocol=json.loads((HERE/"protocol.json").read_text())
    assert sha((HERE/"run.py").read_bytes())==result["script_sha256"]
    assert sha((HERE/"protocol.json").read_bytes())==result["protocol_sha256"]
    source_bytes=SOURCE.read_bytes()
    base_bytes=BASE.read_bytes()
    assert sha(source_bytes)==result["source_sha256"]==protocol["source_sha256"]
    assert sha(base_bytes)==result["base_candidates_packed_sha256"]==protocol["base_candidates_packed_sha256"]
    source=[json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    bases=[row for row in map(json.loads,gzip.decompress(base_bytes).splitlines())
           if row["pair_index"] in protocol["selected_source_indices"]]
    assert len(bases)==600
    packed=(HERE/"candidate-rows.jsonl.gz").read_bytes()
    raw=gzip.decompress(packed)
    assert sha(packed)==result["candidate_rows_packed_sha256"]
    assert sha(raw)==result["candidate_rows_raw_sha256"]
    rows=[json.loads(line) for line in raw.splitlines()]
    assert len(rows)==16200
    shifts=list(itertools.product((-1,0,1),repeat=3))
    counts,by_pair=Counter(),Counter()
    maxbits=0
    for i,row in enumerate(rows):
        bi,si=divmod(i,27)
        base=bases[bi]
        shift=shifts[si]
        assert row["base_index"]==bi and tuple(row["shift"])==shift
        assert row["pair_index"]==base["pair_index"]
        points=source[row["pair_index"]]["lifted_points"]
        dx=points[0][0]-points[1][0]
        p=7 if dx%7==0 else 11
        assert dx%(p*p)!=0
        step=77*p
        k=[base["k"][j]+step*shift[j] for j in range(3)]
        assert row["step"]==step and row["k"]==k
        a=curve(points,k)
        assert row["coefficients"]==list(map(str,a))
        assert discriminant(a)!=0
        assert all(tuple(modq(z,q) for z in a)==tuple(base[f"local_{q}"]) for q in (7,11))
        assert tuple(modq(z,47) for z in a)==(0,0,0,46,45)
        bits=max(max(abs(z.numerator).bit_length(),z.denominator.bit_length()) for z in a)
        assert bits==row["height_bits"]
        maxbits=max(maxbits,bits)
        found=relation(points,a)
        assert row["status"]=="constructed" and row["rational_relation"]==found
        counts["constructed"]+=1
        counts["relations"]+=int(found)
        by_pair[str(row["pair_index"])]+=1
    assert dict(counts)==result["counts"]
    assert dict(by_pair)==result["per_pair_candidates"]
    assert maxbits==result["max_height_bits"]
    pospacked=(HERE/"positive-rows.jsonl.gz").read_bytes()
    posraw=gzip.decompress(pospacked)
    assert sha(pospacked)==result["positive_rows_packed_sha256"]
    assert sha(posraw)==result["positive_rows_raw_sha256"]
    assert len(posraw.splitlines())==counts["relations"]
    print(json.dumps({"status":"PASS","candidate_rows":len(rows),
                      "relations":counts["relations"],"max_height_bits":maxbits,
                      "candidate_rows_raw_sha256":sha(raw)},sort_keys=True,indent=2))

if __name__=="__main__":
    main()
