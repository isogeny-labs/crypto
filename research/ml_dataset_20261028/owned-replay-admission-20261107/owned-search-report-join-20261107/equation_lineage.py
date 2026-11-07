"""Bounded equation lineage relative to captured normalized generators only."""
from dataclasses import dataclass

@dataclass(frozen=True)
class Limits:
    work: int = 2_000_000
    partitions: int = 4096
    nodes: int = 16384
    variables: int = 65536

class Refused(Exception): pass
class Mismatch(Exception): pass
class Meter:
    def __init__(self, limits): self.limits, self.work = limits, 0
    def pay(self, n=1):
        if n > self.limits.work-self.work: raise Refused('work cap')
        self.work += n
    def equal(self,a,b):
        self.pay()
        if type(a) is not type(b): return False
        if isinstance(a,list):
            if len(a)!=len(b): return False
            return all(self.equal(x,y) for x,y in zip(a,b))
        if isinstance(a,str): self.pay(len(a)+len(b))
        return a==b

def need(test, message):
    if not test: raise Mismatch(message)
def uint(x): return type(x) is int and 0 <= x <= 0xffffffffffffffff
def array(x): need(type(x) is list, 'expected list'); return x
def constraint(v,b): return [[],[v]] if b else [[v]]
def canonical(polys,n,m):
    for p in array(polys):
        m.pay(); previous=None
        for t in array(p):
            m.pay(); last=-1
            for v in array(t):
                m.pay(); need(uint(v) and last<v<n,'invalid monomial'); last=v
            key=(len(t),t)
            if previous is not None:
                m.pay(len(previous[1])+len(t));need(previous<key,'noncanonical polynomial')
            previous=key

def compare_seq(actual, expected, partial, m, label):
    array(actual);array(expected);m.pay()
    need(len(actual)<=len(expected) if partial else len(actual)==len(expected),label+' length')
    for a,b in zip(actual,expected):need(m.equal(a,b),label+' mismatch')

def check_lineage(normalized, continuation, variables, *, limits=Limits()):
    """No mutation. Pass means equation ancestry only, never solve/UNSAT/LFD."""
    if not isinstance(limits,Limits) or any(type(v) is not int or v<0 or v>cap for v,cap in [(limits.work,100_000_000),(limits.partitions,65536),(limits.nodes,65536),(limits.variables,1_000_000)]):
        raise ValueError('invalid lineage limits')
    m=Meter(limits); checked_p=checked_n=partial=0; verified_nodes=[]; skipped_nodes=[]
    result={'scope':'equation-lineage-relative-to-captured-normalized',
            'verified_nodes':verified_nodes,'skipped_nodes':skipped_nodes,
            'original_descent_verified':False,'polynomial_identities_replayed':False,
            'solution_or_unsat_verified':False,'lfd_verified':False}
    try:
        need(uint(variables),'invalid variable count')
        if variables>limits.variables:raise Refused('variable cap')
        canonical(normalized,variables,m)
        if continuation is None:raise Refused('no retained continuation')
        need(type(continuation) is dict,'continuation type')
        ps=array(continuation['partitions'])
        if not ps:raise Refused('no retained partitions')
        if len(ps)>limits.partitions:raise Refused('partition cap')
        for i,p in enumerate(ps):
            m.pay();need(type(p) is dict,'partition type')
            prefix=array(p['prefix']);m.pay(len(prefix));need(len(prefix)<=variables and all(type(b) is bool for b in prefix),'prefix type')
            building=p['status']=='Building'
            need(p['status'] in ('Building','Ready','Running','Candidate','Splitting','Excluded','Refuted'),'partition status')
            parent=p['parent']
            if i==0:need(parent is None and not prefix,'root partition ancestry')
            else:
                need(uint(parent) and parent<i,'partition parent')
                q=ps[parent];need(q['status'] in ('Splitting','Excluded'),'partition parent not splitting');children=array(q['children']);m.pay(len(children));need(children.count(i)==1,'partition parent backlink')
                ordinal=children.index(i);search=q['search']
                need(type(search) is dict,'split parent search absent')
                w=search['witness_node'];nodes=array(search['nodes'])
                need(uint(w) and w<len(nodes),'split parent witness missing')
                model=array(nodes[w]['candidate']);m.pay(len(model));need(len(model)==variables and all(type(v) is bool for v in model),'split model length/type')
                fixed=len(q['prefix']);j=fixed+ordinal;need(j<variables,'split child ordinal')
                need(m.equal(model[:fixed],q['prefix']),'split model prefix')
                m.pay(j+1);expected=model[:j]+[not model[j]]
                compare_seq(prefix,expected,building,m,'partition prefix')
            children=array(p['children']);m.pay(len(children))
            need(all(uint(c) and i<c<len(ps) and ps[c]['parent']==i for c in children),'partition children')
            need(len(set(children))==len(children),'duplicate partition child')
            if p['status']=='Excluded':need(len(children)==variables-len(prefix),'excluded partition child count')
            if p['status'] not in ('Splitting','Excluded'):need(not children,'nonsplitting partition has children')
            if p['status']=='Splitting':partial+=1
            m.pay(len(normalized)+len(prefix));expected=normalized+[constraint(v,b) for v,b in enumerate(prefix)]
            canonical(p['equations'],variables,m)
            compare_seq(p['equations'],expected,building,m,'partition equations')
            if building:partial+=1
            checked_p+=1
            search=p['search']
            if search is None:continue
            need(not building,'search on building partition')
            ns=array(search['nodes'])
            if not ns:partial+=1
            if checked_n+len(ns)>limits.nodes:raise Refused('node cap')
            for k,node in enumerate(ns):
                m.pay();building_node=node['status']=='Building'
                need(node['status'] in ('Building','Ready','Running','Branching','Branched','Refuted','Witness'),'node status')
                if k==0:
                    need(node['parent'] is None,'root node parent')
                    expected_eq=p['equations'];m.pay(len(expected_eq));origins=[f'Original({j})' for j in range(len(expected_eq))];decisions=[]
                else:
                    par=node['parent'];need(uint(par) and par<k,'node parent')
                    q=ns[par];need(q['status'] in ('Branching','Branched'),'node parent not branching');cs=array(q['children']);need(len(cs)==2,'binary child slots');m.pay(2)
                    need(cs.count(k)==1,'node backlink');value=cs[1]==k
                    ds=array(q['decisions']);m.pay(len(ds))
                    for d in ds:need(type(d) is list and len(d)==2 and uint(d[0]) and d[0]<variables and type(d[1]) is bool,'decision type')
                    used={d[0] for d in ds};need(len(used)==len(ds),'duplicate decisions')
                    variable=0
                    while variable in used:m.pay();variable+=1
                    need(variable<variables,'no branch variable')
                    m.pay(len(q['equations'])+len(q['origins'])+len(ds))
                    expected_eq=q['equations']+[constraint(variable,value)]
                    origins=q['origins']+[f'Branch {{ parent: {par}, variable: {variable}, value: {str(value).lower()} }}']
                    decisions=ds+[[variable,value]]
                canonical(node['equations'],variables,m)
                compare_seq(node['equations'],expected_eq,building_node,m,'node equations')
                compare_seq(node['origins'],origins,building_node,m,'node origins')
                need(len(node['origins'])==len(node['equations']),'origin equation alignment')
                compare_seq(node['decisions'],decisions,building_node,m,'node decisions')
                cs=array(node['children']);need(len(cs)==2,'node children length');m.pay(2)
                for c in cs:need(c is None or (uint(c) and k<c<len(ns) and ns[c]['parent']==k),'child forward reference')
                need(cs[0] is None or cs[0]!=cs[1],'duplicate binary child')
                if node['status']=='Branching':partial+=1
                if node['status'] not in ('Branching','Branched'):need(cs==[None,None],'nonbranching node has children')
                need(cs[0] is None or cs[1] is not None,'false child built before true')
                if node['status']=='Branched':need(all(c is not None for c in cs),'branched missing child')
                if all(c is not None for c in cs):need(cs[1]<cs[0],'true-before-false construction order')
                if building_node:
                    partial+=1;skipped_nodes.append({'partition':i,'node':k,'reason':'builder incomplete'})
                else:verified_nodes.append({'partition':i,'node':k})
                checked_n+=1
        result.update(status='inconclusive_partial' if partial else 'verified_lineage',reason='retained builder prefixes' if partial else None)
    except Refused as e:result.update(status='inconclusive',reason=str(e))
    except (Mismatch,KeyError,TypeError,IndexError,ValueError) as e:result.update(status='lineage_mismatch',reason=str(e))
    if result['status'] not in ('verified_lineage','inconclusive_partial'):verified_nodes.clear()
    result.update(work=m.work,partitions_checked=checked_p,nodes_checked=checked_n,partial_builders=partial)
    return result
