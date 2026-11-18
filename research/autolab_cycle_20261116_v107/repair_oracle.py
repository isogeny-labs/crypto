from pathlib import Path
import json,runpy
from research.discriminant_roots_20261116_v106d.kernel import Field,gcd,solve
P=Path(__file__).resolve().parent;f=Field(3,[0,1]);checks=0
for padding in range(10):
 assert gcd([0,1,1],[0]*padding,f)==[0,1,1];assert gcd([0]*padding,[0,1,1],f)==[0,1,1];assert gcd([0]*padding,[0]*padding,f)==[];checks+=3
H=[0,0,1,1,1,1]
for method in ['direct','radical']:assert solve(H,f,method,1061611)['roots']==[[0,2],[2,1]];checks+=1
(P/'REGRESSION.json').open('x').write(json.dumps({'zero_padding_and_GF3_checks':checks,'H':H,'expected_roots_with_multiplicities':[[0,2],[2,1]],'passed':True})+'\n')
runpy.run_module('research.discriminant_roots_20261116_v106d.oracles',run_name='__main__')
