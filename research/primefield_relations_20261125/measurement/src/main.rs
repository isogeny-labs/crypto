//! Fixed-fixture operation census, with no clocks, searches, or timing ratios.
//! New measurement source: original correctness/evidence files remain unchanged.
#[path = "counted_native.rs"]
pub mod reused_native;
mod archive_source;

use archive_source::Labelled;
use reused_native::{check_curve, Arithmetic, Base, Point, PRIME31};
use std::collections::{BTreeMap, BTreeSet};

const P: u64 = 2_147_483_647;
const R: u64 = 2_147_446_831;
const Q: (u64, u64) = (1_477_799_919, 824_532_935);
const DEPTH: usize = 3;
const SEED: u64 = 0x4150_535f_5052_3331;
const IDS: [usize; 3] = [437, 1536 + 1247, 3072 + 879];
const ORIGINAL_SIGNS: [i8; 3] = [-1, 1, -1];
const FROZEN: &str = include_str!("../../shared-terminal-gate/results/attempt-001/result.json");
// Two A/A controls, then five pairs in orders AB, BA, AB, BA, AB.
const SCHEDULE: [char; 12] = ['A', 'A', 'A', 'B', 'B', 'A', 'A', 'B', 'B', 'A', 'A', 'B'];
type Poly = Vec<u64>;

macro_rules! custom_counts {
    ($($field:ident),+ $(,)?) => {
        #[derive(Default)]
        struct CustomCounts { $( $field: u64, )+ }
        impl CustomCounts {
            fn append(&self, out: &mut BTreeMap<&'static str, u64>) {
                $( out.insert(concat!("custom.", stringify!($field)), self.$field); )+
            }
        }
    };
}
custom_counts!(
    fp_modular_products_including_squares, fp_additions, fp_subtractions,
    fp_exponentiation_calls, fp_inversion_calls,
    fr_modular_products_including_squares, fr_additions, fr_subtractions,
    fr_exponentiation_calls, fr_inversion_calls, chain_steps,
    polynomial_multiply_calls, polynomial_coefficient_products,
    polynomial_division_calls, polynomial_division_elimination_steps,
    polynomial_division_coefficient_updates, polynomial_gcd_calls,
    polynomial_modular_exponentiation_calls, polynomial_evaluation_calls,
    polynomial_horner_steps, inverse_fiber_nodes, inverse_set_insert_attempts,
    union_set_insert_attempts, membership_queries, membership_key_comparisons,
    roster_lookup_queries, roster_point_comparisons, roster_sort_key_comparisons,
    public_tag_validations, canonicalized_records, source_records
);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Snapshot(BTreeMap<&'static str, u64>);
impl Snapshot {
    fn delta(&self, before: &Self) -> Self {
        assert!(self.0.keys().eq(before.0.keys()));
        Self(self.0.iter().map(|(&k, &v)| (k, v.checked_sub(before.0[k]).expect("counter decreased"))).collect())
    }
    fn plus(&self, rhs: &Self) -> Self {
        assert!(self.0.keys().eq(rhs.0.keys()));
        Self(self.0.iter().map(|(&k, &v)| (k, v.checked_add(rhs.0[k]).expect("counter overflow"))).collect())
    }
    fn json(&self) -> String {
        format!("{{{}}}", self.0.iter().map(|(k,v)|format!("\"{k}\":{v}")).collect::<Vec<_>>().join(","))
    }
    fn get(&self, name: &str) -> u64 { self.0[name] }
}

#[derive(Default)]
struct Meter { ar: Arithmetic, c: CustomCounts }
impl Meter {
    fn snapshot(&self) -> Snapshot {
        let c=&self.ar.counts;
        let mut out=BTreeMap::from([
            ("native.group_add_calls", c.group_additions),
            ("native.identity_add_calls", c.identity_add_calls),
            ("native.inverse_pair_add_calls", c.inverse_pair_add_calls),
            ("native.double_calls", c.double_calls),
            ("native.general_add_calls", c.general_add_calls),
            ("native.fp_modular_products_including_squares", c.fp_multiplications),
            ("native.fp_inversion_calls", c.fp_inversions),
            ("native.point_scalar_calls", c.point_scalar_calls),
            ("native.fp_sqrt_attempts", c.sqrt_attempts),
            ("native.fp_sqrt_successes", c.sqrt_successes),
            ("native.curve_rhs_calls", c.rhs_calls),
            ("native.on_curve_calls", c.on_curve_calls),
            ("native.fr_modular_products_including_squares", c.scalar_multiplications),
            ("native.fr_inversion_calls", c.scalar_inversions),
            ("native.base_seed_trials", c.base_seed_trials),
            ("native.base_chain_steps", c.base_chain_steps),
            ("native.legacy_polynomial_evaluations", c.polynomial_evaluations),
            ("native.legacy_extraction_nodes", c.extraction_nodes),
        ]);
        self.c.append(&mut out);
        Snapshot(out)
    }
}
#[derive(Clone)]
struct Phase { name: &'static str, before: Snapshot, after: Snapshot, delta: Snapshot }
fn phase<T>(m: &mut Meter, phases: &mut Vec<Phase>, name: &'static str,
    f: impl FnOnce(&mut Meter)->Result<T,String>) -> Result<T,String> {
    let before=m.snapshot();
    let value=f(m).map_err(|e|format!("{name}: {e}"));
    let after=m.snapshot();
    let delta=after.delta(&before);
    phases.push(Phase{name,before,after,delta});
    value
}
fn ensure(ok: bool, why: &str) -> Result<(), String> {
    if ok {Ok(())} else {Err(why.to_owned())}
}

fn fp_add(m:&mut Meter,a:u64,b:u64)->u64 {m.c.fp_additions+=1;(a+b)%P}
fn fp_sub(m:&mut Meter,a:u64,b:u64)->u64 {m.c.fp_subtractions+=1;(a+P-b)%P}
fn fp_mul(m:&mut Meter,a:u64,b:u64)->u64 {m.c.fp_modular_products_including_squares+=1;((a as u128*b as u128)%P as u128)as u64}
fn fp_pow(m:&mut Meter,mut a:u64,mut k:u64)->u64 {
    m.c.fp_exponentiation_calls+=1;
    let mut out=1;
    while k!=0 {if k&1!=0 {out=fp_mul(m,out,a);} k>>=1;a=fp_mul(m,a,a);}
    out
}
fn fp_inv(m:&mut Meter,a:u64)->u64 {assert!(a>0&&a<P);m.c.fp_inversion_calls+=1;fp_pow(m,a,P-2)}
fn fr_add(m:&mut Meter,a:u64,b:u64)->u64 {m.c.fr_additions+=1;(a+b)%R}
fn fr_sub(m:&mut Meter,a:u64,b:u64)->u64 {m.c.fr_subtractions+=1;(a+R-b)%R}
fn fr_mul(m:&mut Meter,a:u64,b:u64)->u64 {m.c.fr_modular_products_including_squares+=1;((a as u128*b as u128)%R as u128)as u64}
fn fr_pow(m:&mut Meter,mut a:u64,mut k:u64)->u64 {
    m.c.fr_exponentiation_calls+=1;
    let mut out=1;
    while k!=0 {if k&1!=0 {out=fr_mul(m,out,a);}k>>=1;a=fr_mul(m,a,a);}
    out
}
fn fr_inv(m:&mut Meter,a:u64)->u64 {assert!(a>0&&a<R);m.c.fr_inversion_calls+=1;fr_pow(m,a,R-2)}
fn signed(m:&mut Meter,a:u64,s:i8)->u64 {assert!(s==1||s== -1);if s==1 {a}else{fr_sub(m,0,a)}}
fn trim(mut a:Poly)->Poly {while a.len()>1&&a.last()==Some(&0){a.pop();}if a.is_empty(){a.push(0);}a}
fn zero(a:&[u64])->bool {a==[0]}
fn pmul(m:&mut Meter,a:&[u64],b:&[u64])->Poly {
    m.c.polynomial_multiply_calls+=1;
    let mut out=vec![0;a.len()+b.len()-1];
    for(i,&x)in a.iter().enumerate(){for(j,&y)in b.iter().enumerate(){
        m.c.polynomial_coefficient_products+=1;
        let product=fp_mul(m,x,y);out[i+j]=fp_add(m,out[i+j],product);
    }}trim(out)
}
fn psub(m:&mut Meter,a:&[u64],b:&[u64])->Poly {
    trim((0..a.len().max(b.len())).map(|i|fp_sub(m,*a.get(i).unwrap_or(&0),*b.get(i).unwrap_or(&0))).collect())
}
fn prem(m:&mut Meter,a:&[u64],b:&[u64])->Poly {
    m.c.polynomial_division_calls+=1;assert!(!zero(b));
    let mut out=trim(a.to_vec());let inv=fp_inv(m,*b.last().unwrap());
    while !zero(&out)&&out.len()>=b.len(){
        m.c.polynomial_division_elimination_steps+=1;
        let offset=out.len()-b.len();let q=fp_mul(m,*out.last().unwrap(),inv);
        for(j,&x)in b.iter().enumerate(){m.c.polynomial_division_coefficient_updates+=1;
            let product=fp_mul(m,q,x);out[offset+j]=fp_sub(m,out[offset+j],product);}
        out=trim(out);
    }out
}
fn pgcd(m:&mut Meter,mut a:Poly,mut b:Poly)->Poly {
    m.c.polynomial_gcd_calls+=1;
    while !zero(&b){let r=prem(m,&a,&b);a=b;b=r;}
    if zero(&a){return a;}
    let inv=fp_inv(m,*a.last().unwrap());a.into_iter().map(|x|fp_mul(m,x,inv)).collect()
}
fn ppowmod(m:&mut Meter,mut a:Poly,mut n:u64,modulus:&[u64])->Poly {
    m.c.polynomial_modular_exponentiation_calls+=1;
    let mut r=vec![1];a=prem(m,&a,modulus);
    while n!=0 {if n&1!=0{let product=pmul(m,&r,&a);r=prem(m,&product,modulus);}n>>=1;
        if n!=0{let product=pmul(m,&a,&a);a=prem(m,&product,modulus);}}
    r
}
fn peval(m:&mut Meter,a:&[u64],x:u64)->u64 {
    m.c.polynomial_evaluation_calls+=1;let mut out=0;
    for &coefficient in a.iter().rev(){m.c.polynomial_horner_steps+=1;
        let product=fp_mul(m,out,x);out=fp_add(m,product,coefficient);}out
}
fn terminal(m:&mut Meter,x:u64,depth:usize)->Result<u64,String>{
    ensure(x<P,"noncanonical field input")?;ensure((1..=10).contains(&depth),"unsupported depth")?;
    let mut v=x;for _ in 0..depth{m.c.chain_steps+=1;let square=fp_mul(m,v,v);v=fp_sub(m,square,2);}Ok(v)
}
fn selector_polynomial(m:&mut Meter,cs:&[u64])->Poly {
    let mut d=vec![0,1];for _ in 0..DEPTH{let square=pmul(m,&d,&d);d=psub(m,&square,&[2]);}
    let mut out=vec![1];for &c in cs{let fiber=psub(m,&d,&[c]);out=pmul(m,&out,&fiber);}out
}
fn root_polynomial(m:&mut Meter,roots:&[u64])->Poly {
    let mut out=vec![1];for &x in roots{let constant=fp_sub(m,0,x);out=pmul(m,&out,&[constant,1]);}out
}
fn support_gcd(m:&mut Meter,f:&[u64])->Poly {
    if f.len()==1{return vec![1];}
    let xp=ppowmod(m,vec![0,1],P,f);let difference=psub(m,&xp,&[0,1]);pgcd(m,f.to_vec(),difference)
}
fn inverse_fiber(m:&mut Meter,c:u64)->Vec<u64>{
    let mut current=BTreeSet::from([c]);
    for _ in 0..DEPTH {let mut next=BTreeSet::new();for y in current {
        m.c.inverse_fiber_nodes+=1;let radicand=fp_add(m,y,2);
        if let Some(x)=m.ar.sqrt(radicand,P){m.c.inverse_set_insert_attempts+=2;
            next.insert(x);let negative=fp_sub(m,0,x);next.insert(negative);}}
        current=next;
    }current.into_iter().collect()
}

#[derive(Clone,Debug)]
struct Tag{id:usize,point:(u64,u64),a:u64,b:u64,original:Labelled,orientation:i8}
fn canonical_tag(m:&mut Meter,id:usize,original:Labelled)->Tag{
    m.c.canonicalized_records+=1;let negative_y=fp_sub(m,0,original.pt.1);
    let orientation=if original.pt.1>negative_y{-1}else{1};
    Tag{id,point:(original.pt.0,original.pt.1.min(negative_y)),a:signed(m,original.a,orientation),
        b:signed(m,original.b,orientation),original,orientation}
}
fn validate_tag(m:&mut Meter,t:&Tag)->Result<(),String>{
    m.c.public_tag_validations+=1;
    ensure(t.point.0<P&&t.point.1<P&&t.a<R&&t.b<R,"noncanonical public tag")?;
    ensure(t.point.1<=P-t.point.1,"noncanonical point orientation")?;
    ensure(m.ar.on_curve(PRIME31,Some(t.point)),"tag point off curve")?;
    let ag=m.ar.scalar(PRIME31,t.a,Some(PRIME31.g));let bq=m.ar.scalar(PRIME31,t.b,Some(Q));
    ensure(m.ar.add(PRIME31,ag,bq)==Some(t.point),"public tag equation fails")?;
    let original_point=if t.orientation==1{Some(t.original.pt)}else{m.ar.neg(PRIME31,Some(t.original.pt))};
    ensure(original_point==Some(t.point),"original point identity differs")?;
    let a=signed(m,t.original.a,t.orientation);let b=signed(m,t.original.b,t.orientation);
    ensure(t.a==a&&t.b==b,"original public coefficients differ")
}
fn roster_find(m:&mut Meter,sorted:&[Tag],point:(u64,u64))->Option<usize>{
    m.c.roster_lookup_queries+=1;let(mut lo,mut hi)=(0,sorted.len());
    while lo<hi{let mid=lo+(hi-lo)/2;m.c.roster_point_comparisons+=1;
        match sorted[mid].point.cmp(&point){std::cmp::Ordering::Less=>lo=mid+1,
            std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Some(mid)}}None
}
fn binary_member(m:&mut Meter,keys:&[u64],x:u64)->bool{
    let(mut lo,mut hi)=(0,keys.len());while lo<hi{let mid=lo+(hi-lo)/2;m.c.membership_key_comparisons+=1;
        match keys[mid].cmp(&x){std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,
            std::cmp::Ordering::Equal=>return true}}false
}
fn member(m:&mut Meter,arm:char,x:u64,cs:&[u64],roots:&[u64])->Result<bool,String>{
    m.c.membership_queries+=1;ensure(x<P,"noncanonical membership input")?;
    match arm{'A'=>{let c=terminal(m,x,DEPTH)?;Ok(binary_member(m,cs,c))},
        'B'=>Ok(binary_member(m,roots,x)),_=>Err("unknown membership arm".to_owned())}
}
#[derive(Debug,PartialEq,Eq)]
struct Relation{alpha:u64,beta:u64,logarithm:u64}
fn replay_relation(m:&mut Meter,tags:&[Tag],signs:&[i8])->Result<Relation,String>{
    ensure(tags.len()==signs.len(),"relation arity")?;
    let(mut alpha,mut beta)=(0,0);let mut sum:Point=None;
    for(t,&s)in tags.iter().zip(signs){validate_tag(m,t)?;ensure(s==1||s== -1,"invalid relation sign")?;
        let point=if s==1{Some(t.point)}else{m.ar.neg(PRIME31,Some(t.point))};sum=m.ar.add(PRIME31,sum,point);
        let a=signed(m,t.a,s);let b=signed(m,t.b,s);alpha=fr_add(m,alpha,a);beta=fr_add(m,beta,b);}
    ensure(sum.is_none(),"signed point sum nonzero")?;ensure(beta!=0,"zero aggregate Q coefficient")?;
    let inverse=fr_inv(m,beta);let negative=fr_sub(m,0,alpha);let logarithm=fr_mul(m,negative,inverse);
    ensure(m.ar.scalar(PRIME31,logarithm,Some(PRIME31.g))==Some(Q),"final scalar replay fails")?;
    Ok(Relation{alpha,beta,logarithm})
}

struct MembershipPass{index:usize,arm:char,hits:usize,checksum:u64,counts:Snapshot}
struct Corpus{name:&'static str,queries:usize,passes:Vec<MembershipPass>,bitmap:Vec<bool>}
fn membership_schedule(name:&'static str,xs:&[u64],cs:&[u64],roots:&[u64])->Result<Corpus,String>{
    let mut passes=Vec::new();let mut expected:Option<Vec<bool>>=None;
    let mut a_counts:Option<Snapshot>=None;let mut b_counts:Option<Snapshot>=None;
    for(index,&arm)in SCHEDULE.iter().enumerate(){
        let mut m=Meter::default();let mut bitmap=Vec::with_capacity(xs.len());let mut checksum=0xcbf29ce484222325_u64;
        for &x in xs{let yes=member(&mut m,arm,x,cs,roots)?;bitmap.push(yes);
            checksum=(checksum^x).wrapping_mul(0x100000001b3);checksum=(checksum^u64::from(yes)).wrapping_mul(0x100000001b3);}
        if let Some(ref prior)=expected{ensure(*prior==bitmap,"membership arms disagree")?;}else{expected=Some(bitmap.clone());}
        let counts=m.snapshot();let prior=if arm=='A'{&mut a_counts}else{&mut b_counts};
        if let Some(ref first)=*prior{ensure(*first==counts,"same-arm counter determinism failed")?;}else{*prior=Some(counts.clone());}
        passes.push(MembershipPass{index,arm,hits:bitmap.iter().filter(|&&b|b).count(),checksum,counts});
    }
    Ok(Corpus{name,queries:xs.len(),passes,bitmap:expected.unwrap()})
}
fn frozen_array(name:&str)->Result<Vec<u64>,String>{
    let needle=format!("\"{name}\":");let start=FROZEN.find(&needle).ok_or_else(||format!("missing frozen field {name}"))?+needle.len();
    let rest=&FROZEN[start..];let opening=rest.find('[').ok_or("frozen array has no opening")?;
    let closing=rest[opening..].find(']').ok_or("frozen array has no closing")?+opening;
    rest[opening+1..closing].split(',').map(|x|x.trim().parse::<u64>().map_err(|e|e.to_string())).collect()
}

struct Report{
    phases:Vec<Phase>,common:Snapshot,cs:Vec<u64>,roots:Vec<u64>,rational:Vec<(u64,u64)>,
    h:Poly,gcd:Poly,tags:Vec<Tag>,signs:Vec<i8>,full_admitted:Vec<usize>,historical_admitted:Vec<usize>,
    source_x:Vec<u64>,source_identities:u64,source_duplicates:u64,relation:Relation,corpora:Vec<Corpus>,
    charged_a:Snapshot,charged_b:Snapshot,
}
fn run()->Result<Report,String>{
    let mut m=Meter::default();let mut phases=Vec::new();let initial=m.snapshot();
    phase(&mut m,&mut phases,"curve_admission",|m|{
        check_curve(PRIME31,&mut m.ar)?;
        ensure(m.ar.on_curve(PRIME31,Some(Q))&&m.ar.scalar(PRIME31,R,Some(Q)).is_none(),"target admission failed")?;
        let mut root=0_u64;while(root+1)*(root+1)<=P{root+=1;}
        ensure(root*root<=P&&(root+1)*(root+1)>P&&2*R>P+1+2*(root+1),"integer Hasse certificate failed")
    })?;
    let(parts,identities,duplicates)=phase(&mut m,&mut phases,"archived_source_generation",|m|{
        let value=archive_source::source(Q,SEED,&mut m.ar)?;
        m.c.source_records=value.0.iter().map(|x|x.len()as u64).sum();
        ensure(value.0.iter().all(|x|x.len()==1536)&&value.1==0&&value.2==0,"archived population differs")?;Ok(value)
    })?;
    let(pool,sorted_pool,source_x)=phase(&mut m,&mut phases,"canonical_full_pool_roster",|m|{
        let pool:Vec<_>=parts.iter().flatten().copied().enumerate().map(|(i,t)|canonical_tag(m,i,t)).collect();
        let xs=pool.iter().map(|t|t.point.0).collect::<Vec<_>>();let mut sorted=pool.clone();
        sorted.sort_unstable_by(|a,b|{m.c.roster_sort_key_comparisons+=1;a.point.cmp(&b.point)});
        ensure(sorted.windows(2).all(|w|w[0].point.0!=w[1].point.0),"duplicate source x")?;Ok((pool,sorted,xs))
    })?;
    let(tags,signs,sorted_history)=phase(&mut m,&mut phases,"historical_columns_and_public_tags",|m|{
        let tags:Vec<_>=IDS.iter().map(|&id|pool[id].clone()).collect();
        let signs:Vec<_>=tags.iter().zip(ORIGINAL_SIGNS).map(|(t,s)|s*t.orientation).collect();
        for t in &tags{validate_tag(m,t)?;}
        let mut sorted=tags.clone();sorted.sort_unstable_by(|a,b|{m.c.roster_sort_key_comparisons+=1;a.point.cmp(&b.point)});
        Ok((tags,signs,sorted))
    })?;
    let cs=phase(&mut m,&mut phases,"terminal_fixture_construction",|m|{
        let base=Base::build(PRIME31,DEPTH,&mut m.ar)?;
        ensure(base.values.len()==1<<DEPTH,"native complete fiber differs")?;
        let mut raw=Vec::new();for t in &tags{raw.push(terminal(m,t.point.0,DEPTH)?);}
        raw.extend([base.terminal,2,P-2,base.terminal]);
        let cs:Vec<_>=raw.into_iter().collect::<BTreeSet<_>>().into_iter().collect();
        ensure(cs==frozen_array("terminals")?,"six-terminal fixture differs")?;Ok(cs)
    })?;
    let roots=phase(&mut m,&mut phases,"recursive_root_recovery",|m|{
        let mut all=BTreeSet::new();for &c in &cs{for x in inverse_fiber(m,c){m.c.union_set_insert_attempts+=1;all.insert(x);}}
        let roots:Vec<_>=all.into_iter().collect();ensure(roots==frozen_array("selector_roots")?,"root support differs")?;Ok(roots)
    })?;
    let(h,gcd)=phase(&mut m,&mut phases,"exact_support_certificate",|m|{
        let h=selector_polynomial(m,&cs);let gcd=support_gcd(m,&h);let product=root_polynomial(m,&roots);
        ensure(gcd==product,"gcd does not equal distinct-root product")?;
        for &x in &roots{ensure(member(m,'A',x,&cs,&roots)?&&peval(m,&h,x)==0,"root membership failed")?;}
        ensure(h==frozen_array("shared_selector_coefficients_low_to_high")?&&gcd==frozen_array("gcd_with_Xp_minus_X_coefficients_low_to_high")?,"frozen coefficients differ")?;
        Ok((h,gcd))
    })?;
    let rational=phase(&mut m,&mut phases,"rational_curve_lifts",|m|{
        let mut out=Vec::new();for &x in &roots{let rhs=m.ar.rhs(PRIME31,x);if let Some(y)=m.ar.sqrt(rhs,P){
            let ny=fp_sub(m,0,y);let point=(x,y.min(ny));
            ensure(m.ar.on_curve(PRIME31,Some(point))&&m.ar.on_curve(PRIME31,Some((x,y.max(ny)))),"invalid rational lift")?;
            out.push(point);}}
        ensure(out.iter().map(|p|p.0).collect::<Vec<_>>()==frozen_array("rational_curve_abscissae")?,"rational support differs")?;Ok(out)
    })?;
    let full_admitted=phase(&mut m,&mut phases,"full_4608_pool_label_admission",|m|{
        let mut out=Vec::new();for &point in &rational{if let Some(index)=roster_find(m,&sorted_pool,point){
            let t=&sorted_pool[index];validate_tag(m,t)?;out.push(t.id);}}
        out.sort_unstable();Ok(out)
    })?;
    let historical_admitted=phase(&mut m,&mut phases,"historical_three_column_admission",|m|{
        let mut out=Vec::new();for &point in &rational{if let Some(index)=roster_find(m,&sorted_history,point){out.push(sorted_history[index].id);}}
        out.sort_unstable();ensure(out==IDS,"archived roster admission differs")?;Ok(out)
    })?;
    let relation=phase(&mut m,&mut phases,"archived_relation_replay",|m|{
        ensure(IDS.iter().all(|id|full_admitted.binary_search(id).is_ok()),"historical row absent from full pool admissions")?;
        let relation=replay_relation(m,&tags,&signs)?;
        ensure(relation==Relation{alpha:1_144_570_523,beta:2_011_873_483,logarithm:1_038_579_171},"archived relation output differs")?;Ok(relation)
    })?;
    let common=m.snapshot();let mut phase_sum=initial.clone();for p in &phases{phase_sum=phase_sum.plus(&p.delta);}
    ensure(phase_sum==common,"disjoint phase sum differs from cumulative counter snapshot")?;
    let group_parts=common.get("native.identity_add_calls")+common.get("native.inverse_pair_add_calls")+common.get("native.double_calls")+common.get("native.general_add_calls");
    ensure(group_parts==common.get("native.group_add_calls"),"group call categories do not partition calls")?;
    let source_corpus=membership_schedule("ordered_archived_4608_x_stream",&source_x,&cs,&roots)?;
    let admitted_from_queries:Vec<_>=source_corpus.bitmap.iter().enumerate().filter_map(|(id,&yes)|yes.then_some(id)).collect();
    ensure(admitted_from_queries==full_admitted,"full source membership and root-to-label admission disagree")?;
    let positive_corpus=membership_schedule("all_29_positive_selector_roots",&roots,&cs,&roots)?;
    ensure(positive_corpus.bitmap.iter().all(|&b|b),"positive corpus has a rejected root")?;
    let a=&source_corpus.passes.iter().find(|p|p.arm=='A').unwrap().counts;
    let b=&source_corpus.passes.iter().find(|p|p.arm=='B').unwrap().counts;
    let charged_a=common.plus(a);let charged_b=common.plus(b);
    Ok(Report{phases,common,cs,roots,rational,h,gcd,tags,signs,full_admitted,historical_admitted,source_x,
        source_identities:identities,source_duplicates:duplicates,relation,corpora:vec![source_corpus,positive_corpus],charged_a,charged_b})
}

fn fp_products(s:&Snapshot)->u64{
    s.get("native.fp_modular_products_including_squares").checked_add(s.get("custom.fp_modular_products_including_squares")).expect("Fp product count overflow")
}
fn fr_products(s:&Snapshot)->u64{
    s.get("native.fr_modular_products_including_squares").checked_add(s.get("custom.fr_modular_products_including_squares")).expect("Fr product count overflow")
}
fn presentation_counts(s:&Snapshot)->String{
    format!("{{\"fp_modular_products_including_squares\":{},\"fr_modular_products_including_squares\":{},\"group_add_calls\":{},\"point_scalar_calls\":{}}}",
        fp_products(s),fr_products(s),s.get("native.group_add_calls"),s.get("native.point_scalar_calls"))
}
fn presentation_summary(r:&Report)->String{
    let phases=r.phases.iter().map(|p|format!("{{\"phase\":\"{}\",\"counts\":{}}}",p.name,presentation_counts(&p.delta))).collect::<Vec<_>>().join(",");
    let prepared=r.corpora.iter().map(|corpus|{
        let arm_json=|arm:char|{
            let pass=corpus.passes.iter().find(|p|p.arm==arm).expect("missing presentation arm");
            format!("{{\"counts\":{},\"membership_queries\":{},\"three_way_key_comparisons\":{},\"key_payload_bytes\":{}}}",
                presentation_counts(&pass.counts),pass.counts.get("custom.membership_queries"),
                pass.counts.get("custom.membership_key_comparisons"),if arm=='A'{r.cs.len()*8}else{r.roots.len()*8})
        };
        format!("{{\"corpus\":\"{}\",\"query_count\":{},\"A\":{},\"B\":{}}}",corpus.name,corpus.queries,arm_json('A'),arm_json('B'))
    }).collect::<Vec<_>>().join(",");
    let source=r.phases.iter().find(|p|p.name=="archived_source_generation").expect("source phase absent");
    let numerator=fp_products(&source.delta);let denominator=fp_products(&r.common);
    format!(r#"{{"unit_rule":"native and custom modular-product helper counts are added only within the same field; Fp, Fr, group calls and EC scalar calls remain separate",
"phases":[{phases}],"common_counted_path":{common},
"charged_common_plus_one_full_stream_pass":{{"A":{a},"B":{b}}},
"prepared_query_corpora":[{prepared}],
"source_phase_share_of_common_fp_products":{{"numerator":{numerator},"denominator":{denominator},"form":"exact unreduced rational; no floating-point ratio","numerator_phase":"archived_source_generation","denominator_scope":"common counted path, excluding membership calibration passes"}},
"structural_and_overlapping_counters":"retained separately in the full ledger; inversion/sqrt/chain/polynomial structural counters are not added to product totals"}}"#,
        common=presentation_counts(&r.common),a=presentation_counts(&r.charged_a),b=presentation_counts(&r.charged_b))
}
fn report_json(r:&Report)->String{
    let phases=r.phases.iter().map(|p|format!("{{\"phase\":\"{}\",\"before\":{},\"after\":{},\"delta\":{}}}",p.name,p.before.json(),p.after.json(),p.delta.json())).collect::<Vec<_>>().join(",");
    let corpora=r.corpora.iter().map(|c|{let passes=c.passes.iter().map(|p|format!("{{\"pass\":{},\"arm\":\"{}\",\"hits\":{},\"checksum_fnv1a_style_u64\":{},\"counts\":{}}}",p.index,p.arm,p.hits,p.checksum,p.counts.json())).collect::<Vec<_>>().join(",");
        format!("{{\"corpus\":\"{}\",\"query_count\":{},\"all_pass_bitmaps_equal\":true,\"same_arm_counts_equal\":true,\"passes\":[{}]}}",c.name,c.queries,passes)}).collect::<Vec<_>>().join(",");
    let tags=r.tags.iter().map(|t|format!("{{\"id\":{},\"point\":[{},{}],\"a\":{},\"b\":{},\"original_point\":[{},{}],\"original_a\":{},\"original_b\":{},\"orientation\":{}}}",t.id,t.point.0,t.point.1,t.a,t.b,t.original.pt.0,t.original.pt.1,t.original.a,t.original.b,t.orientation)).collect::<Vec<_>>().join(",");
    let rational_x:Vec<_>=r.rational.iter().map(|p|p.0).collect();
    let signed_lifts=r.rational.iter().map(|p|if p.1==0{1}else{2}).sum::<usize>();
    let presentation=presentation_summary(r);
    format!(r#"{{
"schema":"isogeny.m31-shared-selector-operation-census/v1",
"status":"pass",
"scope":"one fixed M31 construction, complete-support certificate, label admission and archived relation replay",
"p":{P},"r":{R},"curve":[847514846,2087444724],"G":[625037715,2100627251],"Q":[1477799919,824532935],
"clock_calls":0,"wall_time_claim":false,"rho_ratio":null,"new_relation_search_calls":0,"new_relation_extractor":"not implemented","time_to_new_relation":null,
"source_provenance":{{"native_commit":"b60f801660eb423901ddea0c03c629089a1b5bfc","archive_commit":"90a0314f5be63d5ca56465a884091f4be0f114fd","archive_seed_hex":"4150535f50523331","dedup_hasher":"BuildHasherDefault<DefaultHasher>; Rust 1.100.0; deterministic DefaultHasher::new keys","archived_indices_and_scalar":"verifier fixture, not newly discovered and not a blind target"}},
"counter_semantics":{{"units":"heterogeneous named operation counts; no summed cost or GAE conversion","products":"modular multiplication helper calls including squares; exponentiation products included","inversions":"calls overlap their underlying modular products; do not add them as disjoint costs","group_calls":"native add calls, partitioned into identity, inverse-pair, doubling and general cases; not GAE","point_scalar_calls":"EC scalar multiplication calls, distinct from Fr products","comparisons":"one three-way u64 key or point-tuple comparator invocation","polynomial_counts":"coefficient products overlap custom Fp products; division steps and coefficient updates are structural counts","unmeasured":"hash bucket probes, tree comparisons, allocation, copying, primality integer trials, native direct integer add/subtract and small-constant products, bookkeeping, frozen-value comparisons, output formatting and I/O","validation_overhead":"native public-tag and curve checks are included in their named phases; snapshot/assertion bookkeeping is not priced"}},
"cold_phase_ledger":[{phases}],
"common_counted_path_total":{common},
"phase_delta_sum_matches_total":true,"group_add_categories_partition_calls":true,
"selector":{{"depth":3,"terminals":{terminals:?},"roots":{roots:?},"rational_abscissae":{rational_x:?},"signed_curve_lifts":{signed_lifts},"selector_coefficients":{h:?},"support_gcd_coefficients":{gcd:?},"exact_support_equals_frozen":true}},
"source_pool":{{"records":4608,"identity_rejections":{identities},"duplicate_x_rejections":{duplicates},"ordered_query_x":{source_x:?}}},
"label_admission":{{"rational_abscissae_tested":{rational_count},"full_pool_columns":4608,"full_pool_admitted_ids":{full_admitted:?},"full_pool_admitted_count":{full_count},"full_pool_rejected_rational_abscissae":{full_rejected},"historical_roster_columns":3,"historical_roster_admitted_ids":{historical_admitted:?},"historical_roster_rejected_rational_abscissae":{historical_rejected}}},
"historical_tags":[{tags}],"historical_canonical_signs":{signs:?},"historical_relation":{{"alpha":{alpha},"beta":{beta},"replayed_scalar":{logarithm},"matches_archived_output":true}},
"membership_protocol":{{"A":"D8 chain once, then binary search six terminals","B":"binary search explicit 29-root support","schedule":["A","A","A","B","B","A","A","B","B","A","A","B"],"schedule_meaning":"two A/A controls then AB,BA,AB,BA,AB pairs","passes_per_corpus":12,"A_passes":7,"B_passes":5,"yield_replicates":false,"paired_field_cost_ratio":null}},
"membership_corpora":[{corpora}],
"charged_totals":{{"meaning":"computed common counted path plus exactly one full-4608-query pass per arm; calibration repetitions and all-positive corpus excluded","A":{charged_a},"B":{charged_b}}},
"representation_payload":{{"unit":"bytes of stored u64 keys only","A_terminal_keys":{a_bytes},"B_root_keys":{b_bytes},"common_modulus_and_depth_excluded":true,"vector_headers_and_spare_capacity_excluded":true,"heap_bytes":null,"rss_bytes":null}},
"presentation_summary":{presentation},
"standing":"producer-run counted fixture; no independent PCC verdict or new relation search"
}}"#,
        common=r.common.json(),terminals=r.cs,roots=r.roots,h=r.h,gcd=r.gcd,
        identities=r.source_identities,duplicates=r.source_duplicates,source_x=r.source_x,
        rational_count=r.rational.len(),full_admitted=r.full_admitted,full_count=r.full_admitted.len(),
        full_rejected=r.rational.len()-r.full_admitted.len(),historical_admitted=r.historical_admitted,
        historical_rejected=r.rational.len()-r.historical_admitted.len(),signs=r.signs,
        alpha=r.relation.alpha,beta=r.relation.beta,logarithm=r.relation.logarithm,
        charged_a=r.charged_a.json(),charged_b=r.charged_b.json(),a_bytes=r.cs.len()*8,b_bytes=r.roots.len()*8)
}
fn main(){
    if std::env::args_os().len()!=1 {eprintln!("This frozen census takes no arguments");std::process::exit(2);}
    match run(){Ok(r)=>println!("{}",report_json(&r)),Err(e)=>{eprintln!("operation census failed: {e}");std::process::exit(1);}}
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn chain_depth_three_has_exact_multiply_and_subtract_counts(){
        let mut m=Meter::default();assert_eq!(terminal(&mut m,3,3).unwrap(),2207);
        assert_eq!(m.c.fp_modular_products_including_squares,3);assert_eq!(m.c.fp_subtractions,3);assert_eq!(m.c.chain_steps,3);
        assert!(terminal(&mut m,P,3).is_err());
    }
    #[test]
    fn native_sqrt_has_32_modular_products_including_check(){
        let mut m=Meter::default();assert!(m.ar.sqrt(4,P).is_some());
        assert_eq!(m.ar.counts.fp_multiplications,32);assert_eq!(m.ar.counts.sqrt_attempts,1);assert_eq!(m.ar.counts.sqrt_successes,1);
    }
    #[test]
    fn scalar_inverse_is_counted_in_fr_not_fp(){
        let mut m=Meter::default();let inverse=fr_inv(&mut m,7);assert_eq!((inverse as u128*7%R as u128)as u64,1);
        assert_eq!(m.c.fr_inversion_calls,1);assert!(m.c.fr_modular_products_including_squares>0);
        assert_eq!(m.c.fp_modular_products_including_squares,0);assert_eq!(m.ar.counts.point_scalar_calls,0);
    }
    #[test]
    fn snapshot_phase_sum_is_disjoint(){
        let mut m=Meter::default();let mut phases=Vec::new();
        phase(&mut m,&mut phases,"first",|m|{terminal(m,3,3)?;Ok(())}).unwrap();
        phase(&mut m,&mut phases,"second",|m|{let _=fr_inv(m,7);Ok(())}).unwrap();
        assert_eq!(phases[0].delta.plus(&phases[1].delta),m.snapshot());assert_eq!(phases[0].after,phases[1].before);
    }
    #[test]
    fn membership_equivalence_on_complete_support_and_fixed_m31_inputs(){
        let cs=frozen_array("terminals").unwrap();let roots=frozen_array("selector_roots").unwrap();
        let mut xs=roots.clone();xs.extend(0..256);xs.extend([P-1,P-2]);
        let corpus=membership_schedule("test",&xs,&cs,&roots).unwrap();assert_eq!(corpus.passes.len(),12);
        assert_eq!(corpus.passes.iter().filter(|p|p.arm=='A').count(),7);
        let mut m=Meter::default();assert!(member(&mut m,'B',P,&cs,&roots).is_err());
    }
    #[test]
    fn complete_counted_fixture_matches_frozen_outputs_and_phase_accounting(){
        let r=run().unwrap();assert_eq!(r.source_x.len(),4608);assert_eq!(r.roots.len(),29);assert_eq!(r.rational.len(),17);
        assert_eq!(r.historical_admitted,IDS);assert_eq!(r.relation.logarithm,1_038_579_171);
        assert_eq!(r.common.get("custom.fr_inversion_calls"),1);
        assert_eq!(r.corpora[0].passes[0].counts.get("custom.fp_modular_products_including_squares"),4608*3);
        assert_eq!(r.corpora[0].passes[3].counts.get("custom.fp_modular_products_including_squares"),0);
        assert_eq!(r.charged_a,r.common.plus(&r.corpora[0].passes[0].counts));
        assert!(report_json(&r).contains("\"new_relation_search_calls\":0"));
    }
}
