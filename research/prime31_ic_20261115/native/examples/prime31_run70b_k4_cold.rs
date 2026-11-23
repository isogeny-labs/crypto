// Run70: reuse every original Rust Prime31 group/rank/control function from
// the source-specific run69 module. This inclusion keeps the EXACT same
// Arithmetic, Base, group-law free identities, signed-DSU and quotient RREF.
// The pair table is GENERIC and is prohibited in a non-generic extractor.
#[allow(dead_code)]
mod control {
    include!("prime31_run69_genuine_partial_rank.rs");

    fn target_certificates(
        ar:&mut Arithmetic, q:Point, sp:&[Point],
        tab:&HashMap<Point,(u16,u16)>
    )->Result<(Vec<Witness>,u64),String> {
        let mut r=q;
        let mut tried=0u64;
        let mut found=Vec::<Witness>::new();
        for shift in 0..1024u64 {
            for (t,&pt) in sp.iter().enumerate() {
                tried+=1;
                let minus=ar.neg(PRIME31,pt);
                let complement=ar.add(PRIME31,r,minus);
                if let Some(&(i,j))=tab.get(&complement) {
                    let sum=ar.add(PRIME31,sp[i as usize],sp[j as usize]);
                    if ar.add(PRIME31,pt,sum)!=r {
                        return Err("Run70 invalid original-Rust witness".into());
                    }
                    let mut terms=[sgn_index(t),sgn_index(i as usize),sgn_index(j as usize)];
                    terms.sort_unstable();
                    if !found.iter().any(|w|w.shift==shift && w.terms==terms){
                        found.push(Witness {terms,shift});
                    }
                    if found.len()==16{return Ok((found,tried))}
                }
            }
            r=ar.add(PRIME31,r,Some(PRIME31.g));
        }
        Err(format!("Run70 censored: only {} distinct certificates",found.len()))
    }

    // Every width is a prefix of the same 16 ordered public certificates.
    // A target is complete iff at least one of its prefix's affine forms
    // is uniquely determined in the *actual labelled* relation row space.
    fn collect(
        ar:&mut Arithmetic,pts:&[Point],tab:&HashMap<Point,(u16,u16)>,
        free:&[Vec<i64>], witnesses:&[Witness],qs:&[Point]
    )->Result<(Vec<Vec<u64>>,[u64;3],[u64;3],[usize;3],f64),String>{
        if witnesses.len()!=qs.len()*16{return Err("Run70 wrong width".into())}
        let mut ds=AffineDsu::new(pts.len());
        let start=Instant::now();
        let mut r=None;
        let g=Some(PRIME31.g);
        let mut hits=0u64;
        let mut independent=0usize;
        let mut shifts=0u64;
        let widths=[1usize,4,16];
        let mut stops=[0u64;3];
        let mut hit_at=[0u64;3];
        let mut rank_at=[0usize;3];
        let mut saved=vec![Vec::<u64>::new();3];
        for k in 1..=MAX_SHIFTS {
            r=ar.add(PRIME31,r,g);
            shifts=k;
            if let Some(&(i,j))=tab.get(&r) {
                let (a,sa)=sgn_index(i as usize);
                let (b,sb)=sgn_index(j as usize);
                if ds.equation(a,sa,b,sb,k%N)?{independent+=1}
                hits+=1;
                if hits%32!=0{continue}
                let all=target_values(&mut ds,free,witnesses)?;
                for (arm,&width) in widths.iter().enumerate(){
                    if stops[arm]!=0{continue}
                    let mut recovered=Vec::with_capacity(qs.len());
                    for i in 0..qs.len(){
                        match (0..width).find_map(|j|all[i*16+j]){
                            Some(log)=>recovered.push(log),
                            None=>break,
                        }
                    }
                    if recovered.len()!=qs.len(){continue}
                    for (i,(&log,&q)) in recovered.iter().zip(qs).enumerate(){
                        if ar.scalar(PRIME31,log,g)!=q{
                            return Err(format!("Run70 K={width} invalid log for Q[{i}]"))
                        }
                    }
                    stops[arm]=shifts;
                    hit_at[arm]=hits;
                    rank_at[arm]=independent;
                    saved[arm]=recovered;
                    println!("RUN70_ARM k={width} shifts={} hits={} independent={} verified={}",
                        stops[arm],hit_at[arm],rank_at[arm],saved[arm].len());
                }
                if stops.iter().all(|&v|v!=0){break}
            }
        }
        if stops.iter().any(|&v|v==0){
            return Err(format!("Run70 censored: last={shifts} stops={stops:?}"));
        }
        // Source-bound run69 must remain identical for the first certificate.
        if stops[0]!=5_378_231||hit_at[0]!=2_272||rank_at[0]!=651{
            return Err(format!("Run70 first-certificate run69 replay FAILED: stops={stops:?} hits={hit_at:?} rank={rank_at:?}"))
        }
        if !(stops[2]<=stops[1]&&stops[1]<=stops[0]){
            return Err(format!("Run70 impossible monotonicity violation {stops:?}"))
        }
        let elapsed=start.elapsed().as_secs_f64()*1e3;
        println!("RUN70_RELATIONS k1={} k4={} k16={} last_shift={shifts} elapsed_ms={elapsed:.3} CLASS=GENERIC_REFERENCE",
            stops[0],stops[1],stops[2]);
        Ok((saved,stops,hit_at,rank_at,elapsed))
    }

    pub fn run70()->Result<(),String>{
        let mut ar=Arithmetic::default();
        let total=Instant::now();
        let pts=base(&mut ar)?;
        let free=free_rows(&mut ar,&pts)?;
        let (sp,tab)=pair_table(&mut ar,&pts)?;
        let setup_ms=total.elapsed().as_secs_f64()*1e3;
        let mut seed=0x68B677A1D1u64;
        let mut expected=Vec::new();
        let mut qs=Vec::new();
        for _ in 0..32 {
            seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;
            let k=(seed%(N-1))+1;
            expected.push(k); // secret used ONLY after candidate has returned
            qs.push(ar.scalar(PRIME31,k,Some(PRIME31.g)));
        }
        let witness_start=Instant::now();
        let mut witnesses=Vec::with_capacity(32*16);
        let mut leaves=0u64;
        for &q in &qs{
            let (mut certs,probes)=target_certificates(&mut ar,q,&sp,&tab)?;
            if certs.len()!=16{return Err("Run70 incomplete witness panel".into())}
            leaves+=probes;
            witnesses.append(&mut certs);
        }
        let witness_ms=witness_start.elapsed().as_secs_f64()*1e3;
        let (arms,stops,hits,ranks,relation_ms)=collect(&mut ar,&pts,&tab,&free,&witnesses,&qs)?;
        for (j,logs) in arms.iter().enumerate(){
            if logs!=&expected {return Err(format!("Run70 K={} post-solve harness mismatch",[1,4,16][j]))}
        }
        let mut other=Arithmetic::default();
        let (baseline,bs_setup,bs_online)=bsgs(&mut other,&qs)?;
        if arms.iter().any(|v|v!=&baseline){return Err("Run70 separate BSGS mismatch".into())}
        println!("RUN70_COMPLETE verified_per_arm=32 k1_shifts={} k4_shifts={} k16_shifts={} k1_hits={} k4_hits={} k16_hits={} k1_rank={} k4_rank={} k16_rank={} witness_leaves={} generic_keys={} setup_ms={setup_ms:.3} witness_ms={witness_ms:.3} relation_ms={relation_ms:.3} total_ms={:.3} CLASS=GENERIC_NEGATIVE_CONTROL",
            stops[0],stops[1],stops[2],hits[0],hits[1],hits[2],ranks[0],ranks[1],ranks[2],leaves,tab.len(),total.elapsed().as_secs_f64()*1e3);
        println!("RUN70_BSGS setup_ms={bs_setup:.3} online_32_ms={bs_online:.3} original_arithmetic=TRUE");
        println!("RUN70 PASS original Rust; authentic RHS; 32/32 each of K=1/4/16; NOT a non-generic extractor");
        Ok(())
    }

    /// Separately measured K=4 cold control: do NOT enumerate sixteen
    /// witnesses and do NOT continue the genuine-RHS walk through K=1.
    fn target_certificates4(
        ar:&mut Arithmetic,q:Point,sp:&[Point],
        tab:&HashMap<Point,(u16,u16)>
    )->Result<(Vec<Witness>,u64),String>{
        let mut r=q;
        let mut tried=0u64;
        let mut found=Vec::<Witness>::with_capacity(4);
        for shift in 0..1024u64 {
            for (t,&pt) in sp.iter().enumerate(){
                tried+=1;
                let negative=ar.neg(PRIME31,pt);
                let complement=ar.add(PRIME31,r,negative);
                if let Some(&(i,j))=tab.get(&complement){
                    let sum=ar.add(PRIME31,sp[i as usize],sp[j as usize]);
                    if ar.add(PRIME31,pt,sum)!=r{
                        return Err("K4 candidate failed original Rust group proof".into())
                    }
                    let mut terms=[sgn_index(t),sgn_index(i as usize),sgn_index(j as usize)];
                    terms.sort_unstable();
                    if !found.iter().any(|w|w.shift==shift&&w.terms==terms){
                        found.push(Witness{terms,shift});
                    }
                    if found.len()==4{return Ok((found,tried))}
                }
            }
            r=ar.add(PRIME31,r,Some(PRIME31.g));
        }
        Err(format!("K4 censored after 1024 shifts: {} witnesses",found.len()))
    }

    pub fn run_cold4()->Result<(),String>{
        let mut ar=Arithmetic::default();
        let whole=Instant::now();
        let pts=base(&mut ar)?;
        let free=free_rows(&mut ar,&pts)?;
        let (sp,tab)=pair_table(&mut ar,&pts)?;
        let setup_ms=whole.elapsed().as_secs_f64()*1e3;
        let mut seed=0x68B677A1D1u64;
        let mut expected=Vec::new();
        let mut qs=Vec::new();
        for _ in 0..32 {
            seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;
            let scalar=(seed%(N-1))+1;
            expected.push(scalar); // check ONLY after solving
            qs.push(ar.scalar(PRIME31,scalar,Some(PRIME31.g)));
        }
        let witness_start=Instant::now();
        let mut witnesses=Vec::<Witness>::with_capacity(128);
        let mut leaf_probes=0u64;
        for &q in &qs {
            let (mut w,tried)=target_certificates4(&mut ar,q,&sp,&tab)?;
            if w.len()!=4{return Err("K4 panel truncated".into())}
            witnesses.append(&mut w);
            leaf_probes+=tried;
        }
        let witness_ms=witness_start.elapsed().as_secs_f64()*1e3;
        let collection_start=Instant::now();
        let mut ds=AffineDsu::new(pts.len());
        let mut r=None;
        let g=Some(PRIME31.g);
        let mut hits=0u64;
        let mut independent=0usize;
        let mut last=0u64;
        let mut logs=Vec::<u64>::new();
        for k in 1..=MAX_SHIFTS{
            r=ar.add(PRIME31,r,g);
            last=k;
            if let Some(&(i,j))=tab.get(&r) {
                let (a,sa)=sgn_index(i as usize);
                let (b,sb)=sgn_index(j as usize);
                if ds.equation(a,sa,b,sb,k%N)?{independent+=1}
                hits+=1;
                if hits%32!=0{continue}
                let candidates=target_values(&mut ds,&free,&witnesses)?;
                let mut found=Vec::with_capacity(qs.len());
                for t in 0..qs.len(){
                    match (0..4).find_map(|v|candidates[t*4+v]){
                        Some(log)=>found.push(log),
                        None=>break
                    }
                }
                if found.len()!=qs.len(){continue}
                for (t,(&x,&q)) in found.iter().zip(qs.iter()).enumerate(){
                    if ar.scalar(PRIME31,x,g)!=q{
                        return Err(format!("K4 original Rust target verification failed: {t}"))
                    }
                }
                logs=found;
                break;
            }
        }
        let relation_ms=collection_start.elapsed().as_secs_f64()*1e3;
        if logs.len()!=32{return Err(format!("K4 censored: {last} shifts, {hits} hits"))}
        if last!=2_528_933||hits!=1_088||independent!=546{
            return Err(format!("K4 preregistered exact replay failed: shifts={last}, hits={hits}, rank={independent}"));
        }
        if logs!=expected{return Err("K4 secret-only post-solve mismatch".into())}
        let mut bs=Arithmetic::default();
        let (ref_logs,bs_setup,bs_online)=bsgs(&mut bs,&qs)?;
        if ref_logs!=logs{return Err("K4 independent BSGS failed".into())}
        println!("RUN70B_COMPLETE verified=32 shifts={last} hits={hits} independent={independent} witness_leaf_probes={leaf_probes} generic_keys={} setup_ms={setup_ms:.3} witness_ms={witness_ms:.3} relation_ms={relation_ms:.3} total_ms={:.3} CLASS=GENERIC_NEGATIVE_CONTROL",
            tab.len(),whole.elapsed().as_secs_f64()*1e3);
        println!("RUN70B_BSGS setup_ms={bs_setup:.3} online_32_ms={bs_online:.3}");
        println!("RUN70B PASS 32/32 genuine RHS known-log scalar verification; generic dictionary disqualifies IC claim");
        Ok(())
    }
}
fn main()->Result<(),String>{control::run_cold4()}
