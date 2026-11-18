// Explicit quotient-to-quotient maps; byte-preserved v8 arithmetic prefix.
fn qcycle_produce(out: &Path, input8: &Path, input10: &Path, input11: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh quotient-cycle output required".into());
    }
    fs::create_dir_all(out)?;
    let v8: Value = serde_json::from_slice(&fs::read(input8)?)?;
    let v10: Value = serde_json::from_slice(&fs::read(input10)?)?;
    let v11: Value = serde_json::from_slice(&fs::read(input11)?)?;
    assert_eq!(
        v10["status"],
        "native-generic-and-independent-PARI-endomorphism-verified"
    );
    assert_eq!(
        v11["status"],
        "native-and-independent-PARI-kernel-symmetry-verified"
    );
    let e = Quad::new(827, 1, 15);
    let t = Quad::new(827, 4, 120);
    let r = Ring(827);
    let g = Some((Q(2, 0), Q(5, 0)));
    let q = e.pm(g, 37);
    let ktv: Vec<u64> = serde_json::from_value(v10["kernel_generator_twist"].clone())?;
    let kt = Some((Q(ktv[0], 0), Q(ktv[1], 0)));
    assert_eq!(order(t, kt, 833), 17);
    let k17: Vec<_> = (0..17).map(|k| embed(e, t.pm(kt, k))).collect();
    let alpha: (Rat, Rat) = serde_json::from_value(v10["alpha"].clone())?;
    let va: Velu = serde_json::from_value(v10["velu"].clone())?;
    let eval_alpha = |p| scale_point(e, va.eval(e, p), 7);
    let p = (Rat::poly(vec![0, 1]), Rat::poly(vec![1]));
    let mut maps = vec![];
    let mut checks = String::new();
    let mut gp=String::from("default(parisizemax,4000000000);\np=827;\nck(c,s)=if(!c,error(s));\nE=ellinit([Mod(1,p),Mod(15,p)]);T=ellinit([Mod(4,p),Mod(120,p)]);G=Mod(1,p)*[2,5];Q=ellmul(E,G,37);\n");
    gp+=&format!("kt=Mod(1,p)*{};va=ellisogeny(E,{});A=[49*va[2][1]/va[2][3]^2,343*va[2][2]/y/va[2][3]^3];\n",serde_json::to_string(&ktv)?,ppoly(&va.h));
    gp+="gc(a,b)=[subst(a[1],x,b[1]),b[2]*subst(a[2],x,b[1])];\nev(M,H,P,tw=0)={my(xx);if(P==[0],return(P));xx=P[1]/if(tw,2,1);if(subst(H,x,xx)==0,return([0]));[if(tw,2,1)*subst(M[1],x,xx),P[2]*subst(M[2],x,xx)]};\n";
    for i in 0..8 {
        let j = v11["line_permutation"][i].as_u64().unwrap() as usize;
        let vi: Velu = serde_json::from_value(v8["maps"][i]["map"].clone())?;
        let vj: Velu = serde_json::from_value(v8["maps"][j]["map"].clone())?;
        let ei = Quad::new(827, vi.a, vi.b);
        let ej = Quad::new(827, vj.a, vj.b);
        let kk: BTreeSet<_> = k17.iter().map(|p| vi.eval(e, *p)).collect();
        assert_eq!(kk.len(), 17);
        for p in &kk {
            assert!(ei.on(*p));
            assert_eq!(ei.pm(*p, 17), None);
            assert_eq!(ei.frob(*p), ei.pn(*p));
        }
        let kr = roots(&kk);
        assert_eq!(kr.len(), 8);
        let w = Velu::roots(r, vi.a, vi.b, &kr);
        assert_eq!(w.degree, 17);
        assert!(w.certify(r, vi.a, vi.b));
        assert_eq!(w.a * toy_field::pow(7, 4, 827) % 827, vj.a);
        assert_eq!(w.b * toy_field::pow(7, 6, 827) % 827, vj.b);
        let br = w.rat(r, &p.0, &p.1);
        let beta = (br.0.scale(r, 49), br.1.scale(r, 343));
        let pi = vi.rat(r, &p.0, &p.1);
        let lhs = w.rat(r, &pi.0, &pi.1);
        let lhs = (lhs.0.scale(r, 49), lhs.1.scale(r, 343));
        let rhs = vj.rat(r, &alpha.0, &alpha.1);
        assert_eq!(lhs, rhs, "generic commuting square");
        let mut corrupt = w.clone();
        corrupt.n[0] = (corrupt.n[0] + 1) % 827;
        assert!(!corrupt.certify(r, vi.a, vi.b));
        let wrong = (lhs.0.clone(), lhs.1.scale(r, 826));
        assert_ne!(wrong, rhs);
        let eval_beta = |z| scale_point(ei, w.eval(ei, z), 7);
        let mut kernel_checks = 0;
        for z in &kk {
            assert_eq!(eval_beta(*z), None);
            kernel_checks += 1;
        }
        let mut count = 0;
        let mut wrong_sign = 0;
        let mut images = BTreeSet::new();
        for z in rational(e) {
            let zi = vi.eval(e, z);
            let left = eval_beta(zi);
            let right = vj.eval(e, eval_alpha(z));
            assert!(ej.on(left));
            assert_eq!(left, right);
            images.insert(left);
            assert_eq!(left, ej.pm(vj.eval(e, z), 353));
            if z.is_some() {
                assert_ne!(ej.pn(left), right);
                wrong_sign += 1;
            }
            checks += &(serde_json::to_string(
                &json!({"line":i,"next_line":j,"source_point":pair(z),"quotient_point":pair(zi),"beta_image":pair(left),"commuting_image":pair(right)}),
            )? + "\n");
            count += 1;
        }
        assert_eq!((count, images.len(), wrong_sign), (823, 823, 822));
        let bi_g = eval_beta(vi.eval(e, g));
        let bi_q = eval_beta(vi.eval(e, q));
        assert_eq!(ej.pm(bi_g, 37), bi_q);
        assert_eq!(ej.pm(bi_g, 415), vj.eval(e, g));
        let identity_bytes = serde_json::to_vec(&lhs)?;
        let intermediate = registry::identity(827, w.a, w.b, 823, None);
        let result = json!({"line":i,"next_line":j,"source_model":v8["maps"][i]["destination"],"destination_model":v8["maps"][j]["destination"],"source_generator":v8["maps"][i]["mapped_generator"],"next_quotient_generator":v8["maps"][j]["mapped_generator"],"mapped_generator":pair(bi_g),"mapped_target":pair(bi_q),"intermediate_generator":pair(w.eval(ei,vi.eval(e,g))),"intermediate_model":intermediate,"normalized_velu":w,"kernel_roots":kr,"kernel_points":kk,"inverse_isomorphism_scale":7,"beta":beta,"degree":17,"generic_square_verified":true,"generic_composition_x_degree":lhs.0.n.len()-1,"generic_identity_normal_form_sha256":registry::digest(&identity_bytes),"source_rational_checks":count,"image_cardinality":images.len(),"kernel_checks":kernel_checks,"wrong_sign_rejections":wrong_sign,"corrupt_curve_map_rejected":true});
        gp+=&format!("Ei=ellinit([Mod({},p),Mod({},p)]);Ej=ellinit([Mod({},p),Mod({},p)]);Ti=ellinit([Mod({},p),Mod({},p)]);vi=ellisogeny(E,{});vj=ellisogeny(E,{});Pi=[vi[2][1]/vi[2][3]^2,vi[2][2]/y/vi[2][3]^3];Pj=[vj[2][1]/vj[2][3]^2,vj[2][2]/y/vj[2][3]^3];Z=ev(Pi,vi[2][3],kt,1);ck(ellisoncurve(Ti,Z)&&ellorder(Ti,Z)==17,\"image17 kernel\");rr=Set(vector(16,k,lift(ellmul(Ti,Z,k)[1]/2)));ck(rr=={},\"independent17 roots\");w=ellisogeny(Ei,prod(k=1,#rr,x-Mod(rr[k],p)));ck(w[1]==[0,0,0,Mod({},p),Mod({},p)]&&w[2]=={},\"normalized map\");B=[49*w[2][1]/w[2][3]^2,343*w[2][2]/y/w[2][3]^3];ck(B==[{},{}],\"beta coordinates\");ck(Mod(1,p)*(x^3+{}*x+{})*B[2]^2==B[1]^3+{}*B[1]+{},\"curve identity\");ck(gc(B,Pi)==gc(Pj,A),\"generic commuting square\");\n",vi.a,vi.b,vj.a,vj.b,4*vi.a%827,8*vi.b%827,ppoly(&vi.h),ppoly(&vj.h),serde_json::to_string(&kr)?,w.a,w.b,gp_map(&w),prat(&beta.0),prat(&beta.1),vi.a,vi.b,vj.a,vj.b);
        gp+="for(k=0,822,P=ellmul(E,G,k);PiP=ev(Pi,vi[2][3],P);L=ev(B,w[2][3],PiP);R=ev(Pj,vj[2][3],ev(A,va[2][3],P));ck(ellisoncurve(Ej,L)&&L==R&&L==ellmul(Ej,ev(Pj,vj[2][3],P),353),\"all rational points\");if(k,ck(ellneg(Ej,L)!=R,\"wrong sign control\")));for(k=0,16,ck(ev(B,w[2][3],ellmul(Ti,Z,k),1)==[0],\"all17 kernel points\"));\n";
        gp+=&format!("BG=ev(B,w[2][3],ev(Pi,vi[2][3],G));BQ=ev(B,w[2][3],ev(Pi,vi[2][3],Q));ck(lift(BG)=={}&&lift(BQ)=={}&&ellmul(Ej,BG,37)==BQ&&ellmul(Ej,BG,415)==ev(Pj,vj[2][3],G),\"generator target and inverse\");\n",serde_json::to_string(&pair(bi_g))?,serde_json::to_string(&pair(bi_q))?);
        maps.push(result);
    }
    gp += "print(\"ALL_TWIST_QUOTIENT_SQUARES_OK 8 6584 136\");\n";
    let result = json!({"schema":"twist.quotient-cycle/v1","p":827,"source":v8["source"],"degree":17,"line_cycle":v11["line_cycles"][0],"maps":maps,"summary":{"maps":8,"generic_squares":8,"rational_checks":6584,"kernel_checks":136,"wrong_sign_rejections":6576,"corrupt_maps_rejected":8},"scalar_action_relative_to_next_generator":353,"scalar_inverse":415,"performance_claim":false,"novelty_claim":false,"point_checks_sha256":registry::digest(checks.as_bytes())});
    fs::write(out.join("point-checks.jsonl"), checks)?;
    fs::write(
        out.join("RESULTS.json"),
        serde_json::to_string_pretty(&result)? + "\n",
    )?;
    fs::write(out.join("verify.gp"), gp)?;
    println!("{}", result["summary"]);
    Ok(())
}
fn qcycle_admit(raw: &Path, pari: &Path, out: &Path) -> Result<()> {
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
        "ALL_TWIST_QUOTIENT_SQUARES_OK 8 6584 136"
    );
    let err = String::from_utf8(se)?;
    assert!(
        err.trim().is_empty()
            || err.trim()
                == "***   Warning: new maximum stack size = 4000002048 (3814.699 Mbytes)."
    );
    assert_eq!(
        d["point_checks_sha256"],
        hash(&raw.join("point-checks.jsonl"))?
    );
    d["native_result_sha256"] = json!(hash(&raw.join("RESULTS.json"))?);
    d["pari_receipt_sha256"] = json!(hash(&pari.join("RECEIPT.json"))?);
    d["status"] = json!("native-and-independent-PARI-quotient-squares-verified");
    fs::write(out, serde_json::to_string_pretty(&d)? + "\n")?;
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() == 4 && a[0] != "admit" {
        return qcycle_produce(
            Path::new(&a[0]),
            Path::new(&a[1]),
            Path::new(&a[2]),
            Path::new(&a[3]),
        );
    }
    if a.len() == 4 && a[0] == "admit" {
        return qcycle_admit(Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]));
    }
    Err("FRESH_OUT V8_RESULTS V10_ADMISSION V11_ADMISSION | admit RAW PARI FRESH_ADMISSION".into())
}
#[cfg(test)]
mod quotient_controls {
    use super::*;
    #[test]
    fn scaled_composition_and_sign_control() {
        let r = Ring(101);
        let p = (Rat::poly(vec![0, 1]), Rat::poly(vec![1]));
        let first = (p.0.scale(r, 4), p.1.scale(r, 8));
        let lhs = (first.0.scale(r, 9), first.1.scale(r, 27));
        let rhs = (p.0.scale(r, 36), p.1.scale(r, 216 % 101));
        assert_eq!(lhs, rhs);
        assert_ne!((lhs.0, lhs.1.scale(r, 100)), rhs);
    }
}
