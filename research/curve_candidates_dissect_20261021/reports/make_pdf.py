"""Create a paginated technical PDF from the frozen assessment evidence."""
import hashlib
import json
import textwrap
from pathlib import Path
from xml.sax.saxutils import escape

import fitz
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfgen import canvas
from reportlab.platypus import Paragraph, Table, TableStyle
from reportlab.lib.utils import ImageReader

ROOT=Path(__file__).resolve().parent
WORK=ROOT.parent
OUT=WORK/'elliptic-curve-assessment.pdf'
for name,file in [('Sans','DejaVuSans.ttf'),('SansBold','DejaVuSans-Bold.ttf'),('Mono','DejaVuSansMono.ttf')]:
    pdfmetrics.registerFont(TTFont(name,'/usr/share/fonts/truetype/dejavu/'+file))
pdfmetrics.registerFontFamily('Sans',normal='Sans',bold='SansBold')
NAVY=colors.HexColor('#17334B');TEAL=colors.HexColor('#087E80')
INK=colors.HexColor('#263547');PALE=colors.HexColor('#EFF5F8');GREY=colors.HexColor('#607183')
W,H=A4;LEFT=48;RIGHT=48;WIDTH=W-LEFT-RIGHT
BODY=ParagraphStyle('body',fontName='Sans',fontSize=9.3,leading=13.8,textColor=INK,spaceAfter=6)
SMALL=ParagraphStyle('small',parent=BODY,fontSize=8.1,leading=11.6)
CELL=ParagraphStyle('cell',parent=BODY,fontSize=8.2,leading=11.5)


class Report:
    def __init__(self):
        self.c=canvas.Canvas(str(OUT),pagesize=A4,pageCompression=1)
        self.c.setTitle('Elliptic-curve assessment | ICV1 and DiSSECT')
        self.c.setAuthor('Computed assessment')
        self.page=0;self.y=H-58
    def new(self,title,kicker):
        if self.page:self.finish()
        self.page+=1;self.y=H-59
        self.c.setFillColor(TEAL);self.c.setFont('SansBold',8)
        self.c.drawString(LEFT,self.y,kicker.upper());self.y-=29
        self.c.setFillColor(NAVY);self.c.setFont('SansBold',19)
        self.c.drawString(LEFT,self.y,title);self.y-=23
    def finish(self):
        self.c.setStrokeColor(colors.HexColor('#D3DFE6'))
        self.c.line(LEFT,42,W-RIGHT,42)
        self.c.setFillColor(GREY);self.c.setFont('Sans',7.3)
        self.c.drawString(LEFT,29,'Elliptic-curve assessment  •  21 October 2026')
        self.c.drawRightString(W-RIGHT,29,str(self.page));self.c.showPage()
    def p(self,text,small=False):
        obj=Paragraph(text,SMALL if small else BODY)
        _,height=obj.wrap(WIDTH,1000)
        assert self.y-height>=53,('page overflow',self.page,self.y,height,text[:60])
        obj.drawOn(self.c,LEFT,self.y-height);self.y-=height+7
    def heading(self,text):
        self.y-=6;self.c.setFillColor(NAVY);self.c.setFont('SansBold',10.3)
        self.c.drawString(LEFT,self.y,text);self.y-=16
    def code(self,text,size=7.5,shade=True):
        lead=size*1.36
        n=int((WIDTH-18)/pdfmetrics.stringWidth('0','Mono',size))
        lines=[]
        for part in str(text).splitlines() or ['']:
            lines.extend(textwrap.wrap(part,width=n,break_long_words=True,break_on_hyphens=False,replace_whitespace=False,drop_whitespace=False) or [''])
        height=len(lines)*lead+12
        assert self.y-height>=53,('code overflow',self.page,self.y,height)
        if shade:
            self.c.setFillColor(PALE);self.c.roundRect(LEFT,self.y-height,WIDTH,height,3,stroke=0,fill=1)
        self.c.setFont('Mono',size);self.c.setFillColor(INK)
        at=self.y-8-size
        for line in lines:self.c.drawString(LEFT+8,at,line);at-=lead
        self.y-=height+7
    def table(self,rows,widths,header=True):
        formatted=[]
        for i,row in enumerate(rows):
            style=ParagraphStyle('th',parent=CELL,fontName='SansBold',textColor=colors.white) if i==0 and header else CELL
            formatted.append([Paragraph(escape(str(x)),style) for x in row])
        tab=Table(formatted,colWidths=widths,hAlign='LEFT')
        commands=[('VALIGN',(0,0),(-1,-1),'TOP'),('LEFTPADDING',(0,0),(-1,-1),8),
                  ('RIGHTPADDING',(0,0),(-1,-1),8),('TOPPADDING',(0,0),(-1,-1),6),
                  ('BOTTOMPADDING',(0,0),(-1,-1),6),('LINEBELOW',(0,0),(-1,-1),0.35,colors.HexColor('#D9E4EA'))]
        if header:commands.append(('BACKGROUND',(0,0),(-1,0),NAVY))
        for i in range(1 if header else 0,len(rows)):
            if i%2:commands.append(('BACKGROUND',(0,i),(-1,i),PALE))
        tab.setStyle(TableStyle(commands));_,height=tab.wrap(WIDTH,1000)
        assert self.y-height>=53,('table overflow',self.page,self.y,height)
        tab.drawOn(self.c,LEFT,self.y-height);self.y-=height+10


def main():
    identities=json.loads((ROOT/'curves.icv1.json').read_text())['curves']
    certs=json.loads((ROOT/'assessment-certificates.json').read_text())
    cert={c['curve']:c for c in certs}
    summary=json.loads((ROOT/'continuation/continuation-summary.json').read_text())
    coords=json.loads((ROOT/'continuation/coordinate-torsion-certificates.json').read_text())['results']
    raw=json.loads((ROOT/'dissect-results.json').read_text())
    bundle=WORK/'curve-assessment-continued.zip'
    r=Report()
    r.new('Elliptic-curve assessment','ICV1 identities • DiSSECT traits • verified follow-up')
    r.p('Four supplied candidates over the NIST P-224 and P-256 prime fields. '
        'This report combines the original computation and the completed follow-up. '
        'The candidates have their own coefficients and are distinct from the standardized NIST curves.')
    r.table([['Default trait calls','Assessed records with all fields','Output corrections'],
             [f'{raw["recorded_calls"]} across 22 traits',f'{summary["assessed_records_with_all_fields"]} / 292','10 records; 2 source defects']],
             [166,167,WIDTH-333])
    r.heading('Principal findings')
    r.p('All four supplied curves have proven prime full group order, cofactor <b>h = 1</b>, '
        'and a generator satisfying <b>nG = O</b>. The seeds, generator-selection rule and root conventions match. '
        'Each curve is ordinary and non-anomalous, with certified endomorphism conductor <b>1</b>.')
    rows=[['Input candidate','Curve work ≈ bits','Largest twist prime: bits','Twist work ≈ bits']]
    for row in identities:
        c=cert[row['curve_id']['slug']]
        label=row['source_label'].replace('P224-candidate-','224 / ').replace('P256-candidate-','256 / ')
        rows.append([label,f'{c["generic_curve_security_bits_approx"]:.1f}',c['largest_twist_prime_bits'],f'{c["generic_twist_security_bits_approx"]:.1f}'])
    r.table(rows,[115,115,147,WIDTH-377])
    r.p('Work estimates are approximate log₂ group-operation counts from the square root of the relevant prime order. '
        'They are mathematical generic-DLP estimates, rather than measured attacks or a security certification. '
        'Twist figures concern twist points; proper point validation keeps protocol operations in the supplied curve subgroup.',small=True)
    r.heading('What is complete, and what remains')
    r.p('The follow-up closed <b>three of five</b> missing auxiliary results. Two quadratic-twist factorizations '
        'over Fp² remain partial, with proven-composite residuals of <b>327 bits</b> and <b>444 bits</b>. '
        'All base-field curve and twist orders are fully certified.')
    r.p('The original 292-call DiSSECT run is preserved. The 290/292 figure describes a labeled assessed view '
        'that incorporates verified corrections and recovered factorizations; it does not claim a repeat of every default call.',small=True)
    r.heading('Evidence included')
    r.p('The complete evidence ZIP, ICV1 JSON and the two-line source patch are embedded as PDF attachments. '
        'The ZIP includes raw outputs, the assessed dataset, certificates, reproduction scripts, source snapshots and hash manifests. '
        'Some browser PDF viewers require downloading the PDF to access attachments.',small=True)
    r.p('Candidate numbers are preserved as supplied labels. The search iterations, rejection history and any '
        'statistical outlier comparison against a simulated population were not reconstructed.',small=True)

    r.new('Methods and DiSSECT corrections','Execution and verification')
    r.heading('Independent arithmetic certificates')
    r.p('<b>Full group order.</b> Proof-enabled SageMath establishes primality of p and n. Independent affine arithmetic '
        'and SageMath both verify that the supplied nonidentity G is killed by n. Lagrange’s theorem and the unique '
        'multiple of n in the Hasse interval certify the full cardinality and h = 1.')
    r.p('<b>Endomorphism ring and embedding degree.</b> Every Frobenius discriminant D = t² − 4p is factored into '
        'proven primes and is fundamental. For an ordinary curve this gives Z[π] = End(E) = O<sub>K</sub>. '
        'Embedding degrees are checked by modular exponentiation and prime-divisor minimality tests.')
    r.p('<b>Seed and root choices.</b> The X9.62 SHA-1 expansion produces c with b²c = a³ mod p. '
        'Each 224-bit b is the smaller root; the 256-bit b equals (a³/c)<super>(p+1)/4</super> mod p, including its sign. '
        'For the 224-bit field, changing b to −b is an Fp-isomorphism. For the 256-bit field it gives the quadratic twist.',small=True)
    r.heading('Verified source defects')
    r.p('<b>Torsion extension.</b> In the repeated-eigenvalue branch, the full point-field degree must be multiplied '
        'by ℓ; the native code overwrites it with ℓ. Six records are affected. Division-polynomial factorization '
        'and residue-field square-character checks independently account for every nonidentity torsion point.')
    tor=[['Input candidate','ℓ','Least','Full','Relative']]
    for c in coords:
        label=c['source_label'].replace('P224-candidate-','224 / ').replace('P256-candidate-','256 / ')
        d=c['verified_corrected'];tor.append([label,c['l'],d['least'],d['full'],d['relative']])
    r.table(tor,[165,65,85,85,WIDTH-400])
    r.p('<b>Extension trace.</b> Four degree-2 records incorrectly repeat the base trace. The corrected field is '
        't₂ = t² − 2p; their factor lists already apply to that extension trace. The patched copies pass all six '
        'torsion checks and eight trace checks (degrees 1 and 2 on each curve).',small=True)
    r.code('torsion_extension.py:     k2 = l  →  k2 *= l\ntrace_factorization.py:   "trace": curve.trace()  →  "trace": trace',size=7.6)
    r.p('DiSSECT commit: '+raw['tool_commit']+'<br/>'+escape(raw['sage_version'])+
        '. The original default run took '+f'{raw["elapsed_seconds"]:.1f}'+' seconds; internal factorization limits were retained.',small=True)

    r.new('Recovered results and open factors','Follow-up computation')
    r.heading('Three parameter results recovered')
    r.table([['Input candidate','Trait / parameter','Result'],
        ['224 / 1485','conductor, degree 4','Complete factorization via |t(t² − 2p)|'],
        ['256 / 876','kn_factorization, k = 4','Both 4n + 1 and 4n − 1 fully factored'],
        ['256 / 876','square_4p1','p = 1; order = 1: both integers are squarefree']],
        [100,165,WIDTH-265])
    r.p('Every recovered prime factor is proven and every integer product is checked. '
        'The degree-4 conductor-ratio identity reuses the proven base and degree-2 trace factors. '
        'The exact factors are retained in the embedded evidence.',small=True)
    for pending in summary['remaining']:
        label=pending['source_label'].replace('P224-candidate-','224 / ').replace('P256-candidate-','256 / ')
        part=pending['partial_factorizations']['factorization']
        cofactor=part['unresolved_composite_factors'][0]
        r.heading(label+' · quadratic twist over Fp²')
        r.p('Proven prime factors: '+', '.join(part['proven_prime_factors'])+'.',small=True)
        r.p('Unresolved cofactor ('+str(int(cofactor).bit_length())+' bits), proven composite:',small=True)
        r.code(cofactor,size=8)
        r.p('Exact cardinality = the product of the listed prime factors and this cofactor.',small=True)
    r.heading('Factoring scope and provenance')
    r.p('Distinct integer targets were deduplicated. The retry allowed 120 seconds of Sage/PARI factoring '
        'and 120 seconds of deterministic GMP-ECM per target; the exposed residuals then received a '
        '120-second FLINT attempt each. Logs retain bounds, curve seeds and the partial factors. '
        'Residual composites have not been treated as primes.',small=True)
    r.p('These two open factorizations concern auxiliary groups over Fp². '
        'No statistical outlier result, runtime speedup or asymptotic attack claim follows from this report.',small=True)
    r.heading('Primary sources and attached evidence')
    for label,url in [('ICV1 specification','https://github.com/aburan28/crypto/blob/main/docs/curves/ICV1.md'),
                      ('ICV1 reference implementation','https://github.com/aburan28/crypto/blob/main/scripts/curve_id.py'),
                      ('Pinned DiSSECT source','https://github.com/crocs-muni/DiSSECT/tree/'+raw['tool_commit'])]:
        r.p('<link href="'+url+'" color="#087E80">'+label+'</link>',small=True)
    r.p('Evidence ZIP SHA-256:',small=True);r.code(hashlib.sha256(bundle.read_bytes()).hexdigest(),size=7.6)

    for row in identities:
        identity=row['curve_id'];c=cert[identity['slug']];params=row['parameters']
        title=row['source_label'].replace('P224-candidate-','224-bit candidate ').replace('P256-candidate-','256-bit candidate ')
        r.new(title,'Exact parameter record and certified identity')
        r.p(escape(identity['slug']),small=True)
        r.heading('Supplied parameters')
        entries=[]
        for k in ['p','a','b','Gx','Gy','n','h','seed']:
            value=str(params[k]);entries.append([k,value])
        # Parameter values use an embedded monospace font and never lose zero padding.
        data=[]
        for k,v in entries:
            data.append([Paragraph('<b>'+k+'</b>',CELL),Paragraph('<font name="Mono" size="7.6">'+escape(v)+'</font>',CELL)])
        tab=Table(data,colWidths=[44,WIDTH-44]);tab.setStyle(TableStyle([
            ('VALIGN',(0,0),(-1,-1),'TOP'),('BACKGROUND',(0,0),(-1,-1),PALE),
            ('LEFTPADDING',(0,0),(-1,-1),7),('RIGHTPADDING',(0,0),(-1,-1),7),
            ('TOPPADDING',(0,0),(-1,-1),4),('BOTTOMPADDING',(0,0),(-1,-1),4)]))
        _,height=tab.wrap(WIDTH,1000);tab.drawOn(r.c,LEFT,r.y-height);r.y-=height+8
        r.p('Input values are hexadecimal except h. Derived ICV1 integers use the reference format’s decimal encoding.',small=True)
        r.heading('Full canonical ICV1 identity')
        r.code(identity['icv1'],size=7.2)
        r.p('Model SHA-256: <font name="Mono" size="7.1">'+identity['model_sha256']+'</font>',small=True)
        r.code(identity['model_json'],size=7.1)
        r.heading('Certified derived values')
        r.p('Prime full order; h = 1. Endomorphism conductor = 1. ICV1 level = unk; path = r.',small=True)
        r.p('Trace t = '+c['trace']+'<br/>Embedding degree k = '+c['embedding_degree']+
            ' = (n − 1) / '+c['embedding_degree_complement']+'.',small=True)
        r.p('Endomorphism discriminant:',small=True);r.code(c['endomorphism_discriminant'],size=7.5)
        r.p('Base-field quadratic twist order:',small=True);r.code(c['twist_order'],size=7.5)
        r.p('Proven prime factorization of that twist order:',small=True)
        r.code(' × '.join(c['twist_order_prime_factorization']),size=7.4)
    r.new('Result coverage and provenance','Recorded accounting • original and assessed views')
    r.p('The upstream default run and the assessed view each contain 292 parameter records. '
        'The graph shows field availability only. Ten original records contain the verified source defects '
        'described on page 2; numerical availability alone does not certify a result.')
    picture=ImageReader(str(ROOT/'report-figures/availability.png'))
    iw,ih=picture.getSize();height=WIDTH*ih/iw
    r.c.drawImage(picture,LEFT,r.y-height,width=WIDTH,height=height,mask='auto');r.y-=height+20
    r.p('Source: continuation/continuation-summary.json; editable plotting source and '
        'graph-source.json accompany the report. The counts are deterministic accounting for this '
        'single four-curve run; no uncertainty interval or sampled population is implied.',small=True)
    r.heading('How to read the evidence')
    r.table([['Layer','Contents','Role'],
        ['Supplied inputs','Four parameter sets, seeds and generator conventions','Exact requested models'],
        ['Frozen DiSSECT output','22 traits, 292 default calls','Original observations and timeouts'],
        ['Independent checks','Order, factors, embedding degrees and point coordinates','Evidence for stated arithmetic conclusions'],
        ['Assessed view','Ten corrected records; three recovered parameter results','Versioned interpretation with field-level provenance'],
        ['Open obligations','Two composite cofactors over Fp²','Incomplete factorization; no impossibility claim']],
        [100,190,WIDTH-290])
    r.p('No IC/rho timing comparison, solver implementation, global curve-registry update or leaderboard '
        'promotion is made by archiving this report.',small=True)
    r.finish();r.c.save()
    # Embed the evidence without an automatic action or external dependency.
    with fitz.open(OUT) as doc:
        doc.embfile_add('curve-assessment-evidence.zip',bundle.read_bytes(),filename='curve-assessment-evidence.zip',desc='Complete frozen assessment and continuation evidence')
        doc.embfile_add('curves.icv1.json',(ROOT/'curves.icv1.json').read_bytes(),filename='curves.icv1.json',desc='Canonical ICV1 identities and exact supplied models')
        doc.embfile_add('dissect-corrections.patch',(ROOT/'continuation/dissect-corrections.patch').read_bytes(),filename='dissect-corrections.patch',desc='Verified two-line patch for pinned DiSSECT source')
        doc.save(str(OUT.with_suffix('.attached.pdf')),garbage=4,deflate=True)
    OUT.with_suffix('.attached.pdf').replace(OUT)
    with fitz.open(OUT) as doc:
        all_text=''.join(page.get_text() for page in doc)
        flattened=''.join(all_text.split())
        for row in identities:
            assert ''.join(row['curve_id']['icv1'].split()) in flattened
            assert ''.join(row['curve_id']['model_json'].split()) in flattened
            for field in ['p','a','b','Gx','Gy','n','seed']:
                assert row['parameters'][field] in flattened
        assert doc.embfile_count()==3
        assert doc.embfile_get('curve-assessment-evidence.zip')==bundle.read_bytes()
        for page in doc:
            for word in page.get_text('words'):
                assert word[0]>=LEFT-1 and word[2]<=W-RIGHT+1,('horizontal clipping',page.number,word)
                assert word[1]>=20 and word[3]<=H-16,('vertical clipping',page.number,word)
        preview=ROOT/'pdf-preview';preview.mkdir(exist_ok=True)
        for i,page in enumerate(doc):
            page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(str(preview/f'page-{i+1}.png'))
        print('PDF created:',OUT,'pages:',len(doc),'attachments:',doc.embfile_count(),'bytes:',OUT.stat().st_size)


if __name__=='__main__':main()
