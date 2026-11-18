from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.systematic_residue_20261116_v109.kernel import prune

def rewrite(source,outputs,roots,max_terms=16,max_support_work=8000000):
    assert max_terms>=2;G=Circuit(source.n);atoms=[1]+list(range(2,G.n+2));forms=[0,1]+[1<<(j+1)for j in range(G.n)];known={0:0,1:1,**{j+2:1<<(j+1)for j in range(G.n)}};cache={0:0,1:1,**{1<<(j+1):j+2 for j in range(G.n)}};stats={'support_work':0,'cuts':0,'introduced_atoms':0}
    def emit(form):
        if form not in cache:
            stats['support_work']+=form.bit_count()
            if stats['support_work']>max_support_work:raise OverflowError('XORsupportworkcap')
            value=0;left=form
            while left:
                bit=left&-left;value=G.xor(value,atoms[bit.bit_length()-1]);left^=bit
            cache[form]=value;known.setdefault(value,form)
        return cache[form]
    def atom(wire):
        if wire not in known:
            index=len(atoms);atoms.append(wire);known[wire]=1<<index;cache[1<<index]=wire;stats['introduced_atoms']+=1
        return known[wire]
    for node in source.nodes[source.n+2:]:
        kind,a,*rest=node;fa=forms[a]
        if kind=='and':
            fb=forms[rest[0]]
            if fa==0 or fb==0 or fa^fb==1:form=0
            elif fa==1 or fa==fb:form=fb
            elif fb==1:form=fa
            else:form=atom(G.op('and',emit(fa),emit(fb)))
        else:
            assert kind in ['xor','not'];fb=forms[rest[0]]if rest else 1;form=fa^fb
            if form.bit_count()>max_terms:
                wire=G.xor(emit(fa),emit(fb));form=atom(wire);stats['cuts']+=1
        forms.append(form)
    oo=[{**row,'wires':[[emit(forms[w])for w in field]for field in row['wires']]}for row in outputs];rr=[[emit(forms[w])for w in row]for row in roots];H,result=prune(G,oo+[{'recovery_only':True,'wires':rr}]);return H,result[:-1],result[-1]['wires'],stats
