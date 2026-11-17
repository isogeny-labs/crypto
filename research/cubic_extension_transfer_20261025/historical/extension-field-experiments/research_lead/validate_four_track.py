"""Independent finite-field checks of saved graph relations and recovered logs."""
import json
from pathlib import Path
from time import perf_counter
from sage.all import GF,PolynomialRing,matrix,vector,sage_eval
from target_index_calculus import jacobian_context
from line_index_calculus import place_coordinates,split_line
from quotient_sweep import parse_curve
ROOT=Path(__file__).resolve().parent


def graph_check(row):
    p=row['p'];r=row['r'];result=row['target_solver'];field=GF(r);k=GF(p)
    R=PolynomialRing(k,['X','Y','Z']);X,Y,Z=R.gens()
    g=R(sage_eval(row['quartic'],locals={'X':X,'Y':Y,'Z':Z}))
    F,G,B,base,places=jacobian_context(g);coords=place_coordinates(F,places);lookup={P:i for i,P in enumerate(coords)}
    expressions={int(i):vector(field,v) for i,v in result['graph_expressions'].items()}
    width=result['factor_base_including_reference'];canonical=vector(field,width);canonical[-1]=1
    assert not expressions[0]
    for j,idx in enumerate(result['chosen_points'][1:]):
        unit=vector(field,width);unit[j]=1;assert expressions[idx]==unit
    raw=[]
    for record in result['line_certificates']:
        cert=record['certificate'];P=tuple(k(c) for c in cert['parameter_P']);Q=tuple(k(c) for c in cert['parameter_Q'])
        split,exact=split_line(g,P,Q);counts={lookup[P]:e for P,e in split.items()}
        assert counts=={int(i):int(e) for i,e in record['counts'].items()}
        assert exact['restriction_coefficients']==cert['restriction_coefficients']
        assert exact['infinity_multiplicity']==cert['infinity_multiplicity'];raw.append(counts)
    for idx,parent in result['graph_parent_edges'].items():
        counts=raw[parent['edge']]
        assert int(idx) in counts and all(i in expressions for i in counts)
        assert sum((e*expressions[i] for i,e in counts.items()),vector(field,width))==canonical
    rows=[];rhs=[]
    for rec in result['independent_relations']:
        counts={int(i):int(e) for i,e in rec['counts'].items()}
        value=sum((e*expressions[i] for i,e in counts.items()),vector(field,width))
        if rec['kind']=='recombined_line':
            value-=canonical;last=0;right=0
        else:last=-rec['b'];right=rec['a']
        assert list(map(int,value))==rec['coefficients']
        rows.append(list(value)+[last]);rhs.append(right)
    M=matrix(field,rows);assert M.rank()==width+1==result['linear_system_rank']
    answer=M.solve_right(vector(field,rhs));assert int(answer[-1])==result['scalar']==row['scalar']
    return {'p':p,'r':r,'raw_line_certificates':len(raw),'parent_substitutions':len(result['graph_parent_edges']),
            'matrix_rank':int(M.rank()),'scalar':row['scalar'],'passed':True}


def source_check(row):
    K=GF(row['p']**3,name='z');z=K.gen();E=parse_curve(row['source_curve'],K)
    assert str(K.modulus())==row['source_field']['modulus']
    P=E([sage_eval(row['source_generator'][c],locals={'z':z}) for c in ['x','y']])
    Q=E([sage_eval(row['source_target'][c],locals={'z':z}) for c in ['x','y']])
    assert row['scalar']*P==Q
    assert all(run['scalar']==row['scalar'] for run in row['source_pollard_rho']['runs'])
    assert row['source_bsgs']['scalar']==row['scalar']
    assert row['target_group_enumeration_used'] is False and row['source_log_used_in_transfer'] is False


def main():
    start=perf_counter();checks=[]
    for filename in ['four_track_benchmarks.json','four_track_large-prime_benchmarks.json',
                     'repaired_pipeline.json','forced_source_pipeline.json']:
        path=ROOT/'results'/filename
        if not path.exists():continue
        d=json.loads(path.read_text())
        for row in d.get('experiments',[d]):
            source_check(row);record={'file':filename,'p':row['p'],'source_answer_verified':True}
            if 'graph_expressions' in row['target_solver']:record['graph']=graph_check(row)
            checks.append(record)
    forced=ROOT/'results/forced_two_step_audit.json'
    if forced.exists():
        d=json.loads(forced.read_text());p=d['p'];r=d['r'];K=GF(p**3,name='z')
        assert (p**3+1-r)%p and p%3==1 and r%9==4
        for source in d['forced_sources']:
            E=parse_curve(source['curve'],K);assert E.j_invariant()**p!=E.j_invariant()
            assert len(E.division_polynomial(3).roots(multiplicities=False))==1
            assert len(E.isogenies_prime_degree(3))==1
        checks.append({'file':forced.name,'p':p,'non_subfield_forced_sources':len(d['forced_sources']),
                       'target_order_values_checked':len(d['all_12_independent_quotient_certificates']['kernels'])})
    out={'status':'passed','checks':checks,'seconds':perf_counter()-start,
         'scope':'Saved source logs, exact line intersections, tree substitutions and modular linear systems; runtime target-Jacobian equality was checked by each solver.'}
    (ROOT/'results/four_track_validation.json').write_text(json.dumps(out,indent=2)+'\n')
    print('FOUR TRACK VALIDATION COMPLETE',len(checks),out['seconds'],flush=True)


if __name__=='__main__':main()
