"""Independently verify one saved witness per curve with public Magma.

Network is required. The public mathematical witness data and this verification
code are submitted to the public calculator linked by magma-maths.org/calc/.
The local reference implementation is not used to compute the matrix in Magma.
"""
import json
from pathlib import Path
from urllib.request import Request,urlopen

root = Path(__file__).resolve().parent
data = json.loads((root/'results.json').read_text())
lines = ['SetColumns(0);']
for row in data['cases']:
    result = row['guess_determine']
    if 'witness' not in result: continue
    W = result['witness']
    p,r,n,ell = row['p'],row['r'],row['n'],row['ell']
    def seq(values): return '['+','.join(map(str,values))+']'
    points = ','.join('E!'+seq([*d['point'],1]) for d in W['records'])
    flatK = [x for Krow in W['K'] for x in Krow]
    C = seq([i+1 for i in W['zero_minor_columns']])
    Z = seq([i+1 for i in W['point_sum_indices']])
    lines.extend([
        f'F:=GF({p}); E:=EllipticCurve([F!{row["a"]},F!{row["b"]}]);',
        f'assert #E eq {r}; P:=E!{seq([*row["P"],1])}; Q:=E!{seq([*row["Q"],1])};',
        f'Points:=[{points}];',
        f'Powers:=[]; for ua in [0..{n}] do for ub in [0..{n}-ua] do Append(~Powers,<ua,ub>); end for; end for;',
        'M:=Matrix(F,#Points,#Powers,&cat[[R[1]^u[1]*R[2]^u[2]:u in Powers]:R in Points]);',
        f'K:=Matrix(F,{ell},{2*ell},{seq(flatK)});',
        f'assert Rank(M) eq {ell}; assert Rank(K) eq {ell}; assert IsZero(K*M);',
        f'C:={C}; Z:={Z};',
        f'N:=Matrix(F,{ell},{ell},&cat[[K[i,j]:j in C]:i in [1..{ell}]]);',
        'assert Determinant(N) eq 0;',
        'assert &+[Points[i]:i in Z] eq E!0;',
        f'w:=Integers()!(Integers({r})!{W["alpha"]}/Integers({r})!{W["beta"]});',
        f'assert w eq {result["log"]}; assert w*P eq Q;',
        f'print "PASS independent witness p={p} order={r}";',
    ])
lines.append('print "ALL SIX INDEPENDENT WITNESS CHECKS PASSED";')
code = '\n'.join(lines)+'\n'
(root/'verify_witnesses.magma').write_text(code)
request = Request('https://calculator.magma-maths.org/execute',
                  data=json.dumps({'code':code}).encode(),
                  headers={'Content-Type':'application/json',
                           'Origin':'https://magma-maths.org'})
with urlopen(request,timeout=55) as response:
    result = json.load(response)
(root/'witness-magma.json').write_text(json.dumps(result,indent=2)+'\n')
print(result.get('stdout',result))
print('Runtime:',result.get('magma'))
assert result.get('success') is True,result
assert result.get('stdout','').count('PASS independent witness') == 6
