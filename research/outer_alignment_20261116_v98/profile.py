import json,time
from .common import P,finish
from .kernel import family,covered,zero_pair
from research.separator_images_20261116_v91.kernel import Solver
from research.binary_rebracket_20261116_v96.kernel import solve as block
START=time.monotonic();out={'outer':[],'endpoint_blocks':0,'finite_w':0,'covered_w':0,'cell_only_rejects':0,'no_w_rejects':0,'verified_sample_roots':0};error=None
try:
 c=json.loads((P/'DATA.json').read_text())['actual'];F=Solver(c['n'],c['mod'],c['n']);C,cells=family(c['n'],c['mod'],c['d']);out['cells']=len(cells)
 def quad(a,b):return F.solve(*F.coefficients(a,b,1),'stream')
 for x1,x2,x5 in c['outer_samples']:
  aa=quad(x1,x2);bb=quad(x5,c['T']);row={'sources':[x1,x2,x5],'left_endpoints':aa,'right_endpoints':bb,'blocks':[]}
  for a in aa:
   for b in bb:
    ws=quad(a,b);hits=[covered(w,cells)for w in ws];z=zero_pair(F,a,b,c['d']);diag=a==b;reject=not diag and not z and all(hits)
    r=block({'n':c['n'],'mod':c['mod'],'d':c['d'],'a':a,'b':b,'B':1,'source_cap_bits':12,'diagonal_cap_bits':12,'pair_cap_bits':12},'rebracket')
    if reject:assert r['complete']and not r['roots']
    out['endpoint_blocks']+=1;out['finite_w']+=len(ws);out['covered_w']+=sum(hits);out['cell_only_rejects']+=int(reject and bool(ws));out['no_w_rejects']+=int(reject and not ws);out['verified_sample_roots']+=len(r['roots']);row['blocks'].append({'a':a,'b':b,'w':ws,'covered':hits,'zero_branch':z,'diagonal':diag,'complete':r['complete'],'root_count':r['exact_root_count'],'roots':r['roots'],'reject':reject})
  out['outer'].append(row)
 finish(out,None,START,'Frozen originalsource-generatedGF83outerimage:wholeblockcellcoverage withoriginalexceptions andunprunedblockverification.','If alignmentgivesnocuts,retirethisboundedfamily;keepconditionalblockkernel anddesignstrongerproduct-consistency certificates.')
except Exception as e:finish(out,repr(e),START,'Actualouter-imageprofile.','Retainallendpointbranches andinconclusivedata;noglobalrootcountfromsample.')
