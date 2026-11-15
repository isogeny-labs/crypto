//! Deterministic work caps and append-only journals; no partial rank claims.
use super::*;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(Clone, Debug)]
pub struct Limits {
    pub columns: usize,
    pub coefficients: usize,
    pub reductions: usize,
    pub basis_entries: usize,
}
impl Limits {
    fn value(&self) -> Value {
        json!({"columns":self.columns,"coefficients":self.coefficients,
               "reductions":self.reductions,"basis_entries":self.basis_entries})
    }
    pub fn read(v: &Value) -> Self {
        let get=|k:&str| usize::try_from(v[k].as_u64().unwrap()).unwrap();
        Self{columns:get("columns"),coefficients:get("coefficients"),reductions:get("reductions"),basis_entries:get("basis_entries")}
    }
}
fn pairs(row:&Row)->Vec<Value>{row.iter().map(|(&i,c)|json!([i,c.to_string()])).collect()}
fn terms(poly:&Polynomial)->Vec<Value>{poly.iter().map(|(m,c)|json!([m,c.to_string()])).collect()}

/// All caps are tested at atomic operation boundaries. A partially constructed
/// coefficient matrix is NEVER interpreted as a matrix of the original map.
fn compute(m:usize,ell:usize,p:u128,limit:&Limits,emit:&mut impl FnMut(Value))->Value {
    let nb=m*ell;let nv=nb+m-2;let fs=quartics(m,ell,p);
    let domain=quadratic_multipliers(nb,nv);let ncols=(m-1)*domain.len();
    emit(json!({"event":"header","schema":"n83.quadratic-journal/v1","m":m,"ell":ell,"p":p.to_string(),"limits":limit.value(),"columns":ncols}));
    let mut rows=BTreeMap::<Vec<u8>,Row>::new();let mut count=0usize;let mut columns=0;
    let capped=|reason:&str,columns:usize,count:usize,updates:usize,rank:usize,matrix_complete:bool|{
        json!({"status":"capped","reason":reason,"columns_completed":columns,"coefficient_entries":count,
            "reduction_updates":updates,"independent_rows_retained":rank,"matrix_complete":matrix_complete,
            "certificate":null,"first_fall_lower_bound":null})
    };
    for (j,f) in fs.iter().enumerate(){for (q,monomial) in domain.iter().enumerate(){
        let column=j*domain.len()+q;
        if columns>=limit.columns{return capped("columns",columns,count,0,0,false);}
        let factor=[(monomial.clone(),1)].into();let product=product(f,&factor,nb,p);
        if product.len()>limit.coefficients.saturating_sub(count){
            emit(json!({"event":"pending_column","column":column,"terms":terms(&product)}));
            return capped("coefficients",columns,count,0,0,false);
        }
        emit(json!({"event":"column","column":column,"terms":terms(&product)}));
        count+=product.len();columns+=1;
        for (mon,c) in product {assert_eq!(mon.len(),6);put(rows.entry(mon).or_default(),column,c,p);}
    }}
    emit(json!({"event":"matrix_complete","nonzero_rows":rows.len(),"coefficient_entries":count}));
    let mut basis=BTreeMap::<usize,Row>::new();let mut pivots=Vec::new();
    let mut updates=0usize;let mut basis_entries=0usize;
    for (cursor,(mon,raw)) in rows.iter().enumerate(){
        let mut row=raw.clone();let mut new_pivot=Value::Null;
        loop {
            let Some((&lead,&c))=row.first_key_value() else {break};
            if let Some(old)=basis.get(&lead){
                if old.len()>limit.reductions.saturating_sub(updates){
                    emit(json!({"event":"pending_row","cursor":cursor,"monomial":mon,"next_reducer":lead,"reduced_row":pairs(&row)}));
                    return capped("reductions",columns,count,updates,basis.len(),true);
                }
                updates+=old.len();for (&i,&v) in old {put(&mut row,i,neg(mul(c,v,p),p),p);}
            }else{
                if row.len()>limit.basis_entries.saturating_sub(basis_entries){
                    emit(json!({"event":"pending_row","cursor":cursor,"monomial":mon,"next_reducer":null,"reduced_row":pairs(&row)}));
                    return capped("basis_entries",columns,count,updates,basis.len(),true);
                }
                let inverse=power(c,p-2,p);assert_eq!(mul(c,inverse,p),1);
                let pivot=json!({"monomial":mon,"column":lead,"pivot":c.to_string(),"original_row":pairs(raw)});
                pivots.push(pivot.clone());new_pivot=pivot;
                for v in row.values_mut(){*v=mul(*v,inverse,p);}
                basis_entries+=row.len();basis.insert(lead,row);break;
            }
        }
        emit(json!({"event":"row_complete","cursor":cursor,"monomial":mon,"pivot":new_pivot,"reduction_updates":updates,"basis_entries":basis_entries}));
        if basis.len()==ncols{break;}
    }
    let c=json!({"schema":"n83.prime-quadratic-template/v1","m":m,"ell_projected":ell,"multiplier_degree":2,"p":p.to_string(),"columns":ncols,"nonzero_coefficient_rows":rows.len(),"rank":basis.len(),"injective":basis.len()==ncols,"pivot_rows":pivots});
    json!({"status":"complete","matrix_complete":true,"columns_completed":columns,"coefficient_entries":count,
        "reduction_updates":updates,"basis_entries":basis_entries,"certificate":c})
}

pub fn execute(m:usize,ell:usize,p:u128,limit:&Limits,out:&Path,replay:bool)->Value {
    if replay {
        let meta=math::read(&out.join("identity.json"));
        assert_eq!(meta,json!({"m":m,"ell":ell,"p":p.to_string(),"limits":limit.value()}));
        let file=fs::File::open(out.join("journal.jsonl")).unwrap();let mut lines=BufReader::new(file).lines();
        let mut compare=|event:Value|{
            let line=lines.next().expect("journal is an incomplete prefix").unwrap();
            assert_eq!(serde_json::from_str::<Value>(&line).unwrap(),event,"journal event mismatch");
        };
        let result=compute(m,ell,p,limit,&mut compare);compare(json!({"event":"terminal","result":result}));
        drop(compare);assert!(lines.next().is_none(),"extra journal event");
        assert_eq!(math::read(&out.join("result.json")),result);result
    }else{
        assert!(!out.exists());fs::create_dir_all(out).unwrap();
        math::save(&out.join("identity.json"),&json!({"m":m,"ell":ell,"p":p.to_string(),"limits":limit.value()}));
        let file=fs::OpenOptions::new().create_new(true).write(true).open(out.join("journal.jsonl")).unwrap();
        let mut writer=BufWriter::new(file);
        let mut append=|event:Value|{serde_json::to_writer(&mut writer,&event).unwrap();writer.write_all(b"\n").unwrap();writer.flush().unwrap();};
        let result=compute(m,ell,p,limit,&mut append);append(json!({"event":"terminal","result":result}));
        math::save(&out.join("result.json"),&result);result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize,Ordering};
    static NEXT:AtomicUsize=AtomicUsize::new(0);
    fn dir()->std::path::PathBuf {
        let root=std::env::var_os("N83_QUADRATIC_TEST_ARTIFACTS").map(std::path::PathBuf::from).unwrap_or_else(||std::env::temp_dir().join("n83-quadratic-cap-tests"));
        root.join(format!("{}-{}",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed)))
    }
    fn ample()->Limits{Limits{columns:100000,coefficients:10000000,reductions:10000000,basis_entries:1000000}}
    #[test]fn complete_matches_unbounded_and_replays(){
        for ell in [3,7] {
            let d=dir();let limits=ample();let result=execute(3,ell,101,&limits,&d,false);
            assert_eq!(result["status"],"complete");assert_eq!(result["certificate"],certificate(3,ell,101));
            assert_eq!(execute(3,ell,101,&limits,&d,true),result);
        }
    }
    #[test]fn every_cap_is_retained_and_replayed(){
        for name in ["columns","coefficients","reductions","basis_entries"]{
            let mut limit=ample();match name{"columns"=>limit.columns=0,"coefficients"=>limit.coefficients=0,"reductions"=>limit.reductions=0,_=>limit.basis_entries=0}
            let d=dir();let result=execute(3,3,101,&limit,&d,false);
            assert_eq!(result["status"],"capped");assert_eq!(result["reason"],name);
            assert!(result["certificate"].is_null());assert!(result["first_fall_lower_bound"].is_null());
            assert_eq!(execute(3,3,101,&limit,&d,true),result);
        }
    }
    #[test]fn interrupted_and_extra_journals_rejected(){
        for extra in [false,true]{
            let d=dir();let mut limit=ample();limit.columns=0;execute(3,3,101,&limit,&d,false);
            let path=d.join("journal.jsonl");let original=fs::read(&path).unwrap();fs::write(d.join("original-journal.jsonl"),&original).unwrap();
            if extra {let mut file=fs::OpenOptions::new().append(true).open(&path).unwrap();writeln!(file,"{{}}").unwrap();}
            else {fs::write(&path,&original[..original.len()/2]).unwrap();}
            assert!(std::panic::catch_unwind(||execute(3,3,101,&limit,&d,true)).is_err());
        }
    }
}
