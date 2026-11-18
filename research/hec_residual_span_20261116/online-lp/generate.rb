root='research/hec_residual_span_20261116/online-lp'
s=File.read('research/hec_residual_span_20261116/gain-graph/main.rs').gsub('research/hec_residual_span_20261116/gain-graph/',"#{root}/")
s=s.sub('mod graph;',"#[path=\"../gain-graph/graph.rs\"]\nmod graph;\nmod online;\nuse hec_native_ic::counted;")
s=s.sub('    let mut graph_checks = 0;',<<~RS)
    let mut arithmetic=fs::File::create_new("#{root}/ARITH.tsv").unwrap();
    writeln!(arithmetic,"g\\tp\\tsm\\tcase\\tcut\\tanchor\\tt\\tpartials\\tlp_rank\\tlabel_rank\\tonline_cells\\tinput_cells\\tmax_lp_width\\tonline_mul\\tonline_inv\\tonline_redc\\tref_mul\\tref_inv\\tref_redc\\tk").unwrap();
    let mut graph_checks = 0;
RS
needle='                let mut rejected = vec![0usize; 2];'
raise 'rejected marker' unless s.include?(needle)
s=s.sub(needle,needle+<<~RS)
                let mut incremental:Vec<_>=cuts.iter().map(|&cut|online::Online::new(cut,x.reps.len()-cut,x.ell)).collect();let mut own_stats=vec![[0u64;16];2];let mut ref_stats=vec![[0u64;16];2];
RS
needle='                            let mut ps = SolveStats::default();'
raise 'stats marker' unless s.include?(needle)
s=s.sub(needle,<<~RS+needle)
                            counted::reset();incremental[ci].insert(v,f.neg(a%x.ell));let own=counted::snapshot().get(&x.ell).copied().unwrap_or([0;16]);for j in 0..16{own_stats[ci][j]+=own[j]}assert_eq!(incremental[ci].lp_rank,exact);assert_eq!(incremental[ci].label_rank,labelrank);
                            counted::reset();
RS
needle='                            if let Some(k) = k {'
raise 'k marker' unless s.include?(needle)
s=s.sub(needle,<<~RS+needle)
                            let measured=counted::snapshot().get(&x.ell).copied().unwrap_or([0;16]);for j in 0..16{ref_stats[ci][j]+=measured[j]}assert_eq!(incremental[ci].k(),k);let own=own_stats[ci];let reference=ref_stats[ci];assert_eq!(own[8],2*own[7]+own[5]+own[6]);assert_eq!(reference[8],2*reference[7]+reference[5]+reference[6]);let input_cells:usize=partials[ci].iter().map(|(v,_)|v.len()+1).sum();writeln!(arithmetic,"{g}\\t{p}\\t{sm}\\t{case}\\t{cut}\\t{a}\\t{t}\\t{}\\t{}\\t{}\\t{}\\t{input_cells}\\t{}\\t{}\\t{}\\t{}\\t{}\\t{}\\t{}\\t{}",partials[ci].len(),exact,labelrank,incremental[ci].cells(),incremental[ci].max_lp_width,own[4],own[9],own[8],reference[4],reference[9],reference[8],k.map(|v|v.to_string()).unwrap_or("none".into())).unwrap();
RS
File.write("#{root}/main.rs",s)
