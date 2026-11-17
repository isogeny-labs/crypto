root='research/hec_residual_span_20261116/gain-graph'
s=File.read('research/hec_residual_span_20261116/pencil/main.rs').gsub('research/hec_residual_span_20261116/pencil/',"#{root}/")
s="mod graph;\n"+s
s=s.sub('    let mut failures = 0;',<<~RS)
    let mut failures = 0;
    let mut graph_receipt=fs::File::create_new("#{root}/GRAPH.tsv").unwrap();
    writeln!(graph_receipt,"g\\tp\\tsm\\tcase\\tsmall_orbits\\tanchor\\tt\\tpartials\\trejected\\tlp_rank\\tgraph_rank\\tnaive_rank\\tlabel_relations\\tlabel_rank\\tk").unwrap();
    let mut graph_checks=0;let mut false_cycle_prefixes=0;
RS
needle='        let f = Fp::new(x.ell);'
raise 'f marker' unless s.include?(needle)
s=s.sub(needle,needle+<<~RS)
        // Two parallel gain edges: a graph cycle but full-rank LP block.
        let negative=vec![vec![f.neg(1),1],vec![f.neg(x.lambda),1]];
        assert_ne!(x.lambda,1);assert_eq!(rank(negative.clone(),&f),2);assert_eq!(graph::ranks(&negative,2,&f),(2,1));
RS
needle='                let mut rows: Vec<(Row, u64)> = vec![];'
raise 'rows marker' unless s.include?(needle)
s=s.sub(needle,<<~RS+needle)
                let cuts=vec![(x.reps.len()+3)/4,(x.reps.len()+1)/2];
                let mut partials:Vec<Vec<(Row,u64)>>=vec![vec![];2];let mut lp_rows:Vec<Vec<Vec<u64>>>=vec![vec![];2];let mut label_rows:Vec<Vec<Vec<u64>>>=vec![vec![];2];let mut rejected=vec![0usize;2];
RS
needle='                        dense.push(coeff);'
raise 'dense marker' unless s.include?(needle)
s=s.sub(needle,needle+<<~RS)
                        for (ci,&cut)in cuts.iter().enumerate(){let v=dense.last().unwrap();let lp=v[cut..x.reps.len()].to_vec();if lp.iter().filter(|&&x|x!=0).count()>2{rejected[ci]+=1;continue}lp_rows[ci].push(lp);partials[ci].push(rows.last().unwrap().clone());label_rows[ci].push(v.clone());let exact=rank(lp_rows[ci].clone(),&f);let(gr,naive)=graph::ranks(&lp_rows[ci],x.reps.len()-cut,&f);assert_eq!(exact,gr);assert!(naive<=gr);if naive<gr{false_cycle_prefixes+=1}let surviving=partials[ci].len()-gr;let labelrank=rank(label_rows[ci].clone(),&f)-gr;assert!(labelrank<=surviving);let mut ps=SolveStats::default();let k=solve_for_last(&partials[ci],x.reps.len(),x.ell,Solver::Fast,&[],&mut ps);if let Some(k)=k{assert_eq!(c.mul(&source,k),target);assert_eq!(k,expected)}graph_checks+=1;writeln!(graph_receipt,"{g}\\t{p}\\t{sm}\\t{case}\\t{cut}\\t{a}\\t{t}\\t{}\\t{}\\t{exact}\\t{gr}\\t{naive}\\t{surviving}\\t{labelrank}\\t{}",partials[ci].len(),rejected[ci],k.map(|v|v.to_string()).unwrap_or("none".into())).unwrap();}
RS
s=s.sub('    if failures != 0 {',"    assert!(graph_checks>0);eprintln!(\"graph_prefix_checks={graph_checks} naive_false_cycle_prefixes={false_cycle_prefixes} negative_controls=3\");\n    if failures != 0 {")
File.write("#{root}/main.rs",s)
