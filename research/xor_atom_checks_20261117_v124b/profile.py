from pathlib import Path
import json,time,random,hashlib,gc
from collections import Counter
from .common import P,finish
from .kernel import rewrite
from research.root_verifier_checks_20261117_v123b.kernel import compile_all
from research.local_split_circuit_20261116_v100.kernel import cnf
START=time.monotonic();out={'variants':{},'original_complete':False,'cost_improved':False,'selected_terms':None};error=None
try:
    d=json.loads((P/'DATA.json').read_text());cfg=json.loads(Path(d['config']).read_text());chart=json.loads(Path(d['cover']).read_text())['records'][1]['chart'];source,_=compile_all(cfg,chart);C,outputs,roots=source;b=json.dumps({'nodes':C.nodes,'outputs':outputs,'roots':roots},separators=(',',':')).encode();prior=json.loads(Path(d['source_receipt']).read_text());assert hashlib.sha256(b).hexdigest()==prior['out']['formulations']['candidate']['graph_sha256'];source_clauses=len(cnf(C,outputs));out['source']={'inputs':C.n,'variables':len(C.nodes)-2,'clauses':source_clauses,'graph_sha256':hashlib.sha256(b).hexdigest()};rng=random.Random(d['seed']);samples=[rng.randrange(1<<C.n)for _ in range(32)];old_values=[]
    for x in samples:
        values=C.evaluate(x);old_values.append([values[w]for row in outputs for field in row['wires']for w in field]+[values[w]for row in roots for w in row])
    for terms in d['term_limits']:
        t=time.monotonic()
        try:
            G,oo,rr,stats=rewrite(C,outputs,roots,terms,d['support_work_cap'])
            for x,wanted in zip(samples,old_values):
                values=G.evaluate(x);assert [values[w]for row in oo for field in row['wires']for w in field]+[values[w]for row in rr for w in row]==wanted
            raw=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode()
            with(P/('graph_terms'+str(terms)+'.json')).open('xb')as f:f.write(raw)
            out['variants'][str(terms)]={'status':'complete','inputs':G.n,'variables':len(G.nodes)-2,'clauses':len(cnf(G,oo)),'gate_kinds':dict(Counter(n[0]for n in G.nodes)),'graph_sha256':hashlib.sha256(raw).hexdigest(),'rewrite_and_check_seconds':time.monotonic()-t,'stats':stats,'actual_controls':len(samples)}
        except (OverflowError,MemoryError,AssertionError)as e:
            if isinstance(e,AssertionError)and str(e)!='gate cap':raise
            out['variants'][str(terms)]={'status':'capped','error':repr(e),'seconds':time.monotonic()-t}
        gc.collect()
    eligible=[(v['clauses'],v['variables'],int(k))for k,v in out['variants'].items()if v['status']=='complete'and v['clauses']<source_clauses and v['variables']<=len(C.nodes)-2]
    if eligible:out['selected_terms']=min(eligible)[2];out['cost_improved']=True
    out['total_seconds']=time.monotonic()-START;out['scope']='trainingrepresentationcalibrationonly;selectionbypreregisteredclauses/vars/termsrule;notqualifiedperformanceholdout'
except Exception as e:error=repr(e)
finish(out,error,START,'Boundedterm-limit calibration ontheexactfrozenverifier,allconstraint/rootvaluespreserved.','Onlyregisterchangedhardsearchforapassingcostimprovingvariant;otherwiserefuteflatteninganddesignsharing-awarelinearstageoptimization.')
