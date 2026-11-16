from pathlib import Path
import importlib.util, sys, hashlib, subprocess, json, shutil

root=Path('/workspace/crypto')
source=Path('research/chained_jacobian_transfers_20261110/lean/ChainCompression.lean')
e=root/'research/chained_jacobian_transfers_20261110/evidence'
assert not (e/'chain-compression-claims-01.json').exists()
spec=importlib.util.spec_from_file_location('ct30_pcc',root/'tools/pcc/pcc.py')
pcc=importlib.util.module_from_spec(spec);sys.modules[spec.name]=pcc;spec.loader.exec_module(pcc)
claims=[];verdicts=[]
for name in ['ChainCompression.two_hop_double_cleared','ChainCompression.rational_two_hop_factorization']:
    found=pcc.lean_find_declaration([source],name);assert found is not None
    path,statement=found
    proof=dict(path=str(path),theorem=name,decl_sha256=hashlib.sha256(statement.encode()).hexdigest())
    args=[sys.executable,'tools/pcc/pcc.py','claim','new','--kind','theorem','--claim-type','interpretive','--statement',statement,'--scope','Faithful module consequence of explicit cleared map factors, middle norm-pullback4 and linearity. Rational cancellation requires explicit multiplication4 injectivity; actual fields, varieties, Picard, global Hom cancellation and geometry remain written.','--proof','lean:'+json.dumps(proof)]
    claim=subprocess.run(args,cwd=root,capture_output=True,text=True,check=True).stdout.strip()
    claims.append(dict(name=name,claim_path=claim,statement=statement,proof=proof))
    (e/'chain-compression-claims-partial-01.json').write_text(json.dumps(claims,indent=2)+'\n')
    result=subprocess.run([sys.executable,'tools/pcc/pcc.py','verify',claim,'--json'],cwd=root,capture_output=True,text=True,check=True)
    verdict=json.loads(result.stdout);assert verdict['verdict']=='pass';verdicts.append(verdict)
    print(name,claim,flush=True)
(e/'chain-compression-claims-01.json').write_text(json.dumps(claims,indent=2)+'\n')
(e/'chain-compression-local-verdicts-01.json').write_text(json.dumps(verdicts,indent=2)+'\n')
shutil.copyfile(root/source,e/'chain-compression-source-01.lean')
shutil.copyfile('/workspace/scratch/ct30-chain-compression-01.txt',e/'chain-compression-gate-01.txt')
gate=Path('/workspace/scratch/ct30-chain-compression-gate-01/research_chained_jacobian_transfers_20261110_lean_ChainCompression_lean')
for n in ['audit','build']:shutil.copyfile(gate/f'{n}.log',e/f'chain-compression-{n}-01.txt')
workflow=root/'.github/workflows/chained-jacobian-transfers.yml'
s=workflow.read_text();old='BalancedChart QuarticFibre; do';assert s.count(old)==1
workflow.write_text(s.replace(old,'BalancedChart QuarticFibre ChainCompression; do'))
lines=[]
for f in sorted((root/source.parent).glob('*.lean')):
    for i,line in enumerate(f.read_text().splitlines(),1):
        if line.startswith(('theorem ','lemma ')):lines.append(f'{f.relative_to(root)}:{i}:{line}')
assert len(lines)==138,len(lines)
(e/'lean-declaration-census-compression-20261112.txt').write_text('\n'.join(lines)+'\n')
print('PASS two individually bound local verdicts;138written/26future named workflow modules;scientific signing separate.',flush=True)
