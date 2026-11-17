// Exact affine-rank diagnostic; the v8 arithmetic prefix remains byte-preserved.
#[derive(Clone, Debug, Serialize)]
struct Fiber {
    rank: usize,
    pivots: Vec<usize>,
    free: Vec<usize>,
    particular: Vec<u64>,
    nullspace: Vec<Vec<u64>>,
}
fn fiber(rows: &[Vec<u64>], n: usize, p: u64) -> Fiber {
    let mut a = rows.to_vec();
    let mut pivots = vec![];
    let mut r = 0;
    for c in 0..n {
        let Some(k) = (r..a.len()).find(|k| a[*k][c] != 0) else {
            continue;
        };
        a.swap(k, r);
        let z = inv(a[r][c], p);
        for x in &mut a[r] {
            *x = *x * z % p;
        }
        for i in 0..a.len() {
            if i != r {
                let z = a[i][c];
                for j in 0..=n {
                    a[i][j] = (a[i][j] + p - z * a[r][j] % p) % p;
                }
            }
        }
        pivots.push(c);
        r += 1;
    }
    assert!(
        a.iter()
            .all(|v| v[..n].iter().any(|x| *x != 0) || v[n] == 0),
        "inconsistent equations"
    );
    let free: Vec<_> = (0..n).filter(|c| !pivots.contains(c)).collect();
    let mut particular = vec![0; n];
    for (i, c) in pivots.iter().enumerate() {
        particular[*c] = a[i][n];
    }
    let nullspace: Vec<Vec<u64>> = free
        .iter()
        .map(|c| {
            let mut v = vec![0; n];
            v[*c] = 1;
            for (i, j) in pivots.iter().enumerate() {
                v[*j] = (p - a[i][*c]) % p;
            }
            v
        })
        .collect();
    let f = Fiber {
        rank: r,
        pivots,
        free,
        particular,
        nullspace,
    };
    for row in rows {
        assert_eq!(dot_mod(&row[..n], &f.particular, p), row[n]);
        for v in &f.nullspace {
            assert_eq!(dot_mod(&row[..n], v, p), 0);
        }
    }
    f
}
fn dot_mod(a: &[u64], b: &[u64], p: u64) -> u64 {
    a.iter().zip(b).fold(0, |s, (x, y)| (s + x * y) % p)
}
fn target_fixed(f: &Fiber, k: usize) -> bool {
    f.nullspace.iter().all(|v| v[k] == 0)
}
fn parse_point(v: &Value) -> P {
    let a: Vec<u64> = serde_json::from_value(v.clone()).unwrap();
    if a == vec![0] {
        None
    } else {
        Some((Q(a[0], 0), Q(a[1], 0)))
    }
}
fn base_rule(
    e: Quad,
    t: Quad,
    g: P,
    kind: u8,
    wanted: usize,
    shift: u64,
    h: P,
    ord: u64,
) -> (Vec<P>, usize) {
    let mut xs = BTreeSet::new();
    let mut scanned = 0;
    if kind == 0 {
        for x in 0..e.p {
            scanned += 1;
            if x != g.unwrap().0 .0 && toy_field::sqrt(e.rhs(Q(x, 0)).0, e.p).is_some() {
                xs.insert(x);
            }
            if xs.len() + 1 == wanted {
                break;
            }
        }
    } else {
        let group = if kind >= 2 { t } else { e };
        for k in 1..ord {
            scanned = k as usize;
            let z = group.pm(h, k as u128).unwrap();
            let xx = if kind >= 2 {
                z.0 .0 * inv(e.d, e.p) % e.p
            } else {
                z.0 .0
            };
            let x = (xx + shift) % e.p;
            if x != g.unwrap().0 .0 && toy_field::sqrt(e.rhs(Q(x, 0)).0, e.p).is_some() {
                xs.insert(x);
            }
            if xs.len() + 1 == wanted {
                break;
            }
        }
    }
    let mut b = vec![g];
    b.extend(xs.into_iter().map(|x| canon(e, x)));
    (b, scanned)
}
fn catalogs(e: Quad, base: &[P], targets: &[P], m: usize) -> Vec<Vec<Witness>> {
    let mut out = vec![vec![]; targets.len()];
    for i in 0..base.len() {
        for j in i..base.len() {
            for k in 0..if m == 3 { base.len() } else { 1 } {
                for mask in 0..1 << m {
                    let w = Witness {
                        indices: if m == 3 { vec![i, j, k] } else { vec![i, j] },
                        mask,
                    };
                    let s = witness_sum(e, base, &w);
                    if let Some(q) = targets.iter().position(|x| *x == s) {
                        out[q].push(w);
                    }
                }
            }
        }
    }
    out
}
fn unique_rows(rows: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    rows.into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
fn anchor_row(n: usize) -> Vec<u64> {
    let mut v = vec![0; n + 1];
    v[0] = 1;
    v[n] = 1;
    v
}
fn gp_matrix_rows(rows: &[Vec<u64>]) -> String {
    format!("Mat({}~)*Mod(1,823)", serde_json::to_string(rows).unwrap())
}
fn gp_column(v: &[u64]) -> String {
    format!("Mod(1,823)*{}~", serde_json::to_string(v).unwrap())
}
fn gp_helpers() -> &'static str {
    concat!(
"ck(c,s)=if(!c,error(s));\n",
"canonpt(xx)={my(z=Mod(xx^3,p)+E.a4*xx+E.a6,yy);if(!issquare(z),return([]));yy=lift(sqrt(z));[Mod(xx,p),Mod(min(yy,p-yy),p)]};\n",
"makebase(kind,nb,sh,hh,ord)={my(xs=Set([]),sc=0,z,xx,bb);if(kind==0,for(j=0,p-1,sc++;if(j!=lift(G[1])&&canonpt(j)!=[],xs=setunion(xs,[j]));if(#xs+1==nb,break)),for(j=1,ord-1,sc=j;z=ellmul(if(kind>=2,T,E),hh,j);xx=lift(z[1]/if(kind>=2,2,1)+sh);if(xx!=lift(G[1])&&canonpt(xx)!=[],xs=setunion(xs,[xx]));if(#xs+1==nb,break)));bb=concat([G],vector(#xs,j,canonpt(xs[j])));[bb,sc]};\n",
"summ(bb,ii,mask)={my(acc=[0],z);for(j=1,#ii,z=bb[ii[j]+1];if(bittest(mask,j-1),z=ellneg(E,z));acc=elladd(E,acc,z));acc};\n",
"collectcat(bb,tt,arity)={my(out=vector(#tt,j,List()),ii,z);for(i=0,#bb-1,for(j=i,#bb-1,for(k=0,if(arity==3,#bb-1,0),ii=if(arity==3,[i,j,k],[i,j]);for(mask=0,2^arity-1,z=summ(bb,ii,mask);for(oi=1,#tt,if(z==tt[oi],listput(out[oi],[ii,mask]);break))))));vector(#tt,j,Vec(out[j]))};\n",
"torow(w,off,nb)={my(v=vector(nb+2));for(j=1,#w[1],v[w[1][j]+1]+=if(bittest(w[2],j-1),-1,1));v[nb+1]=-1;v[nb+2]=off;lift(v*Mod(1,823))};\n",
"allrows(c2,c3,ed,nb,mode,on)={my(v=List(),ar=vector(nb+2),rr);ar[1]=1;ar[nb+2]=1;listput(v,ar);if(on,for(i=1,#ed,ar=vector(nb+2);ar[ed[i][2]+1]=1;ar[ed[i][1]+1]-=ed[i][3];listput(v,lift(ar*Mod(1,823)))));for(m=2,3,if(mode==m||mode==23,rr=if(m==2,c2,c3);for(j=0,15,for(k=1,#rr[j+1],listput(v,torow(rr[j+1][k],j,nb))))));Set(Vec(v))};\n"
)
}
fn controls(out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh development control output required".into());
    }
    fs::create_dir_all(out)?;
    let e = Quad::new(101, 2, 3);
    let t = Quad::new(101, 8, 24);
    let g = Some((Q(1, 0), Q(39, 0)));
    assert_eq!(order(e, g, 96), 96);
    let q = e.pm(g, 37);
    let targets: Vec<_> = (0..16).map(|j| e.pa(q, e.pm(g, j))).collect();
    let mut script=String::from("p=101;\nE=ellinit([Mod(2,p),Mod(3,p)]);T=ellinit([Mod(8,p),Mod(24,p)]);G=Mod(1,p)*[1,39];Q=ellmul(E,G,37);\n");
    script += gp_helpers();
    script += "tt=vector(16,j,elladd(E,Q,ellmul(E,G,j-1)));\n";
    for kind in [0, 1] {
        let (b, sc) = base_rule(e, t, g, kind, 4, 1, g, 96);
        script += &format!(
            "z=makebase({},4,1,G,96);bb=z[1];ck(lift(bb)=={}&&z[2]=={},\"control base\");\n",
            kind,
            serde_json::to_string(&b.iter().map(|p| pair(*p)).collect::<Vec<_>>())?,
            sc
        );
        for m in [2, 3] {
            let cat = catalogs(e, &b, &targets, m);
            for j in 0..16 {
                assert_eq!(cat[j], collect(e, &b, targets[j], m));
            }
            script += &format!(
                "ck(collectcat(bb,tt,{})=={},\"control complete catalogs\");\n",
                m,
                serde_json::to_string(
                    &cat.iter()
                        .map(|ws| ws
                            .iter()
                            .map(|w| json!([w.indices, w.mask]))
                            .collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                )?
            );
        }
    }
    let ambiguous = fiber(&[vec![1, 0, 0, 1], vec![0, 1, 1, 7]], 3, 101);
    assert!(!target_fixed(&ambiguous, 2));
    let fixed = fiber(&[vec![0, 0, 1, 7]], 3, 101);
    assert!(target_fixed(&fixed, 2));
    assert_eq!(fixed.rank, 1);
    script+="ma=Mod(1,101)*[1,0,0;0,1,1];ka=matker(ma);ck(sum(j=1,matsize(ka)[2],ka[3,j]!=0)>0,\"ambiguous control\");mf=Mod(1,101)*Mat([0,0,1]);kf=matker(mf);ck(matrank(mf)==1&&sum(j=1,matsize(kf)[2],kf[3,j]!=0)==0,\"rank deficient target fixed\");ck(ma*Mat([Mod(1,101)*[0,-1,1]~])==matrix(2,1,i,j,Mod(0,101)),\"rectangular kernel control\");print(\"RANK_REPAIR_DEVELOPMENT_CONTROLS_OK\");\n";
    fs::write(out.join("verify.gp"), script)?;
    Ok(())
}
fn wvalues(ws: &[Witness]) -> Value {
    json!(ws
        .iter()
        .map(|w| json!([w.indices, w.mask]))
        .collect::<Vec<_>>())
}
fn produce_rank_panel(out: &Path, input8: &Path, input10: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh rank-repair output required".into());
    }
    fs::create_dir_all(out)?;
    let v8: Value = serde_json::from_slice(&fs::read(input8)?)?;
    let va: Value = serde_json::from_slice(&fs::read(input10)?)?;
    assert_eq!(
        va["status"],
        "native-generic-and-independent-PARI-endomorphism-verified"
    );
    let e = Quad::new(827, 1, 15);
    let t = Quad::new(827, 4, 120);
    let g = Some((Q(2, 0), Q(5, 0)));
    let q = e.pm(g, 37);
    assert!(toy_field::prime(823));
    assert_eq!(order(e, g, 823), 823);
    let first = parse_point(&v8["torsion_basis"][0]);
    let maximal = parse_point(&v8["twist_maximal_generator"]);
    assert_eq!(pair(first), vec![0, 51]);
    assert_eq!(pair(maximal), vec![2, 386]);
    assert_eq!(order(t, first, 833), 7);
    assert_eq!(order(t, maximal, 833), 119);
    let vm: Velu = serde_json::from_value(va["velu"].clone())?;
    assert!(vm.certify(Ring(827), 1, 15));
    let image = |z| scale_point(e, vm.eval(e, z), 7);
    let targets: Vec<_> = (0..16).map(|j| e.pa(q, e.pm(g, j))).collect();
    assert!(targets.iter().all(Option::is_some));
    let mut gp=String::from("default(parisizemax,4000000000);\np=827;\nE=ellinit([Mod(1,p),Mod(15,p)]);T=ellinit([Mod(4,p),Mod(120,p)]);G=Mod(1,p)*[2,5];Q=ellmul(E,G,37);\n");
    gp += gp_helpers();
    gp += &format!(
        "av=ellisogeny(E,{});ah=av[2][3];amap=[49*av[2][1]/ah^2,343*av[2][2]/y/ah^3];\n",
        ppoly(&vm.h)
    );
    gp+="aim(P)={my(xx);if(P==[0],return(P));xx=P[1];if(subst(ah,x,xx)==0,return([0]));[subst(amap[1],x,xx),P[2]*subst(amap[2],x,xx)]};\nck(ellcard(E)==823&&ellorder(E,G)==823&&ellcard(T)==833&&isprime(823),\"group checks\");tt=vector(16,j,elladd(E,Q,ellmul(E,G,j-1)));gpconfigs=0;gpqueries=0;gpsystems=0;gpunderfills=0;\n";
    let mut configs = vec![];
    let mut cells = vec![];
    let mut query_text = String::new();
    let mut matrix_text = String::new();
    let (
        mut filled,
        mut underfilled,
        mut queries,
        mut zero_queries,
        mut witnesses,
        mut identifiable,
        mut ambiguous,
        mut map_checks,
        mut historical,
        mut historical_queries,
    ) = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    let rules = [
        (0, "low-x", 0),
        (1, "curve-orbit", 1),
        (1, "curve-orbit", 3),
        (2, "twist-first", 1),
        (2, "twist-first", 3),
        (3, "twist-maximal", 1),
        (3, "twist-maximal", 3),
    ];
    for wanted in [8, 12, 16] {
        for (kind, name, shift) in rules {
            for augmented in [false, true] {
                let id = format!(
                    "b{wanted}-{name}-s{shift}-{}",
                    if augmented { "alpha-half" } else { "plain" }
                );
                let (h, ord) = match kind {
                    0 | 1 => (g, 823),
                    2 => (first, 7),
                    3 => (maximal, 119),
                    _ => unreachable!(),
                };
                let seed_wanted = if augmented { wanted / 2 } else { wanted };
                let (seed, scanned) = base_rule(e, t, g, kind, seed_wanted, shift, h, ord);
                let mut base = seed.clone();
                if augmented {
                    let mut xs: BTreeSet<_> = seed.iter().map(|p| p.unwrap().0 .0).collect();
                    for p in &seed {
                        xs.insert(image(*p).unwrap().0 .0);
                    }
                    xs.remove(&2);
                    base = vec![g];
                    base.extend(xs.into_iter().map(|x| canon(e, x)));
                }
                assert!(base.len() <= wanted);
                assert_eq!(base[0], g);
                let mut edges: Vec<Vec<u64>> = vec![];
                for (i, p) in base.iter().enumerate() {
                    let z = image(*p);
                    assert!(e.on(z));
                    assert_eq!(z, e.pm(*p, 353));
                    assert_ne!(z, e.pm(*p, 354));
                    map_checks += 1;
                    let c = canon(e, z.unwrap().0 .0);
                    let factor = if z == c { 353 } else { 823 - 353 };
                    if let Some(j) = base.iter().position(|v| *v == c) {
                        edges.push(vec![i as u64, j as u64, factor]);
                    }
                }
                let is_filled = base.len() == wanted;
                let legacy = if !augmented && wanted == 8 && kind == 0 {
                    Some(0)
                } else if !augmented && wanted == 8 && kind == 3 && shift == 1 {
                    Some(1)
                } else {
                    None
                };
                if let Some(bi) = legacy {
                    assert_eq!(
                        json!(base.iter().map(|p| pair(*p)).collect::<Vec<_>>()),
                        v8["source_bases"][bi]
                    );
                    historical += 1;
                }
                gp+=&format!("gpconfigs++;z=makebase({},{},{},Mod(1,p)*{},{});seedbb=z[1];ck(z[2]=={},\"scan count\");bb=seedbb;\n",kind,seed_wanted,shift,serde_json::to_string(&pair(h))?,ord,scanned);
                if augmented {
                    gp+="xs=Set(concat(vector(#seedbb,j,lift(seedbb[j][1])),vector(#seedbb,j,lift(aim(seedbb[j])[1]))));xs=setminus(xs,[lift(G[1])]);bb=concat([G],vector(#xs,j,canonpt(xs[j])));\n";
                }
                gp+=&format!("ck(lift(bb)=={},\"complete base\");ed=List();for(ii=1,#bb,zz=aim(bb[ii]);ck(ellisoncurve(E,zz)&&zz==ellmul(E,bb[ii],353)&&zz!=ellmul(E,bb[ii],354),\"map/scalar control\");zzc=canonpt(lift(zz[1]));fac=if(zz==zzc,353,470);for(jj=1,#bb,if(bb[jj]==zzc,listput(ed,[ii-1,jj-1,fac]);break)));ed=Vec(ed);ck(ed=={},\"all internal alpha edges\");\n",serde_json::to_string(&base.iter().map(|p|pair(*p)).collect::<Vec<_>>())?,serde_json::to_string(&edges)?);
                configs.push(json!({"id":id,"kind":name,"kind_code":kind,"shift":shift,"requested_B":wanted,"transform":if augmented{"alpha-half"}else{"plain"},"seed_requested":seed_wanted,"seed_size":seed.len(),"seed_scan_terms":scanned,"given_seed":pair(h),"given_seed_order":ord,"points":base.iter().map(|p|pair(*p)).collect::<Vec<_>>(),"actual_B":base.len(),"alpha_edges":edges,"status":if is_filled{"filled"}else{"underfilled"},"planned_queries":32,"attempted_queries":if is_filled{32}else{0},"legacy_base_match":legacy.is_some()}));
                if !is_filled {
                    underfilled += 1;
                    gp += &format!("ck(#bb!={},\"underfill\");gpunderfills++;\n", wanted);
                    for mode in [2, 3, 23] {
                        for links in [false, true] {
                            let cell = json!({"configuration":id,"mode":mode,"alpha_links":links,"status":"excluded_underfilled","actual_B":base.len(),"requested_B":wanted});
                            matrix_text += &(serde_json::to_string(&cell)? + "\n");
                            cells.push(cell);
                        }
                    }
                    continue;
                }
                filled += 1;
                gp += &format!("ck(#bb=={},\"filled base\");\n", wanted);
                let n = base.len() + 1;
                let cat2 = catalogs(e, &base, &targets, 2);
                let cat3 = catalogs(e, &base, &targets, 3);
                gp += "cc2=collectcat(bb,tt,2);cc3=collectcat(bb,tt,3);gpqueries+=32;\n";
                for (m, cat) in [(2, &cat2), (3, &cat3)] {
                    for j in 0..16 {
                        let ws = &cat[j];
                        queries += 1;
                        witnesses += ws.len();
                        if ws.is_empty() {
                            zero_queries += 1;
                        }
                        for w in ws {
                            assert_eq!(witness_sum(e, &base, w), targets[j]);
                        }
                        if let Some(bi) = legacy {
                            let old = v8["cases"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .find(|c| c["model"] == 0 && c["base"] == bi)
                                .unwrap();
                            let oldq = old["queries"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .find(|v| v["arity"] == m && v["offset"] == j)
                                .unwrap();
                            let oldws: Vec<Witness> =
                                serde_json::from_value(oldq["witnesses"].clone())?;
                            assert_eq!(*ws, oldws);
                            historical_queries += 1;
                        }
                        query_text += &(serde_json::to_string(
                            &json!({"configuration":id,"arity":m,"offset":j,"target":pair(targets[j]),"witnesses":ws}),
                        )? + "\n");
                        gp += &format!(
                            "ck({}[{}]=={},\"complete witness query\");\n",
                            if m == 2 { "cc2" } else { "cc3" },
                            j + 1,
                            serde_json::to_string(&wvalues(ws))?
                        );
                    }
                }
                for mode in [2, 3, 23] {
                    for links in [false, true] {
                        let mut known = vec![anchor_row(n)];
                        if links {
                            for edge in &edges {
                                let mut rr = vec![0; n + 1];
                                let i = edge[0] as usize;
                                let j = edge[1] as usize;
                                rr[j] = 1;
                                rr[i] = (rr[i] + 823 - edge[2]) % 823;
                                known.push(rr);
                            }
                        }
                        let known = unique_rows(known);
                        let kf = fiber(&known, n, 823);
                        assert!(!target_fixed(&kf, n - 1));
                        let mut qr = vec![];
                        let mut raw_count = 0;
                        for (m, cat) in [(2, &cat2), (3, &cat3)] {
                            if mode == m || mode == 23 {
                                for (j, ws) in cat.iter().enumerate() {
                                    for w in ws {
                                        qr.push(row(w, j as u64, base.len()));
                                        raw_count += 1;
                                    }
                                }
                            }
                        }
                        let qr = unique_rows(qr);
                        let coefficient_count = qr
                            .iter()
                            .map(|r| r[..n].to_vec())
                            .collect::<BTreeSet<_>>()
                            .len();
                        assert_eq!(coefficient_count, qr.len());
                        let mut rows = known.clone();
                        rows.extend(qr.clone());
                        let rows = unique_rows(rows);
                        let f = fiber(&rows, n, 823);
                        let fixed = target_fixed(&f, n - 1);
                        assert!(f.rank >= kf.rank);
                        let alternative = if fixed {
                            identifiable += 1;
                            assert_eq!(e.pm(g, f.particular[n - 1] as u128), q);
                            None
                        } else {
                            ambiguous += 1;
                            let dir = f.nullspace.iter().find(|v| v[n - 1] != 0).unwrap();
                            let alt: Vec<_> = f
                                .particular
                                .iter()
                                .zip(dir)
                                .map(|(a, b)| (a + b) % 823)
                                .collect();
                            assert_ne!(alt[n - 1], f.particular[n - 1]);
                            for r in &rows {
                                assert_eq!(dot_mod(&r[..n], &alt, 823), r[n]);
                            }
                            Some(alt)
                        };
                        gp+=&format!("gpsystems++;rr=allrows(cc2,cc3,ed,{}, {}, {});ck(rr=={},\"independent deduplicated rows\");mr={};coef=mr[,1..{}];rhs=mr[,{}];kk=matker(coef);ss=matinverseimage(coef,rhs);ck(matrank(coef)=={}&&matsize(kk)[2]=={}&&#ss=={}&&coef*ss==rhs,\"rank/kernel/fiber\");np={};ck(coef*np==rhs,\"native particular\");ck((sum(j=1,matsize(kk)[2],kk[{},j]!=0)==0)=={},\"target projection\");kr=allrows(cc2,cc3,ed,{},0,{});km=Mat(kr~)*Mod(1,823);ck(matrank(km[,1..{}])=={},\"known information rank\");\n",base.len(),mode,usize::from(links),serde_json::to_string(&rows)?,gp_matrix_rows(&rows),n,n+1,f.rank,f.free.len(),n,gp_column(&f.particular),n,usize::from(fixed),base.len(),usize::from(links),n,kf.rank);
                        if !f.nullspace.is_empty() {
                            let cols = f
                                .nullspace
                                .iter()
                                .map(|v| gp_column(v))
                                .collect::<Vec<_>>()
                                .join(",");
                            gp+=&format!("nk=Mat([{}]);ck(matrank(nk)=={}&&coef*nk==matrix(matsize(coef)[1],matsize(nk)[2],i,j,Mod(0,823)),\"complete native nullspace\");\n",cols,f.free.len());
                        }
                        if fixed {
                            gp+=&format!("ck(lift(ss[{}])=={}&&ellmul(E,G,lift(ss[{}]))==Q,\"target recovery\");\n",n,f.particular[n-1],n);
                        } else {
                            gp+=&format!("alt={};ck(coef*alt==rhs&&alt[{}]!=np[{}],\"two target values in same fiber\");\n",gp_column(alternative.as_ref().unwrap()),n,n);
                        }
                        let compact = json!({"configuration":id,"mode":mode,"alpha_links":links,"status":"analyzed","B":base.len(),"unknowns":n,"raw_witness_rows":raw_count,"distinct_query_equations":qr.len(),"known_rank":kf.rank,"total_rank":f.rank,"new_rank_from_queries":f.rank-kf.rank,"kernel_dimension":f.free.len(),"target_identifiable":fixed,"recovered_scalar":if fixed{json!(f.particular[n-1])}else{Value::Null}});
                        let mut record = compact.clone();
                        record["rows"] = json!(rows);
                        record["fiber"] = json!(f);
                        record["alternative_solution"] = json!(alternative);
                        matrix_text += &(serde_json::to_string(&record)? + "\n");
                        cells.push(compact);
                    }
                }
            }
        }
    }
    assert_eq!(configs.len(), 42);
    assert_eq!(cells.len(), 252);
    assert_eq!((historical, historical_queries), (2, 64));
    gp+=&format!("ck(gpconfigs==42&&gpunderfills=={}&&gpqueries=={}&&gpsystems=={},\"complete flow\");print(\"ALL_TWIST_RANK_REPAIR_OK 42\");\n",underfilled,queries,filled*6);
    let summary = json!({"configurations":42,"filled_configurations":filled,"underfilled_configurations":underfilled,"planned_queries":1344,"attempted_queries":queries,"zero_witness_queries":zero_queries,"raw_witnesses":witnesses,"planned_systems":252,"analyzed_systems":filled*6,"excluded_underfilled_systems":underfilled*6,"target_identifiable_systems":identifiable,"target_ambiguous_systems":ambiguous,"base_point_map_scalar_controls":map_checks,"legacy_bases_replayed":historical,"legacy_queries_replayed":historical_queries});
    fs::write(out.join("QUERIES.jsonl"), &query_text)?;
    fs::write(out.join("MATRICES.jsonl"), &matrix_text)?;
    fs::write(out.join("verify.gp"), &gp)?;
    let result = json!({"schema":"twist.rank-repair/v1","summary":summary,"configurations":configs,"cells":cells,"queries_sha256":registry::digest(query_text.as_bytes()),"matrices_sha256":registry::digest(matrix_text.as_bytes()),"gp_script_sha256":registry::digest(gp.as_bytes()),"no_factor_base_logs_or_fixture_scalar_enter_elimination":true,"performance_claim":false,"novelty_claim":false});
    fs::write(
        out.join("RESULTS.json"),
        serde_json::to_string_pretty(&result)? + "\n",
    )?;
    println!("{}", summary);
    Ok(())
}
fn admit_rank_panel(raw: &Path, pari: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh admission required".into());
    }
    let d: Value = serde_json::from_slice(&fs::read(raw.join("RESULTS.json"))?)?;
    let p: Value = serde_json::from_slice(&fs::read(pari.join("RECEIPT.json"))?)?;
    let hash = |p: &Path| -> Result<String> { Ok(registry::digest(&fs::read(p)?)) };
    assert_eq!(p["exit_code"], 0);
    assert!(p["signal"].is_null());
    assert_eq!(p["script_sha256"], hash(&raw.join("verify.gp"))?);
    assert_eq!(d["gp_script_sha256"], p["script_sha256"]);
    assert_eq!(d["queries_sha256"], hash(&raw.join("QUERIES.jsonl"))?);
    assert_eq!(d["matrices_sha256"], hash(&raw.join("MATRICES.jsonl"))?);
    let so = fs::read(pari.join("pari.stdout"))?;
    let se = fs::read(pari.join("pari.stderr"))?;
    assert_eq!(p["stdout_sha256"], registry::digest(&so));
    assert_eq!(p["stderr_sha256"], registry::digest(&se));
    assert_eq!(String::from_utf8(so)?.trim(), "ALL_TWIST_RANK_REPAIR_OK 42");
    let err = String::from_utf8(se)?;
    assert!(
        err.trim().is_empty()
            || err.trim()
                == "***   Warning: new maximum stack size = 4000002048 (3814.699 Mbytes)."
    );
    let n: Value = serde_json::from_slice(&fs::read(
        raw.parent().unwrap().join("producer-transport.json"),
    )?)?;
    assert_eq!(n["exit_code"], 0);
    assert!(n["signal"].is_null());
    let mut compact = d;
    compact["status"] = json!("native-and-independent-PARI-rank-panel-verified");
    compact["native_result_sha256"] = json!(hash(&raw.join("RESULTS.json"))?);
    compact["pari_receipt_sha256"] = json!(hash(&pari.join("RECEIPT.json"))?);
    fs::write(out, serde_json::to_string_pretty(&compact)? + "\n")?;
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() == 2 && a[0] == "controls" {
        return controls(Path::new(&a[1]));
    }
    if a.len() == 3 {
        return produce_rank_panel(Path::new(&a[0]), Path::new(&a[1]), Path::new(&a[2]));
    }
    if a.len() == 4 && a[0] == "admit" {
        return admit_rank_panel(Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]));
    }
    Err(
        "controls FRESH_OUT | FRESH_OUT V8_RESULTS V10_ADMISSION | admit RAW PARI FRESH_ADMISSION"
            .into(),
    )
}
#[cfg(test)]
mod rank_controls {
    use super::*;
    #[test]
    fn deficient_rank_can_fix_target() {
        let f = fiber(&[vec![0, 0, 1, 7]], 3, 101);
        assert_eq!(f.rank, 1);
        assert!(target_fixed(&f, 2));
        assert_eq!(f.particular[2], 7);
    }
    #[test]
    fn deficient_rank_can_leave_target_free() {
        let f = fiber(&[vec![1, 0, 0, 1], vec![0, 1, 1, 7]], 3, 101);
        assert!(!target_fixed(&f, 2));
    }
    #[test]
    fn duplicate_rows_preserve_fiber() {
        let rows = vec![vec![1, 0, 1], vec![0, 1, 7]];
        let mut dup = rows.clone();
        dup.extend(rows.clone());
        let f = fiber(&rows, 2, 101);
        let d = fiber(&dup, 2, 101);
        assert_eq!(f.particular, d.particular);
        assert_eq!(f.nullspace, d.nullspace);
        assert_eq!(f.rank, d.rank);
    }
    #[test]
    fn catalog_matches_bruteforce_on_other_field() {
        let e = Quad::new(101, 2, 3);
        let g = Some((Q(1, 0), Q(39, 0)));
        let b = vec![g, e.pm(g, 2), e.pm(g, 5)];
        let tt: Vec<_> = (0..16).map(|j| e.pm(g, 37 + j)).collect();
        for m in [2, 3] {
            let c = catalogs(e, &b, &tt, m);
            for j in 0..16 {
                assert_eq!(c[j], collect(e, &b, tt[j], m));
            }
        }
    }
}
