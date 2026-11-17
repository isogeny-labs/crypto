from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.systematic_residue_20261116_v109.kernel import prune

def rewrite(source,outputs,roots,max_terms=64,max_support_work=8000000):
    assert max_terms>=2;G=Circuit(source.n);atoms=[1]+list(range(2,G.n+2));forms=[0,1]+[1<<(j+1)for j in range(G.n)];known={0:0,1:1,**{j+2:1<<(j+1)for j in range(G.n)}};cache={0:0,1:1,**{1<<(j+1):j+2 for j in range(G.n)}};recipes={};stats={'support_work':0,'cuts':0,'introduced_atoms':0,'emitted_recipes':0}
    def emit(form):
        visiting=set();stack=[(form,False)]
        while stack:
            f,expanded=stack.pop()
            if f in cache:continue
            a,b=recipes[f]
            if expanded:
                assert a in cache and b in cache;wire=G.xor(cache[a],cache[b]);cache[f]=wire;known.setdefault(wire,f);visiting.remove(f);stats['emitted_recipes']+=1
            else:
                if f in visiting:raise AssertionError('cyclicXORrecipe')
                visiting.add(f);stats['support_work']+=f.bit_count()
                if stats['support_work']>max_support_work:raise OverflowError('XORsupportworkcap')
                stack.extend([(f,True),(b,False),(a,False)])
        return cache[form]
    def atom(wire):
        if wire not in known:
            i=len(atoms);atoms.append(wire);known[wire]=1<<i;cache[1<<i]=wire;stats['introduced_atoms']+=1
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
            if form.bit_count()>max_terms:form=atom(G.xor(emit(fa),emit(fb)));stats['cuts']+=1
            elif form not in cache and form not in recipes:recipes[form]=(fa,fb)
        forms.append(form)
    oo=[{**row,'wires':[[emit(forms[w])for w in field]for field in row['wires']]}for row in outputs];rr=[[emit(forms[w])for w in row]for row in roots];H,result=prune(G,oo+[{'recovery_only':True,'wires':rr}]);return H,result[:-1],result[-1]['wires'],stats
