root='research/hec_residual_span_20261116/anchor-rank'
s=File.read('research/hec_residual_span_20261116/pencil/main.rs').gsub('research/hec_residual_span_20261116/pencil/',"#{root}/")
s=s.sub('    let mut failures = 0;',"    let mut failures = 0;\n    let mut single_checks=0;\n    let mut positive_checks=0;")
needle='                let mut rows: Vec<(Row, u64)> = vec![];'
s=s.sub(needle,<<~RS+needle)
                // Normalize only when the PUBLIC source Mumford divisor splits.
                let mut source_u=source.u;let mut source_coeff=vec![0;x.reps.len()];let mut source_sum=c.zero();
                for xx in 0..p{while source_u.deg()>0&&source_u.eval(xx,&c.fp)==0{source_u=source_u.div_exact(&Poly::x_minus(xx,&c.fp),&c.fp);let yy=source.v.eval(xx,&c.fp);source_sum=c.add_place(&source_sum,xx,yy);let j=x.fb.iter().position(|v|v.0==xx).unwrap();let(r,w)=x.map[j];if r!=usize::MAX{let w=if yy==x.fb[j].1{w}else{assert_eq!(yy,c.fp.neg(x.fb[j].1));f.neg(w)};source_coeff[r]=f.add(source_coeff[r],w)}}}
                let normalization=if source_u.deg()==0{assert_eq!(source_sum,source);Some((source_coeff.iter().enumerate().filter_map(|(j,&v)|(v!=0).then_some((j as u32,v))).collect::<Row>(),1u64))}else{None};
RS
s=s.sub('                for a in 1..=16u64 {',"                for a in 1..=16u64 {\n                    let mut single:Vec<(Row,u64)>=vec![];let mut single_dense:Vec<Vec<u64>>=vec![];")
needle='                        dense.push(coeff);'
raise unless s.include?(needle)
s=s.sub(needle,needle+<<~RS)
                        single.push(rows.last().unwrap().clone());single_dense.push(dense.last().unwrap().clone());
                        assert!(!c.is_zero(&anchor));
                        let sr=rank(single_dense.clone(),&f);let dropped=single_dense.iter().map(|v|v[..x.reps.len()].to_vec()).collect();assert_eq!(sr,rank(dropped,&f));let augmented=single_dense.iter().map(|v|{let mut v=v.clone();v.push(f.neg(a%x.ell));v}).collect();assert_eq!(sr,rank(augmented,&f));
                        let mut one_stats=SolveStats::default();assert_eq!(solve_for_last(&single,x.reps.len(),x.ell,Solver::Fast,&[],&mut one_stats),None);single_checks+=1;
                        if let Some(norm)=&normalization{let mut normalized=single.clone();normalized.push(norm.clone());let mut ns=SolveStats::default();if let Some(k)=solve_for_last(&normalized,x.reps.len(),x.ell,Solver::Fast,&[],&mut ns){assert_eq!(c.mul(&source,k),target);assert_eq!(k,expected);positive_checks+=1}}
RS
s=s.sub('    if failures != 0 {',"    assert_eq!(single_checks,892);assert!(positive_checks>0);eprintln!(\"single_anchor_prefix_checks={single_checks} source_normalized_determining_prefixes={positive_checks}\");\n    if failures != 0 {")
File.write("#{root}/main.rs",s)
