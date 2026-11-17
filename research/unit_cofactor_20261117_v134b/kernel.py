from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.degree_clipped_checks_20261117_v125c.kernel import isolated
from research.residue_quotient_20261116_v99b.kernel import apply

def embed(z,p):return (z&((1<<p)-1))|((z>>p)<<(p+1))
def compress(x,p):return (x&((1<<p)-1))|((x>>(p+1))<<p)
def scalar_reconstruct(z,p,v,h):return embed(z,p)^(v if h else 0)
def original_readback(chart,theta):return chart['particular']^apply(chart['columns'],theta)

def clone(source,targets,C,mapped):
 needed=set();stack=list(targets)
 while stack:
  i=stack.pop()
  if i in mapped or i in needed:continue
  needed.add(i);node=source.nodes[i];assert node[0]in ['not','and','xor'];stack.extend(node[1:])
 for i in sorted(needed):
  kind,a,*rest=source.nodes[i];mapped[i]=C.op(kind,mapped[a],mapped[rest[0]]if rest else None)
 return [mapped[w]for w in targets]

def transform(source,outputs,roots,target,direction,pivot,max_nodes=100000):
 assert 0<direction<1<<source.n and direction>>pivot&1
 C=Circuit(source.n-1);x=[0 if i==pivot else 2+i-(i>pivot)for i in range(source.n)];co={0:0,1:1,**{i+2:w for i,w in enumerate(x)}};h=clone(source,[target],C,co)[0];cofactor_nodes=len(C.nodes)-2;theta=[C.xor(w,h)if direction>>i&1 else w for i,w in enumerate(x)];mapped={0:0,1:1,**{i+2:w for i,w in enumerate(theta)}};mapped[target]=0
 flat=[w for row in outputs for field in row['wires']for w in field]+[w for group in roots for w in group];clone(source,flat,C,mapped);assert len(C.nodes)<=max_nodes,'transformednodecap'
 oo=[{**row,'wires':[[mapped[w]for w in field]for field in row['wires']]}for row in outputs];rr=[[mapped[w]for w in group]for group in roots];G,oo,allroots=isolated(C,oo,rr+[theta,[h]])
 return G,oo,allroots[:-2],allroots[-2],allroots[-1][0],{'direction_hex':hex(direction),'pivot':pivot,'source_inputs':source.n,'free_inputs':G.n,'cofactor_nodes_before_final_prune':cofactor_nodes,'unpruned_nodes':len(C.nodes)-2,'pruned_nodes':len(G.nodes)-2}

def accepted(values,outputs):return not any(values[w]for row in outputs for field in row['wires']for w in field)
def word(values,wires):return sum(values[w]<<i for i,w in enumerate(wires))
