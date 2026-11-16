// Exact finite module diagnostic. Original arithmetic prefix remains unchanged.
type Mat2 = [[u64; 2]; 2];
const ID7: Mat2 = [[1, 0], [0, 1]];
const COMP7: Mat2 = [[0, 4], [1, 1]];
fn mm7(a: Mat2, b: Mat2) -> Mat2 {
    std::array::from_fn(|i| std::array::from_fn(|j| (a[i][0] * b[0][j] + a[i][1] * b[1][j]) % 7))
}
fn mv7(a: Mat2, v: [u64; 2]) -> [u64; 2] {
    [
        (a[0][0] * v[0] + a[0][1] * v[1]) % 7,
        (a[1][0] * v[0] + a[1][1] * v[1]) % 7,
    ]
}
fn mp7(a: Mat2, n: usize) -> Mat2 {
    (0..n).fold(ID7, |v, _| mm7(v, a))
}
fn scalar7(a: Mat2) -> bool {
    a[0][1] == 0 && a[1][0] == 0 && a[0][0] == a[1][1] && a[0][0] != 0
}
fn mo7(a: Mat2, projective: bool) -> usize {
    (1..=48)
        .find(|n| {
            if projective {
                scalar7(mp7(a, *n))
            } else {
                mp7(a, *n) == ID7
            }
        })
        .expect("order outside declared GL2 irreducible/control bound")
}
fn line7(v: [u64; 2]) -> usize {
    assert_ne!(v, [0, 0]);
    if v[0] == 0 {
        7
    } else {
        (v[1] * inv(v[0], 7) % 7) as usize
    }
}
fn point_perm7(a: Mat2) -> Vec<usize> {
    (0..49)
        .map(|k| {
            let v = mv7(a, [k / 7, k % 7]);
            (7 * v[0] + v[1]) as usize
        })
        .collect()
}
fn line_perm7(a: Mat2) -> Vec<usize> {
    (0..8)
        .map(|k| line7(mv7(a, if k == 7 { [0, 1] } else { [1, k] })))
        .collect()
}
fn cycles7(perm: &[usize]) -> Vec<Vec<usize>> {
    assert_eq!(
        perm.iter().copied().collect::<BTreeSet<_>>(),
        (0..perm.len()).collect()
    );
    let mut seen = vec![false; perm.len()];
    let mut cycles = vec![];
    for i in 0..perm.len() {
        if seen[i] {
            continue;
        }
        let mut j = i;
        let mut c = vec![];
        while !seen[j] {
            seen[j] = true;
            c.push(j);
            j = perm[j];
        }
        assert_eq!(j, i);
        cycles.push(c);
    }
    cycles
}
fn parse_pair7(v: &Value) -> P {
    let a: Vec<u64> = serde_json::from_value(v.clone()).unwrap();
    if a == vec![0] {
        None
    } else {
        assert_eq!(a.len(), 2);
        Some((Q(a[0], 0), Q(a[1], 0)))
    }
}
fn unembed7(e: Quad, p: P) -> P {
    p.map(|(x, y)| {
        assert_eq!((x.1, y.0), (0, 0));
        (Q(2 * x.0 % e.p, 0), Q(4 * y.1 % e.p, 0))
    })
}
fn gp_mat7(m: Mat2) -> String {
    format!("Mod(1,7)*[{},{};{},{}]", m[0][0], m[0][1], m[1][0], m[1][1])
}
fn symmetry_produce(out: &Path, input8: &Path, input10: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh kernel-symmetry output required".into());
    }
    fs::create_dir_all(out)?;
    let v8: Value = serde_json::from_slice(&fs::read(input8)?)?;
    let v10: Value = serde_json::from_slice(&fs::read(input10)?)?;
    assert_eq!(
        v10["status"],
        "native-generic-and-independent-PARI-endomorphism-verified"
    );
    let e = Quad::new(827, 1, 15);
    let t = Quad::new(827, 4, 120);
    let b1 = parse_pair7(&v8["torsion_basis"][0]);
    let b2 = parse_pair7(&v8["torsion_basis"][1]);
    assert_eq!(pair(b1), vec![0, 51]);
    assert_eq!(pair(b2), vec![6, 130]);
    let points: Vec<P> = (0..49)
        .map(|k| t.pa(t.pm(b1, k / 7), t.pm(b2, k % 7)))
        .collect();
    let archived: BTreeSet<P> = v8["torsion_points"]
        .as_array()
        .unwrap()
        .iter()
        .map(parse_pair7)
        .collect();
    assert_eq!(points.iter().copied().collect::<BTreeSet<_>>(), archived);
    assert_eq!(archived.len(), 49);
    let velu: Velu = serde_json::from_value(v10["velu"].clone())?;
    assert!(velu.certify(Ring(827), 1, 15));
    assert_eq!(velu.degree, 17);
    let eval = |p| unembed7(e, scale_point(e, velu.eval(e, embed(e, p)), 7));
    let images: Vec<P> = points.iter().map(|p| eval(*p)).collect();
    let label = |p: P| {
        points
            .iter()
            .position(|q| *q == p)
            .expect("image not seven-torsion")
    };
    let c1 = label(eval(b1));
    let c2 = label(eval(b2));
    let m = [
        [c1 as u64 / 7, c2 as u64 / 7],
        [c1 as u64 % 7, c2 as u64 % 7],
    ];
    let pp = point_perm7(m);
    let lp = line_perm7(m);
    let pc = cycles7(&pp);
    let lc = cycles7(&lp);
    let mut rows = vec![];
    let mut bad = m;
    bad[0][0] = (bad[0][0] + 1) % 7;
    let mut corrupt_rejections = 0;
    for (k, p) in points.iter().enumerate() {
        let image = images[k];
        assert!(t.on(image));
        assert_eq!(t.pm(image, 7), None);
        assert_eq!(image, points[pp[k]]);
        let mut p8 = *p;
        for _ in 0..8 {
            p8 = eval(p8);
        }
        assert_eq!(p8, points[point_perm7(mp7(m, 8))[k]]);
        if point_perm7(bad)[k] != label(image) {
            corrupt_rejections += 1;
        }
        rows.push(json!({"index":k,"coordinates":[k/7,k%7],"point":pair(*p),"image":pair(image),"image_index":label(image),"eighth_image":pair(p8)}));
    }
    assert!(corrupt_rejections > 0);
    let mut additions = 0;
    for p in &points {
        for q in &points {
            assert_eq!(eval(t.pa(*p, *q)), t.pa(eval(*p), eval(*q)));
            additions += 1;
        }
    }
    assert_eq!(additions, 2401);
    let trace = (m[0][0] + m[1][1]) % 7;
    let determinant = (m[0][0] * m[1][1] + 7 - m[0][1] * m[1][0] % 7) % 7;
    assert_eq!((trace, determinant), (1, 3));
    let m2 = mm7(m, m);
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!((m2[i][j] + 7 - m[i][j] + if i == j { 3 } else { 0 }) % 7, 0);
        }
    }
    // In the cyclic basis(v,Mv), the action is the fixed companion matrix.
    let cb = [[1, m[0][0]], [0, m[1][0]]];
    assert_ne!(m[1][0], 0);
    assert_eq!(mm7(m, cb), mm7(cb, COMP7));
    let mut kernel_rows = vec![];
    for line in 0..8 {
        let p = parse_pair7(&v8["maps"][line]["twist_kernel_generator"]);
        assert_eq!(line7([label(p) as u64 / 7, label(p) as u64 % 7]), line);
        let image = eval(p);
        let image_points: BTreeSet<_> = (0..7).map(|k| embed(e, t.pm(image, k))).collect();
        let rr = roots(&image_points);
        let expected: Vec<u64> =
            serde_json::from_value(v8["maps"][lp[line]]["kernel_roots"].clone())?;
        assert_eq!(rr, expected);
        kernel_rows.push(json!({"line":line,"generator":pair(p),"image_generator":pair(image),"image_line":lp[line],"image_kernel_roots":rr}));
    }
    let mut bad_lp = lp.clone();
    bad_lp.swap(0, 1);
    let bad_lines = (0..8).filter(|i| bad_lp[*i] != lp[*i]).count();
    assert_eq!(bad_lines, 2);
    let controls = [
        ("identity", ID7, 1, 1),
        ("scalar2", [[2, 0], [0, 2]], 3, 1),
        ("companion", COMP7, 48, 8),
    ];
    let mut control_rows = vec![];
    for (name, a, po, lo) in controls {
        let point_order = mo7(a, false);
        let projective_order = mo7(a, true);
        assert_eq!((point_order, projective_order), (po, lo));
        control_rows.push(json!({"name":name,"matrix":a,"point_order":point_order,"projective_order":projective_order,"point_cycles":cycles7(&point_perm7(a)),"line_cycles":cycles7(&line_perm7(a))}));
    }
    let point_order = mo7(m, false);
    let projective_order = mo7(m, true);
    let predicted = point_order == 48
        && projective_order == 8
        && pc
            .iter()
            .filter(|c| c[0] != 0)
            .map(Vec::len)
            .collect::<Vec<_>>()
            == vec![48]
        && lc.len() == 1
        && lc[0].len() == 8;
    let result = json!({"schema":"twist.kernel-symmetry/v1","p":827,"torsion_prime":7,"basis":v8["torsion_basis"],"source":v8["source"],"twist":v8["twist"],"matrix":m,"trace":trace,"determinant":determinant,"companion_matrix":COMP7,"companion_change_basis":cb,"eighth_matrix":mp7(m,8),"point_order":point_order,"projective_order":projective_order,"point_permutation":pp,"line_permutation":lp,"point_cycles":pc,"line_cycles":lc,"point_rows":rows,"kernel_rows":kernel_rows,"controls":control_rows,"point_checks":49,"addition_checks":additions,"kernel_checks":8,"corrupt_matrix_rejections":corrupt_rejections,"corrupt_line_rejections":bad_lines,"filed_prediction_48_and_8_holds":predicted,"performance_claim":false,"novelty_claim":false,"source_scalar_order_823_group":274});
    fs::write(
        out.join("RESULTS.json"),
        serde_json::to_string_pretty(&result)? + "\n",
    )?;
    let mut gp=String::from("default(parisizemax,4000000000);\np=827;\nck(c,s)=if(!c,error(s));\nT=ellinit([Mod(4,p),Mod(120,p)]);E=ellinit([Mod(1,p),Mod(15,p)]);T1=Mod(1,p)*[0,51];T2=Mod(1,p)*[6,130];\n");
    gp += &format!(
        "v=ellisogeny(E,{});H=v[2][3];A=[49*v[2][1]/H^2,343*v[2][2]/y/H^3];\n",
        ppoly(&velu.h)
    );
    gp+="ev(P)={my(xx);if(P==[0],return(P));xx=P[1]/2;ck(subst(H,x,xx)!=0,\"unexpected seven-torsion pole\");[2*subst(A[1],x,xx),P[2]*subst(A[2],x,xx)]};\npv=vector(49,k,elladd(T,ellmul(T,T1,(k-1)\\7),ellmul(T,T2,(k-1)%7)));ck(#Set(pv)==49,\"basis\");\nci(P)={for(k=1,49,if(pv[k]==P,return(k-1)));error(\"missing torsion image\")};\n";
    gp += "i1=ci(ev(T1));i2=ci(ev(T2));M=Mod(1,7)*[i1\\7,i2\\7;i1%7,i2%7];\n";
    gp+=&format!("ck(M=={},\"native matrix\");ck(trace(M)==Mod(1,7)&&matdet(M)==Mod(3,7)&&M^2-M+3*matid(2)==0*matid(2),\"characteristic polynomial\");\n",gp_mat7(m));
    gp+="ap(M,k)={my(v=M*[k\\7,k%7]~);7*lift(v[1])+lift(v[2])};\nlp(M,k)={my(v=M*if(k==7,[0,1]~,[1,k]~));if(v[1]==0,7,lift(v[2]/v[1]))};\ncyc(v)={my(seen=vector(#v),out=List(),j,c);for(i=1,#v,if(!seen[i],j=i;c=List();while(!seen[j],seen[j]=1;listput(c,j-1);j=v[j]+1);ck(j==i,\"not permutation\");listput(out,Vec(c))));Vec(out)};\nord(M,pr=0)={my(B);for(k=1,48,B=M^k;if(if(pr,B[1,2]==0&&B[2,1]==0&&B[1,1]==B[2,2]&&B[1,1]!=0,B==matid(2)),return(k)));error(\"order bound\")};\n";
    gp+=&format!("pp=vector(49,k,ap(M,k-1));ll=vector(8,k,lp(M,k-1));ck(pp=={}&&ll=={}&&cyc(pp)=={}&&cyc(ll)=={},\"complete permutations and cycles\");ck(ord(M)=={}&&ord(M,1)=={}&&M^8=={},\"orders/eighth power\");\n",serde_json::to_string(&result["point_permutation"])?,serde_json::to_string(&result["line_permutation"])?,serde_json::to_string(&result["point_cycles"])?,serde_json::to_string(&result["line_cycles"])?,point_order,projective_order,gp_mat7(mp7(m,8)));
    gp += &format!(
        "cb={};ck(M*cb==cb*{},\"cyclic basis conjugacy\");\n",
        gp_mat7(cb),
        gp_mat7(COMP7)
    );
    gp+="for(k=1,49,P=pv[k];Z=ev(P);ck(ellisoncurve(T,Z)&&ellmul(T,Z,7)==[0]&&ci(Z)==pp[k],\"point action\");W=P;for(j=1,8,W=ev(W));ck(ci(W)==ap(M^8,k-1),\"eighth point action\"));\nfor(i=1,49,for(j=1,49,ck(ev(elladd(T,pv[i],pv[j]))==elladd(T,ev(pv[i]),ev(pv[j])),\"all additions\")));\n";
    for k in &kernel_rows {
        gp+=&format!("P=Mod(1,p)*{};Z=ev(P);rr=Set(vector(6,k,lift(ellmul(T,Z,k)[1]/2)));ck(rr=={}&&lp(M,{})=={},\"image kernel roots\");\n",serde_json::to_string(&k["generator"])?,serde_json::to_string(&k["image_kernel_roots"])?,k["line"],k["image_line"]);
    }
    gp+=&format!("B=M;B[1,1]++;br=sum(k=0,48,ap(B,k)!=ci(ev(pv[k+1])));ck(br=={},\"corrupt matrix\");bl=ll;z=bl[1];bl[1]=bl[2];bl[2]=z;ck(sum(k=1,8,bl[k]!=ll[k])==2,\"corrupt lines\");\n",corrupt_rejections);
    for (_, a, po, lo) in controls {
        gp+=&format!("C={};ck(ord(C)=={}&&ord(C,1)=={},\"control orders\");ck(cyc(vector(49,k,ap(C,k-1)))=={}&&cyc(vector(8,k,lp(C,k-1)))=={},\"control cycles\");\n",gp_mat7(a),po,lo,serde_json::to_string(&cycles7(&point_perm7(a)))?,serde_json::to_string(&cycles7(&line_perm7(a)))?);
    }
    gp += "print(\"ALL_TWIST_KERNEL_SYMMETRY_OK 49 2401 8\");\n";
    fs::write(out.join("verify.gp"), gp)?;
    println!(
        "{}",
        json!({"point_order":point_order,"projective_order":projective_order,"prediction_holds":predicted,"matrix":m})
    );
    Ok(())
}
fn symmetry_admit(raw: &Path, pari: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh admission required".into());
    }
    let mut d: Value = serde_json::from_slice(&fs::read(raw.join("RESULTS.json"))?)?;
    let p: Value = serde_json::from_slice(&fs::read(pari.join("RECEIPT.json"))?)?;
    let hash = |p: &Path| -> Result<String> { Ok(registry::digest(&fs::read(p)?)) };
    assert_eq!(p["exit_code"], 0);
    assert!(p["signal"].is_null());
    assert_eq!(p["script_sha256"], hash(&raw.join("verify.gp"))?);
    let so = fs::read(pari.join("pari.stdout"))?;
    let se = fs::read(pari.join("pari.stderr"))?;
    assert_eq!(p["stdout_sha256"], registry::digest(&so));
    assert_eq!(p["stderr_sha256"], registry::digest(&se));
    assert_eq!(
        String::from_utf8(so)?.trim(),
        "ALL_TWIST_KERNEL_SYMMETRY_OK 49 2401 8"
    );
    let err = String::from_utf8(se)?;
    assert!(
        err.trim().is_empty()
            || err.trim()
                == "***   Warning: new maximum stack size = 4000002048 (3814.699 Mbytes)."
    );
    assert_eq!(d["point_checks"], 49);
    assert_eq!(d["addition_checks"], 2401);
    assert_eq!(d["kernel_checks"], 8);
    d["native_result_sha256"] = json!(hash(&raw.join("RESULTS.json"))?);
    d["pari_receipt_sha256"] = json!(hash(&pari.join("RECEIPT.json"))?);
    d["status"] = json!("native-and-independent-PARI-kernel-symmetry-verified");
    fs::write(out, serde_json::to_string_pretty(&d)? + "\n")?;
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() == 3 {
        return symmetry_produce(Path::new(&a[0]), Path::new(&a[1]), Path::new(&a[2]));
    }
    if a.len() == 4 && a[0] == "admit" {
        return symmetry_admit(Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]));
    }
    Err("FRESH_OUT V8_RESULTS V10_ADMISSION | admit RAW PARI FRESH_ADMISSION".into())
}
#[cfg(test)]
mod symmetry_controls {
    use super::*;
    #[test]
    fn identity_vs_scalar_orders() {
        assert_eq!((mo7(ID7, false), mo7(ID7, true)), (1, 1));
        assert_eq!(
            (mo7([[2, 0], [0, 2]], false), mo7([[2, 0], [0, 2]], true)),
            (3, 1)
        );
        assert_eq!(cycles7(&point_perm7(ID7)).len(), 49);
    }
    #[test]
    fn companion_exact_orbits() {
        assert_eq!((mo7(COMP7, false), mo7(COMP7, true)), (48, 8));
        assert_eq!(mp7(COMP7, 8), [[3, 0], [0, 3]]);
        assert_eq!(
            cycles7(&point_perm7(COMP7))
                .iter()
                .map(Vec::len)
                .collect::<Vec<_>>(),
            vec![1, 48]
        );
        assert_eq!(cycles7(&line_perm7(COMP7))[0].len(), 8);
    }
    #[test]
    fn corrupt_matrix_is_detected() {
        let mut b = COMP7;
        b[0][0] = 1;
        assert_ne!(point_perm7(b), point_perm7(COMP7));
        assert_ne!(line_perm7(b), line_perm7(COMP7));
    }
}
