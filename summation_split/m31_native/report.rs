//! Native, dependency-free rendering of the component report and its SVG.
//! This renderer formats recorded facts; it performs no research experiment.
//! Run from this directory: rustc --edition 2021 report.rs -o /tmp/m31-report
//!                         /tmp/m31-report .
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

const F4_NOTE: &str = "Bounded native quotient F4 completes one m=3 geometric sequential case: 561/561 pairs, 10 matrices, maximum raw product and normalized matrix-column degree 6. A separate ordinary Buchberger checker validates the resulting ideal.";
const PR: &str = "https://github.com/isogeny-labs/crypto/pull/5087";
const DIEM: &str = "https://doi.org/10.1112/S0010437X10005075";
const DEGREE: &str = "https://arxiv.org/abs/2112.05579";
const KY: &str = "https://arxiv.org/abs/1503.08001";

type Color = (f64,f64,f64);
const INK: Color = (0.08,0.16,0.24);
const MUTED: Color = (0.34,0.42,0.48);
const TEAL: Color = (0.03,0.43,0.43);
const BLUE: Color = (0.15,0.32,0.55);
const PALE: Color = (0.93,0.96,0.97);
const LIGHT: Color = (0.88,0.93,0.94);
const WHITE: Color = (1.0,1.0,1.0);

#[derive(Clone)]
enum Draw {
    Text(f64,f64,f64,u8,Color,String),
    Rect(f64,f64,f64,f64,Color),
    Line(f64,f64,f64,f64,f64,Color),
}
struct Link { x:f64,y:f64,w:f64,h:f64,url:String }
struct Canvas { w:f64,h:f64,items:Vec<Draw>,links:Vec<Link> }
impl Canvas {
    fn new(w:f64,h:f64)->Self {Self{w,h,items:vec![],links:vec![]}}
    fn text(&mut self,x:f64,y:f64,size:f64,font:u8,color:Color,s:&str) {
        assert!(s.is_ascii(),"PDF text uses standard ASCII fonts");
        self.items.push(Draw::Text(x,y,size,font,color,s.into()));
    }
    fn rect(&mut self,x:f64,y:f64,w:f64,h:f64,c:Color){self.items.push(Draw::Rect(x,y,w,h,c));}
    fn line(&mut self,x:f64,y:f64,xx:f64,yy:f64,width:f64,c:Color){self.items.push(Draw::Line(x,y,xx,yy,width,c));}
    fn paragraph(&mut self,x:f64,mut y:f64,w:f64,size:f64,font:u8,c:Color,s:&str)->f64 {
        let limit = (w/(size*0.53)) as usize;
        let mut line=String::new();
        for word in s.split_whitespace() {
            if !line.is_empty() && line.len()+word.len()+1>limit {
                self.text(x,y,size,font,c,&line);y+=size*1.45;line.clear();
            }
            if !line.is_empty(){line.push(' ');}line.push_str(word);
        }
        if !line.is_empty(){self.text(x,y,size,font,c,&line);y+=size*1.45;}
        y
    }
    fn link(&mut self,x:f64,y:f64,w:f64,h:f64,url:&str) {
        self.links.push(Link{x,y,w,h,url:url.into()});
    }
    fn heading(&mut self,num:usize,kicker:&str,title:&str,subtitle:&str) {
        self.rect(0.,0.,self.w,8.,TEAL);
        self.text(42.,38.,9.,2,TEAL,kicker);
        self.text(42.,72.,25.,2,INK,title);
        self.paragraph(42.,94.,528.,10.,1,MUTED,subtitle);
        self.line(42.,742.,570.,742.,0.7,LIGHT);
        self.text(42.,760.,8.,1,MUTED,"ISOGENY LABS  |  M31 PROJECTIVE CONSTRAINTS  |  2026-11-23 PACIFIC");
        self.text(42.,773.,7.5,1,MUTED,"Local development evidence; unsigned PCC standing. Source-linked component report.");
        self.text(547.,760.,9.,2,TEAL,&format!("{num}/4"));
    }
    fn section(&mut self,y:f64,title:&str) {self.text(42.,y,13.,2,INK,title);}
    fn source(&mut self,y:f64,label:&str,url:&str) {
        self.text(42.,y,8.5,1,BLUE,label);self.link(42.,y-9.,528.,12.,url);
    }
}

fn quote(s:&str)->String {s.replace('\\',"\\\\").replace('(',"\\(").replace(')',"\\)")}
fn xml(s:&str)->String {s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;")}
fn rgb(c:Color)->String {format!("#{:02x}{:02x}{:02x}",(c.0*255.) as u8,(c.1*255.) as u8,(c.2*255.) as u8)}
fn pdf_content(c:&Canvas)->String {
    let mut s=String::new();
    for d in &c.items {
        match d {
            Draw::Text(x,y,z,f,col,t)=>{writeln!(s,"BT /F{f} {z:.2} Tf {:.3} {:.3} {:.3} rg 1 0 0 1 {x:.2} {:.2} Tm ({}) Tj ET",col.0,col.1,col.2,c.h-y,quote(t)).unwrap();},
            Draw::Rect(x,y,w,h,col)=>{writeln!(s,"{:.3} {:.3} {:.3} rg {x:.2} {:.2} {w:.2} {h:.2} re f",col.0,col.1,col.2,c.h-y-h).unwrap();},
            Draw::Line(x,y,xx,yy,w,col)=>{writeln!(s,"{:.3} {:.3} {:.3} RG {w:.2} w {x:.2} {:.2} m {xx:.2} {:.2} l S",col.0,col.1,col.2,c.h-y,c.h-yy).unwrap();},
        }
    }
    s
}
fn write_pdf(path:&Path,pages:&[Canvas]) {
    let mut objects=vec![
        "<< /Type /Catalog /Pages 2 0 R >>".into(),
        format!("<< /Type /Pages /Count {} /Kids [{}] >>",pages.len(),(0..pages.len()).map(|i|format!("{} 0 R",6+2*i)).collect::<Vec<_>>().join(" ")),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".into(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold >>".into(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>".into(),
    ];
    let mut link_id=6+2*pages.len();
    for (i,p) in pages.iter().enumerate() {
        let annotations=(0..p.links.len()).map(|j|format!("{} 0 R",link_id+j)).collect::<Vec<_>>().join(" ");
        link_id+=p.links.len();
        objects.push(format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 3 0 R /F2 4 0 R /F3 5 0 R >> >> /Contents {} 0 R /Annots [{}] >>",p.w,p.h,7+2*i,annotations));
        let content=pdf_content(p);
        objects.push(format!("<< /Length {} >>\nstream\n{}endstream",content.len(),content));
    }
    for p in pages {for l in &p.links {
        objects.push(format!("<< /Type /Annot /Subtype /Link /Rect [{} {} {} {}] /Border [0 0 0] /A << /S /URI /URI ({}) >> >>",l.x,p.h-l.y-l.h,l.x+l.w,p.h-l.y,quote(&l.url)));
    }}
    let mut out=b"%PDF-1.4\n%Native vector report\n".to_vec();
    let mut offsets=vec![0usize];
    for (i,object) in objects.iter().enumerate() {
        offsets.push(out.len());out.extend_from_slice(format!("{} 0 obj\n{}\nendobj\n",i+1,object).as_bytes());
    }
    let xref=out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n",objects.len()+1).as_bytes());
    for &offset in &offsets[1..]{out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());}
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",objects.len()+1).as_bytes());
    fs::write(path,out).unwrap();
}
fn write_svg(path:&Path,c:&Canvas) {
    let mut s=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\"><title>M31 projective quotient and guarded multiplicity</title><rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n",c.w,c.h,c.w,c.h);
    for d in &c.items {match d {
        Draw::Text(x,y,z,f,col,t)=>{writeln!(s,"<text x=\"{x}\" y=\"{y}\" font-size=\"{z}\" font-family=\"{}\" font-weight=\"{}\" fill=\"{}\">{}</text>",if *f==3 {"monospace"} else {"Arial,Helvetica,sans-serif"},if *f==2 {"700"}else{"400"},rgb(*col),xml(t)).unwrap();},
        Draw::Rect(x,y,w,h,col)=>{writeln!(s,"<rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\" fill=\"{}\"/>",rgb(*col)).unwrap();},
        Draw::Line(x,y,xx,yy,w,col)=>{writeln!(s,"<line x1=\"{x}\" y1=\"{y}\" x2=\"{xx}\" y2=\"{yy}\" stroke-width=\"{w}\" stroke=\"{}\"/>",rgb(*col)).unwrap();},
    }}s.push_str("</svg>\n");fs::write(path,s).unwrap();
}

fn arrow(c:&mut Canvas,x:f64,y:f64,xx:f64,yy:f64,col:Color) {
    c.line(x,y,xx,yy,1.6,col);
    if (x-xx).abs()<1.0 {c.line(xx,yy,xx-4.,yy-7.,1.6,col);c.line(xx,yy,xx+4.,yy-7.,1.6,col);}
    else {c.line(xx,yy,xx-7.,yy-4.,1.6,col);c.line(xx,yy,xx-7.,yy+4.,1.6,col);}
}
fn quotient_diagram(c:&mut Canvas,x:f64,y:f64,w:f64) {
    c.rect(x,y,w,57.,INK);
    c.text(x+16.,y+21.,11.,2,WHITE,"ONE NORMALIZED PROJECTIVE NODE");
    c.text(x+16.,y+42.,10.,3,WHITE,"J = (z^2-z, zw-w, zs-s, w^2-s)");
    let half=(w-24.)/2.;let mid=x+w/2.;
    c.line(mid,y+57.,mid,y+76.,1.5,TEAL);
    c.line(x+half/2.,y+76.,x+w-half/2.,y+76.,1.5,TEAL);
    arrow(c,x+half/2.,y+76.,x+half/2.,y+95.,TEAL);
    arrow(c,x+w-half/2.,y+76.,x+w-half/2.,y+95.,TEAL);
    c.rect(x,y+99.,half,132.,PALE);c.rect(x+half+24.,y+99.,half,132.,PALE);
    c.text(x+16.,y+124.,13.,2,TEAL,"INFINITY COMPONENT");
    c.text(x+16.,y+148.,10.5,3,INK,"z = 0, w = 0, s = 0");
    c.text(x+16.,y+170.,10.5,3,INK,"(A,B,C) = (1,0,0)");
    c.paragraph(x+16.,y+195.,half-30.,10.,1,MUTED,"A single point retains identity and cancellation branches.");
    let r=x+half+24.;
    c.text(r+16.,y+124.,13.,2,TEAL,"AFFINE COMPONENT");
    c.text(r+16.,y+148.,10.5,3,INK,"z = 1, s = w^2");
    c.text(r+16.,y+170.,10.5,3,INK,"(A,B,C) = (w^2,w,1)");
    c.paragraph(r+16.,y+195.,half-30.,10.,1,MUTED,"One affine line. Its coordinate w may be geometric.");
    c.text(x+12.,y+257.,11.,3,INK,"Fp[z,w,s]/J  =  Fp  x  Fp[w]");
    c.text(x+12.,y+278.,9.5,1,MUTED,"The product is a decomposition of coordinate rings, not field descent.");
}
fn sector_diagram(c:&mut Canvas,x:f64,y:f64,w:f64) {
    c.rect(x,y,w,47.,INK);
    c.text(x+14.,y+20.,11.,2,WHITE,"CLASSIFY ALL ORIGINAL LEAVES + TARGET");
    c.text(x+14.,y+38.,9.5,3,WHITE,"e_i = f(x_i)^((p-1)/2)  in {0,+1,-1}");
    let h=(w-24.)/2.;let right=x+h+24.;let mid=x+w/2.;
    c.line(mid,y+47.,mid,y+66.,1.5,TEAL);
    c.line(x+h/2.,y+66.,right+h/2.,y+66.,1.5,TEAL);
    arrow(c,x+h/2.,y+66.,x+h/2.,y+86.,TEAL);
    arrow(c,right+h/2.,y+66.,right+h/2.,y+86.,TEAL);
    c.rect(x,y+90.,h,132.,PALE);c.rect(right,y+90.,h,132.,(0.89,0.95,0.93));
    c.text(x+14.,y+114.,12.,2,INK,"PURE OR ZERO-ORDINATE");
    c.text(x+14.,y+138.,11.,3,INK,"G = 0");
    c.paragraph(x+14.,y+163.,h-28.,10.,1,MUTED,"Added derivative equations are inactive. Simple roots remain allowed.");
    c.text(right+14.,y+114.,12.,2,TEAL,"NONZERO MIXED SECTORS");
    c.text(right+14.,y+138.,11.,3,INK,"G = 1");
    c.paragraph(right+14.,y+163.,h-28.,10.,1,MUTED,"At a Semaev zero, every coordinate partial derivative vanishes.");
    c.rect(x,y+244.,w,46.,INK);
    c.text(x+14.,y+264.,10.,3,WHITE,"append NF(G*dS/dt), NF(G*dS/du)");
    c.text(x+14.,y+282.,9.,1,WHITE,"Differentiate the unreduced S first; then reduce in the common section.");
}

fn pages()->Vec<Canvas> {
    let mut p=Canvas::new(612.,792.);
    p.heading(1,"RESEARCH COMPONENT / PR #5087","M31 constraints, made testable","Preclosed projective tensors and a guarded multiplicity branch for the next matched solver comparison.");
    p.rect(42.,137.,528.,59.,PALE);
    p.text(56.,159.,12.,2,TEAL,"THE CLAIM AT THIS STAGE");
    p.paragraph(56.,178.,500.,10.,1,INK,"Exact native construction and local correctness evidence. Actual solver runtime, peak memory and a 50-case projected F4 panel remain unmeasured.");
    for (x,n,title,line) in [(42.,"500","SPECIALIZED FIBERS","Complete roots + multiplicities"),(222.,"792","TENSOR CERTIFICATES","Every generated equation checked"),(402.,"400","MONOLITHIC VALUES","Exact resultant-value agreement")] {
        p.rect(x,215.,168.,95.,INK);p.text(x+13.,250.,28.,2,WHITE,n);
        p.text(x+13.,272.,8.5,2,(0.60,0.85,0.83),title);
        p.paragraph(x+13.,290.,143.,8.5,1,WHITE,line);
    }
    p.section(343.,"Frozen mathematical scope");
    p.text(42.,367.,11.,3,INK,"p = 2^31 - 1 = 2147483647");
    p.text(42.,387.,11.,3,INK,"E: y^2 = x^3 - 3x + 21");
    p.paragraph(42.,412.,528.,10.,1,INK,"Original coordinates and projected outputs are in M31. Fp2 is used only for geometric witnesses. The matched two-parameter diagnostic section is t(t-1)=u(u-1)=0: four M31 points, with the factor-base map unchanged.");
    p.section(477.,"What the new implementation supplies");
    p.paragraph(42.,499.,528.,10.,1,INK,"A canonical local quotient; complete sequential and balanced tensor generators; exact guarded derivatives; immutable solver-input export; an output-projection checker; independent signed-point and fixed-resultant correctness paths.");
    p.paragraph(42.,551.,528.,9.5,1,MUTED,F4_NOTE);
    p.section(614.,"Historical Horner counts remain historical");
    p.text(42.,637.,11.,3,INK,"m=5 generation: 8,424,950 -> 6,420,200 multiplications");
    p.paragraph(42.,659.,528.,9.5,1,MUTED,"With the common root-extraction GCD included, recorded ratios for m=3,4,5 are 1.006x, 1.022x and 1.056x. These are archived operation counts, not new measurements or F4 speedups.");
    p.source(718.,"Sources: native component / historical M31_HORNER_FINDINGS_20261124.md",PR);

    let mut q=Canvas::new(612.,792.);
    q.heading(2,"EXACT LOCAL ALGEBRA","Preclose each projective chart","Remove known local closure work while retaining the identity point and geometric intermediate abscissae.");
    quotient_diagram(&mut q,42.,140.,528.);
    q.section(463.,"What a quotient-aware matrix can use");
    q.text(42.,487.,10.5,3,INK,"Standard monomials: 1, z, s^k (k>=1), w*s^k (k>=0)");
    q.paragraph(42.,512.,528.,10.,1,INK,"The four quadratic relations form a local Groebner basis for degree-compatible orders. Normal form preserves coefficients and does not increase degree. Separate node blocks use disjoint variables.");
    q.rect(42.,568.,528.,78.,PALE);
    q.text(56.,590.,11.,2,TEAL,"A LOCAL REDUCTION IS NOT A GLOBAL DEGREE BOUND");
    q.paragraph(56.,611.,500.,9.5,1,INK,"Coupled cubic tensors may still create high-degree work. Keep raw product degree, normalized matrix degree and pair-LCM degree distinct. Completion requires the full pair logic, including overlap with the chart ideal.");
    q.paragraph(42.,675.,528.,9.5,1,MUTED,"The prior degree-three first fall disappears after preinserting the local chart consequences. That presentation artifact does not bound F4 solving degree or imply an asymptotic gain.");
    q.source(718.,"Sources: src/quotient.rs; M31_LOCAL_PRECLOSURE.md; Caminata-Gorla",DEGREE);

    let mut r=Canvas::new(612.,792.);
    r.heading(3,"MULTIPLICITY / COMPLETE BRANCH HANDLING","Guard the derivative constraints","Mixed nonzero Frobenius sectors force singular Semaev zeros; pure-sector simple roots must remain available.");
    sector_diagram(&mut r,42.,140.,528.);
    r.section(467.,"The exact sector indicator in the section algebra");
    r.text(42.,491.,9.,3,INK,"G = prod(e_i^2) - prod((e_i^2+e_i)/2)");
    r.text(42.,507.,9.,3,INK,"                  - prod((e_i^2-e_i)/2)");
    r.paragraph(42.,535.,528.,10.,1,INK,"Frobenius pairs the sign classes of each rational root in a mixed specialized fiber. Every such root has even multiplicity. At a complete mixed relation, each sector has at least two leaves with nonzero ordinates, so every coordinate partial vanishes.");
    r.rect(42.,595.,528.,74.,PALE);
    r.text(56.,618.,11.,2,TEAL,"500 FIBERS / 750 PARTIALS / 500 MUTATIONS");
    r.paragraph(56.,639.,500.,9.5,1,INK,"The gate retains 4,630 finite signed branches and 90 infinity branches, covering 3,240 rational roots. All 300 pure-sector fixtures have simple roots. The frozen curve has no rational zero-ordinate leaf (exact gcd=1).");
    r.paragraph(42.,694.,528.,9.,1,MUTED,"Added equations preserve projected zero sets. Equality of nonradical ideals, multiplicities or a performance benefit is not claimed.");
    r.source(718.,"Sources: multiplicity.md; src/multiplicity.rs; src/systems.rs; Diem 2011",DIEM);

    let mut s=Canvas::new(612.,792.);
    s.heading(4,"NEXT DISCRIMINATING COMPARISON","Measure complete solver work","Use the same M31 inputs, section, order and resource limits across all required arms.");
    s.text(42.,146.,9.,2,TEAL,"PRESENTATION");s.text(210.,146.,9.,2,TEAL,"WHAT MUST BE CHARGED");
    let rows=[("Monolithic Semaev","Fixed-degree resultants, construction, solve, projection."),
        ("Preclosed sequential","Leaf lifts, local charts, cubic tensors, all branches."),
        ("Preclosed balanced","The same leaves and charts; only tree shape changes."),
        ("Multiplicity balanced","Full S construction, derivatives, Euler powers, extra rows.")];
    for (i,(a,b)) in rows.iter().enumerate() {
        let y=160.+i as f64*47.;s.rect(42.,y,528.,42.,if i%2==0 {PALE}else{WHITE});
        s.text(52.,y+18.,10.,2,INK,a);s.paragraph(210.,y+16.,347.,9.3,1,INK,b);
    }
    s.section(388.,"The existing checks answer different questions");
    s.paragraph(42.,411.,528.,10.,1,INK,"The 792 positive certificates cover 50 fixtures in each arity/arm/witness-mode cell. They check all equations, including infinity and nonrational intermediates. The 400 monolithic checks compare exact polynomial values. Neither is 50 completed projected Groebner solves per configuration.");
    s.section(480.,"Required result record");
    s.paragraph(42.,503.,528.,10.,1,INK,"Retain exact projected solutions, maximum raw and normalized Macaulay degrees, matrix rows/columns/nonzeros, pair counts, complete generation/solve/extraction time, per-process peak RSS, input/source hashes and every timeout. A resource limit stays incomplete.");
    s.section(575.,"Current execution boundary");
    s.paragraph(42.,598.,528.,9.5,1,INK,"The algebra toolchain is available through workspace PR #109. Shared Conductor/research-KB access, an authenticated EKS lease/capacity view and checkpoint custody remain execution dependencies. The proposed panel has not been submitted as a paid job.");
    s.paragraph(42.,655.,528.,9.5,1,MUTED,"Compare against the applicable original Rust engine and exact sparse msolve F4 on matched inputs. Singular is an algebra checker; its standard-basis command is not relabelled as F4. This four-point section does not establish natural factor-base scaling.");
    s.source(718.,"Sources: PROTOCOL.md; src/system_certificate_tests.rs; tracked PR #5087",PR);
    vec![p,q,r,s]
}

fn markdown()->String {
    format!(r#"# Native M31 projective constraints: component report

**November 23, 2026 Pacific | PR #5087 | producer-local, unsigned evidence.**

The component supplies exact local quotient arithmetic, matched polynomial
presentations and independent correctness checks for the next M31 solver
comparison. Actual matched F4 runtime, peak memory and the required 50
projected solver outputs per configuration remain unmeasured. [pcc:unverified]

## Scope and outputs

The coordinate field is `p=2147483647`, and the curve remains
`E: y^2=x^3-3x+21`. Original input and projected coordinates are in M31;
Fp2 is a witness representation. Every matched arm uses rank-two affine
forms in `t,u` followed by the same section `t(t-1)=u(u-1)=0`. Its four
points are M31 points. This is a bounded correctness workload, not a
natural factor-base scaling experiment. The factor-base map is unchanged.

- [Protocol and execution conditions](PROTOCOL.md)
- [Local quotient arithmetic](src/quotient.rs)
- [Matched system construction and guarded derivatives](src/systems.rs)
- [Bounded quotient F4 prototype](src/f4.rs)
- [Independent multiplicity gate](src/multiplicity.rs)
- [All-equation witness checks](src/system_certificate_tests.rs)
- [Multiplicity proof and limitations](multiplicity.md)
- [Native report renderer](report.rs), [PDF](REPORT.pdf), [editable SVG](report.svg)

{f4_note} [pcc:unverified]

This is a graded reverse-lexicographic reference prototype with simple
bounded matrices, not the optimized production engine. The single completed
case has 30 nonfixed basis polynomials. The independent ordinary checker
examines 561 candidate critical pairs, performs 528 ordinary pairs, checks
11 original generators and 30 reverse containments, and obtains a basis
with 33 polynomials. Two earlier verifier-budget failures remain preserved:
a 32-pair step cap that had been treated as a batch cap, and a 20,000-step
reduction cap. The producer was unchanged; the third bounded verification
completed. This is one actual degree-six case, not a 50-case projected panel
or a runtime/RSS comparison. The complete crate unit-test total is recorded
in the final integration receipt, not inferred here. [pcc:unverified]

## Distinct local correctness observations

| Observation | Count | Exact scope |
|---|---:|---|
| Specialized Semaev fibers | 500 | Full coefficient products, rational root sets and multiplicities |
| Positive tensor certificates | 792 | All generated equations on actual signed-point witnesses |
| Monolithic comparisons | 400 | Exact polynomial values on the complete common section |
| Coordinate partial checks | 750 | Mixed positive Semaev tuples |
| Coefficient mutations rejected | 500 | Specialized fiber corruption controls |
| Pure-sector cases with simple roots | 300 | Reject unconditional derivative constraints |

The specialized panel has 50 cases per sector/arity cell for pure plus,
pure minus and mixed positives at `m=3,4,5`, plus 50 forced infinity and
repeated-coordinate cases. It retains 4,630 finite signed branches and 90
infinity branches, with 3,240 rational roots. The tensor gate uses 50
fixtures in every `m=3,4` / three decomposed arms / two witness-mode cell.
The 400 monolithic checks are `2 arities * 50 planes * 4 section points`.
The all-equation certificates do not establish that a solver recovered all
projected roots, and are not counted as completed Groebner solves.
[pcc:unverified]

## The local quotient

For each geometric internal node,

\[
J=(z^2-z,zw-w,zs-s,w^2-s),\qquad (A,B,C)=(1-z+s,w,z).
\]

At `z=0` the chart is the identity `(A,B,C)=(1,0,0)`. At `z=1`,
`s=w^2`, giving the affine Kummer line. Thus

\[
\mathbb F_p[z,w,s]/J\cong\mathbb F_p\times\mathbb F_p[w].
\]

The standard monomials are `1,z,s^k (k>=1),w*s^k (k>=0)`.
Local normal form never increases total degree. This is a local algebra
fact; coupling the cubic tensors may still produce high global degree.
The exact relation to the prior preclosure appears in
[M31_LOCAL_PRECLOSURE.md](../M31_LOCAL_PRECLOSURE.md). [pcc:unverified]

The bounded F4 prototype must retain overlaps with the chart relations and
distinguish completion from resource exhaustion. Raw product degree,
normalized matrix degree and pair-LCM degree are separate telemetry.
Neither the quadratic generator degree nor a local first fall supplies a
bound on the whole system's solving degree.

## The guarded multiplicity constraint

For a mixed rational Semaev zero with every ordinate nonzero, the two
Frobenius-sector sums coincide in rational two-torsion. Each sector must
contain at least two leaves. Frobenius acts without fixed points on the
relevant sign classes modulo global negation, forcing even multiplicity
in every coordinate fiber. Consequently every coordinate partial vanishes.
The proof retains projective infinity branches. [pcc:unverified]

In the common section algebra, compute all leaf symbols, including the
target,

\[
e_i=f(x_i)^{{(p-1)/2}},\qquad
G=\prod_i e_i^2-\prod_i\frac{{e_i^2+e_i}}{{2}}-
\prod_i\frac{{e_i^2-e_i}}{{2}}.
\]

`G=1` on mixed nonzero sectors and `G=0` on pure or zero-ordinate
branches. The multiplicity-balanced arm appends

\[
\operatorname{{NF}}(G\,\partial_t S),\qquad
\operatorname{{NF}}(G\,\partial_u S).
\]

Differentiate the unreduced monolithic polynomial first, then reduce in
the common section. The added constraints preserve projected zero sets.
Equality of nonradical ideals or multiplicities is not asserted. The exact
gcd `gcd(X^3-3X+21,X^p-X)=1` excludes rational zero-ordinate leaves on this
frozen curve; this does not justify deleting that branch for other curves.
[pcc:unverified]

Pure-sector simple roots show why the guard is necessary. In particular,
ordinary factor-base points on `E(Fp)` and the rational target are pure
plus. A mixed-sector lemma is not an unconditional replacement for their
Semaev equation. Charge the guard, full monolithic construction, formal
derivatives and all added rows to the multiplicity-aware arm.

## Historical operation counts

The archived `m=5` generation counts are **8,424,950** for the previous
pair norm and **6,420,200** for Horner. With the common root-extraction GCD
included, the recorded ratios at `m=3,4,5` are **1.006x, 1.022x, 1.056x**.
These are historical field multiplication counts; this continuation does
not relabel them as new measurements or F4 speedups. See
[the archived Horner note](../M31_HORNER_FINDINGS_20261124.md).

## The next comparison remains required

| Arm | Included preparation |
|---|---|
| Monolithic Semaev | Complete fixed-degree polynomial construction |
| Preclosed sequential | Leaf lifts, local charts and sequential cubic tensors |
| Preclosed balanced | The identical leaves/charts with balanced tree shape |
| Multiplicity balanced | Balanced tensors plus full guarded-derivative preparation |

Run at least 50 actual projected solver outputs per configuration after
the execution conditions in [PROTOCOL.md](PROTOCOL.md) are met. Compare
native quotient F4 with the applicable original Rust engine and exact
sparse msolve F4 under matched order and resource assumptions. Retain full
projected solutions, raw/normalized matrix degrees, dimensions/nonzeros,
pair counts, complete preparation/solve/extraction costs, per-process peak
RSS and every censored attempt. No unmeasured value is entered as zero.

The algebra toolchain is available via workspace PR #109. Shared
Conductor/research-KB access, authenticated EKS lease/capacity information
and checkpoint custody remain execution dependencies. No paid comparison
job is claimed here. The four-point section alone cannot establish natural
factor-base scaling or complete ECDLP improvement. [pcc:unverified]

## Primary references and status

- [Diem 2011]({diem}), Proposition 3.2(c) and Lemma 3.5: finite-flat
  projective fibers and fixed homogeneous resultants.
- [Caminata-Gorla]({degree}): distinctions among solving degree, last fall
  degree and related invariants.
- [Kosters-Yeo]({ky}): first-fall pitfalls; its characteristic-two results
  are not M31 bounds.
- [Tracked source and evidence lane]({pr}).

No novelty priority, Lean theorem, signed PCC verdict, measured runtime
advantage or measured memory advantage is established by this report.
"#,f4_note=F4_NOTE,diem=DIEM,degree=DEGREE,ky=KY,pr=PR)
}

fn main() {
    let output=std::env::args().nth(1).unwrap_or_else(||".".into());
    let path=Path::new(&output);fs::create_dir_all(path).unwrap();
    write_pdf(&path.join("REPORT.pdf"),&pages());
    let mut svg=Canvas::new(1120.,800.);
    svg.rect(0.,0.,1120.,8.,TEAL);
    svg.text(35.,47.,24.,2,INK,"M31 projective algebra and guarded multiplicity");
    svg.text(35.,74.,13.,1,MUTED,"Exact component structure; projected solver runtime and memory remain unmeasured.");
    quotient_diagram(&mut svg,35.,112.,510.);
    sector_diagram(&mut svg,575.,112.,510.);
    svg.line(35.,440.,1085.,440.,1.,LIGHT);
    svg.text(35.,478.,18.,2,TEAL,"COMMON COORDINATE FIELD");
    svg.text(35.,509.,15.,3,INK,"p=2147483647, E: y^2=x^3-3x+21");
    svg.paragraph(35.,545.,495.,13.,1,INK,"Geometric witnesses may use Fp2. Input and projected coordinates remain in M31. Infinity is a valid internal chart, not a discarded denominator failure.");
    svg.text(575.,478.,18.,2,TEAL,"COMMON FOUR-POINT SECTION");
    svg.text(575.,509.,15.,3,INK,"t(t-1)=u(u-1)=0 over M31");
    svg.paragraph(575.,545.,495.,13.,1,INK,"The guard is computed in this exact section algebra. The construction keeps all pure and exceptional branches while activating derivatives only where the invariant applies.");
    svg.rect(35.,653.,1050.,87.,INK);
    svg.text(54.,682.,16.,2,WHITE,"500 specialized fibers  |  792 tensor certificates  |  400 exact monolithic values");
    svg.text(54.,711.,13.,1,WHITE,"Distinct development checks. They are not a 50-case projected Groebner extraction panel.");
    svg.text(35.,775.,10.5,1,MUTED,"Source: PR #5087, summation_split/m31_native/{multiplicity.md,PROTOCOL.md,src/}. Unsigned local evidence.");
    write_svg(&path.join("report.svg"),&svg);
    fs::write(path.join("REPORT.md"),markdown()).unwrap();
    println!("Rendered REPORT.pdf (4 pages), report.svg and REPORT.md");
}
