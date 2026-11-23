from research.relation_zero_locus_20261116_v103b.kernel import variable_truth
from research.table_relations_20261116_v103.kernel import features

def separating_subset(width,records,cap_bytes):
    masks=features(width,3);states=1<<width;assert ((states+7)//8)*(len(masks)+len(records)+width+16)<=cap_bytes
    one=(1<<states)-1;variables=[variable_truth(width,j)for j in range(width)];cache={0:one};vectors=[]
    for m in masks:
        if m:
            low=m&-m;cache[m]=cache[m^low]&variables[low.bit_length()-1]
        vectors.append(cache[m])
    candidates=list(dict.fromkeys(r for degree in records for r in degree['relations']));rejects=[];total=0
    for r in candidates:
        value=0
        for i in range(r.bit_length()):
            if r>>i&1:value^=vectors[i]
        rejects.append(value);total|=value
    remaining=total;chosen=[];history=[]
    while remaining:
        best=None;best_gain=0;best_cost=1
        for i,mask in enumerate(rejects):
            if i in chosen:continue
            gain=(mask&remaining).bit_count();cost=candidates[i].bit_count()+1
            if gain and(best is None or gain*best_cost>best_gain*cost):best=i;best_gain=gain;best_cost=cost
        assert best is not None;chosen.append(best);remaining&=~rejects[best];history.append({'candidate_index':best,'new_rejections':best_gain,'remaining':remaining.bit_count(),'cost':best_cost})
    selected=[candidates[i]for i in chosen];union=0
    for i in chosen:union|=rejects[i]
    assert union==total
    # Backward deletion is also exact; preserve deterministic original order.
    for i in list(reversed(chosen)):
        test=0
        for j in chosen:
            if j!=i:test|=rejects[j]
        if test==total:chosen.remove(i)
    selected=[candidates[i]for i in chosen];final=0
    for i in chosen:final|=rejects[i]
    assert final==total
    return selected,{'candidate_count':len(candidates),'selected_count':len(selected),'terms_before':sum(r.bit_count()for r in candidates),'terms_selected':sum(r.bit_count()for r in selected),'rejected_states':total.bit_count(),'history':history},one^total
