from pathlib import Path
import re,json,hashlib,shutil,datetime,subprocess
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';snap=Path('/workspace/root-unit-before-344');assert snap.is_dir() # Resume after sparse-only edit; retain initial before bytes
files=['src/cryptanalysis/koblitz_sparse_la.rs','src/cryptanalysis/ic_boundary.rs','src/cryptanalysis/pq_f4_f2.rs','src/cryptanalysis/prime_root_trace.rs','src/cryptanalysis/ic_framework/pkm_semaev.rs','src/cryptanalysis/ic_framework/prime_tower.rs','src/cryptanalysis/ic_framework/semaev_projection_oracle.rs']
assert (snap/files[0]).is_file()
def edit(n,f):
 p=repo/n;a=p.read_text();b=f(a);assert a!=b,n;p.write_text(b)
def once(a,x,y):assert a.count(x)==1,(x,a.count(x));return a.replace(x,y,1)
def block(a,needle):
 start=a.index(needle);o=a.index('{',start);depth=1;i=o+1
 while depth:
  if a[i]=='{':depth+=1
  elif a[i]=='}':depth-=1
  i+=1
 return start,i,a[start:i]
trusted=Path('/workspace/sparse-f66-334.rs').read_text()
def sparse(a):
 # Recover only established accounting, retaining every current kernel/layout.
 _,_,field=block(trusted,'pub struct FilterReport {');_,_,old=block(a,'pub struct FilterReport {')
 # Exact previous fields must match after removing the known accounting addition.
 extra='''    /// Multiply-subtracts on a row entry (right-hand side included) the
    /// merges performed: one per entry of the pivot row per combination.
    #[serde(default, skip_serializing_if = "is_default")]
    pub row_ops: u64,
'''
 assert field.replace(extra,'')==old;a=once(a,old,field)
 a=once(a,'let reduced = row.axpy(factor, &pivot, m);','let reduced = row.axpy(factor, &pivot, m);\n            self.report.row_ops += pivot.weight() as u64 + 1;')
 a=once(a,'let reduced = originals[k].axpy(factor, p, m);','let reduced = originals[k].axpy(factor, p, m);\n            self.report.row_ops += p.weight() as u64 + 1;')
 # Check that trusted counted reconstruction math equals current, apart from instrumentation.
 _,_,current=block(a,'pub fn reconstruct(')
 _,_,wrapper=block(trusted,'pub fn reconstruct(')
 lo,hi,counted=block(trusted,'fn reconstruct_counted(')
 stripped=counted.replace('fn reconstruct_counted(','pub fn reconstruct(').replace('        ops: &mut u64,\n','').replace('let mut solve_row =','let solve_row =').replace('            *ops += row.weight() as u64;\n','').replace('            *ops += (rows.len() * unknown.len() * unknown.len()) as u64;\n','')
 assert stripped==current,'reconstruction math divergence'
 comment='''

    /// [`Self::reconstruct`], adding to `ops` one per row entry a
    /// back-substitution reads and, for the residual dense solve, the
    /// Gauss–Jordan bound `rows × unknowns²`.
    '''
 a=once(a,current,wrapper+comment+counted)
 a=once(a,'report.filter = filtered.report.clone();','report.filter = filtered.report.clone();\n    report.row_ops = filtered.report.row_ops;')
 _,_,old=block(a,'fn solve_filtered_core<W: ModWord>(');_,_,new=block(trusted,'fn solve_filtered_core<W: ModWord>(')
 removed=new.replace('''            report.row_ops += core[core_cols.min(core.len())..]
                .iter()
                .map(|r| (r.weight() as u64 + 1) * opts.fold.max(1) as u64)
                .sum::<u64>();
            let square_nonzeros: u64 = square.iter().map(|r| r.weight() as u64).sum();
''','').replace('''                    report.row_ops +=
                        (wr.products * wr.block_n) as u64 * (square_nonzeros + square.len() as u64);
''','').replace('''                    report.row_ops += (lr.sparse_products * lr.block_size) as u64 * square_nonzeros;
''','').replace('''            report.row_ops += report.core_nonzeros as u64;
''','').replace('filtered.reconstruct_counted(&core_solution, rows, &mut report.row_ops)','filtered.reconstruct(&core_solution, rows)').replace('''    report.row_ops += rows.iter().map(|r| r.weight() as u64).sum::<u64>();
''','')
 assert removed==old,'core algorithm divergence'
 return once(a,old,new)
assert (repo/files[0]).read_text()==sparse((snap/files[0]).read_text())
def views(a):
 x='''    #[inline(always)]
    fn home(&self, key: u64) -> usize {'''
 y='''    // Complete occupied-entry views used by the original collision-ledger tests.
    #[cfg(test)]
    fn iter(&self) -> impl Iterator<Item = (u64, (u128, u64, u64))> + '_ {
        self.keys.iter().enumerate().filter_map(move |(i, &key)| {
            (key != 0).then(|| (key, (self.us[i], self.zs[i], self.bs[i])))
        })
    }

    #[cfg(test)]
    fn values(&self) -> impl Iterator<Item = (u128, u64, u64)> + '_ {
        self.iter().map(|(_, value)| value)
    }

'''+x
 lo,hi,body=block(a,"impl OpenCoefMap {");return a[:lo]+once(body,x,y)+a[hi:]
edit(files[1],views)
def f4(a):
 a=once(a,'groebner_basis_f4_inner(eqs, n, None, None, max_words, guard, true)','groebner_basis_f4_inner(eqs, n, None, None, max_words, guard, None, TraceMode::Off, true)')
 pattern=r'(TraceMode::Verify\(&eqs\),\n)(\s*)(\);)';a,n=re.subn(pattern,lambda m:m[1]+m[2]+'    fl_enabled(),\n'+m[2]+m[3],a);assert n==3,n;return a
edit(files[2],f4)
def trace(a):
 pat=r'let mut engine = Engine \{';hits=list(re.finditer(pat,a));edits=[]
 for h in hits:
  lo,hi,chunk=block(a[h.start():],h[0]);lo+=h.start();hi+=h.start()
  if 'reciprocal32:' in chunk:continue
  pos=a.index('{',lo)+1
  edits.append((pos,' reciprocal32: if p <= u32::MAX as u64 { ((1u128 << 64) / p as u128) as u64 } else { 0 },'))
 assert len(edits)==4,len(edits)
 for pos,x in reversed(edits):a=a[:pos]+x+a[pos:]
 return a
edit(files[3],trace)
def tower(a):
 a,n=re.subn(r'isogeny_tower\((inst\.curve\.p|curve\.p), t, 0, &mut GroupOps::default\(\)\)',r'isogeny_tower(\1, t, 0, AuxFamily::Any, &mut GroupOps::default())',a);assert n==4,n;return a
edit(files[4],tower)
edit(files[5],lambda a:once(a,'isogeny_tower(p, t, 2, &mut tower_ops)','isogeny_tower(p, t, 2, AuxFamily::Any, &mut tower_ops)'))
edit(files[6],lambda a:once(a,'''        let target = inst
            .curve
            .add(&mut ops, inst.curve.add(&mut ops, p0, p1), p2);''','''        let first_pair = inst.curve.add(&mut ops, p0, p1);
        let target = inst.curve.add(&mut ops, first_pair, p2);'''))
record={'schema':'actual-root-unit-api-accounting-recovery/v1','parent':'95f6003d167c91f6e40e82980f00a67b5adf75b2','trusted_sparse_ref':'f66a02c945','trusted_sparse_git_blob':'52facfde637baa1bc299fac20481c64a3796293c','reconstruction_and_core_math_byte_equal_after_removing_only_accounting':True,'all_current_optimized_kernels_preserved':True,'cost_counter_scope':'Established partial row-work unit including documented residual Gauss-Jordan bound; not all primitive operations/wallclock. Failed Krylov attempts without returned reports remain outside this legacy counter. Not performance evidence.','test_changes':'Real reciprocal fields; explicit legacy Any/FL/trace defaults; complete occupied-entry views cfgtest; split nested mutable borrow. No assertion removed or changed.','files':{n:{'before':hashlib.sha256((snap/n).read_bytes()).hexdigest(),'after':hashlib.sha256((repo/n).read_bytes()).hexdigest()} for n in files},'signed_execution_verdict':False,'performance_comparison':False}
for n in files:
 before=(snap/n).read_text();after=(repo/n).read_text()
 assert re.findall(r'assert(?:_eq|_ne)?!\s*\(',before)==re.findall(r'assert(?:_eq|_ne)?!\s*\(',after),n
record['assertion_macro_rosters_preserved']=True
(s/'ROOT_UNIT_RECOVERY_344.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record))
