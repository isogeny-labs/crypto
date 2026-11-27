use std::env;
use std::fmt::Write as _;
use std::fs;

#[derive(Clone, Copy)]
struct Row {
    depth: usize,
    roots: f64,
    poly: f64,
    norm: f64,
    direct: f64,
    slowdown: f64,
}

fn parse_rows(path: &str) -> Vec<Row> {
    let input = fs::read_to_string(path).expect("cannot read result data");
    let mut rows = Vec::new();
    for line in input.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 7, "malformed result row");
        rows.push(Row {
            depth: fields[0].parse().expect("invalid depth"),
            roots: fields[1].parse().expect("invalid root count"),
            poly: fields[3].parse().expect("invalid polynomial work"),
            norm: fields[4].parse().expect("invalid norm cost"),
            direct: fields[5].parse().expect("invalid direct cost"),
            slowdown: fields[6].parse().expect("invalid slowdown"),
        });
    }
    assert_eq!(rows.len(), 6, "expected six frozen depths");
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(row.depth, index + 1, "depths must be consecutive");
    }
    rows
}

fn x_at(index: usize, left: f64, width: f64, count: usize) -> f64 {
    left + width * index as f64 / (count - 1) as f64
}

fn log_y(value: f64, top: f64, height: f64) -> f64 {
    let low = 2.0_f64;
    let high = 6.0_f64;
    top + height * (high - value.log10()) / (high - low)
}

fn polyline(
    svg: &mut String,
    rows: &[Row],
    left: f64,
    top: f64,
    width: f64,
    height: f64,
    color: &str,
    value: impl Fn(Row) -> f64,
    dash: &str,
) {
    let points = rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            format!(
                "{:.1},{:.1}",
                x_at(index, left, width, rows.len()),
                log_y(value(*row), top, height)
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    writeln!(
        svg,
        r#"<polyline points="{points}" fill="none" stroke="{color}" stroke-width="4" stroke-linejoin="round" stroke-linecap="round" stroke-dasharray="{dash}"/>"#
    )
    .unwrap();
    if dash.is_empty() {
        for (index, row) in rows.iter().enumerate() {
            writeln!(
                svg,
                r##"<circle cx="{:.1}" cy="{:.1}" r="5" fill="{color}" stroke="#ffffff" stroke-width="2"/>"##,
                x_at(index, left, width, rows.len()),
                log_y(value(*row), top, height)
            )
            .unwrap();
        }
    }
}

fn panel_axes(svg: &mut String, rows: &[Row], left: f64, top: f64, width: f64, height: f64) {
    writeln!(
        svg,
        r##"<rect x="{left}" y="{top}" width="{width}" height="{height}" rx="12" fill="#ffffff" stroke="#d9e0ea"/>"##
    )
    .unwrap();
    for exponent in 2..=6 {
        let value = 10_f64.powi(exponent);
        let y = log_y(value, top, height);
        writeln!(
            svg,
            r##"<line x1="{left}" y1="{y:.1}" x2="{}" y2="{y:.1}" stroke="#e8edf3"/><text x="{}" y="{:.1}" text-anchor="end" class="tick">10^{exponent}</text>"##,
            left + width,
            left - 10.0,
            y + 4.0
        )
        .unwrap();
    }
    for (index, row) in rows.iter().enumerate() {
        let x = x_at(index, left, width, rows.len());
        writeln!(
            svg,
            r##"<line x1="{x:.1}" y1="{}" x2="{x:.1}" y2="{}" stroke="#e8edf3"/><text x="{x:.1}" y="{}" text-anchor="middle" class="tick">{}</text>"##,
            top,
            top + height,
            top + height + 24.0,
            row.roots as usize
        )
        .unwrap();
    }
    writeln!(
        svg,
        r#"<text x="{}" y="{}" text-anchor="middle" class="axis">full Dickson roots</text>"#,
        left + width / 2.0,
        top + height + 50.0
    )
    .unwrap();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 3, "usage: generate_visual INPUT.tsv OUTPUT.svg");
    let rows = parse_rows(&args[1]);
    let mut svg = String::new();
    svg.push_str(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1280" height="760" viewBox="0 0 1280 760">
<style>
text { font-family: Inter, "DejaVu Sans", sans-serif; fill: #172033; }
.title { font-size: 31px; font-weight: 700; }
.subtitle { font-size: 16px; fill: #556176; }
.panel-title { font-size: 20px; font-weight: 700; }
.panel-note { font-size: 14px; fill: #556176; }
.tick { font-size: 12px; fill: #667085; }
.axis { font-size: 13px; font-weight: 600; fill: #475467; }
.legend { font-size: 13px; font-weight: 600; }
.pill { font-size: 13px; font-weight: 700; }
.footer-title { font-size: 15px; font-weight: 700; fill: #ffffff; }
.footer-note { font-size: 13px; fill: #d0d5dd; }
</style>
<rect width="1280" height="760" fill="#f4f7fb"/>
<text x="54" y="52" class="title">Correct primitive, wrong economics</text>
<text x="54" y="80" class="subtitle">Target-specialized Dickson norm-gcd • sealed depth 1–6 production census • seed=none</text>
<rect x="750" y="31" width="142" height="32" rx="16" fill="#dcfce7"/><text x="821" y="52" text-anchor="middle" class="pill" fill="#166534">CORRECTNESS PASS</text>
<rect x="902" y="31" width="124" height="32" rx="16" fill="#dbeafe"/><text x="964" y="52" text-anchor="middle" class="pill" fill="#1d4ed8">GROWTH PASS</text>
<rect x="1036" y="31" width="104" height="32" rx="16" fill="#fee2e2"/><text x="1088" y="52" text-anchor="middle" class="pill" fill="#b91c1c">SPEED FAIL</text>
<rect x="1150" y="31" width="78" height="32" rx="16" fill="#fef3c7"/><text x="1189" y="52" text-anchor="middle" class="pill" fill="#92400e">NO 4-LEAF</text>
"##,
    );

    let top = 180.0;
    let height = 360.0;
    let width = 500.0;
    let left_one = 82.0;
    let left_two = 698.0;
    panel_axes(&mut svg, &rows, left_one, top, width, height);
    panel_axes(&mut svg, &rows, left_two, top, width, height);

    svg.push_str(
        r##"<text x="82" y="137" class="panel-title">A. Dense polynomial work grows subquadratically</text>
<text x="82" y="160" class="panel-note">ratios 3.575 and 3.341; fitted slope 1.874 &lt; 2</text>
<text x="698" y="137" class="panel-title">B. Yet it loses badly to the direct scan</text>
<text x="698" y="160" class="panel-note">panel 1: square=0.8 FE, inverse=50 FE</text>
"##,
    );

    let anchor = rows[2].poly;
    polyline(
        &mut svg,
        &rows,
        left_one,
        top,
        width,
        height,
        "#2563eb",
        |row| row.poly,
        "",
    );
    polyline(
        &mut svg,
        &rows,
        left_one,
        top,
        width,
        height,
        "#98a2b3",
        |row| anchor * (row.roots / 8.0).powi(2),
        "8 7",
    );
    polyline(
        &mut svg,
        &rows,
        left_two,
        top,
        width,
        height,
        "#dc2626",
        |row| row.norm,
        "",
    );
    polyline(
        &mut svg,
        &rows,
        left_two,
        top,
        width,
        height,
        "#059669",
        |row| row.direct,
        "",
    );

    svg.push_str(
        r##"<line x1="100" y1="624" x2="132" y2="624" stroke="#2563eb" stroke-width="4"/><text x="140" y="629" class="legend">measured max polynomial work</text>
<line x1="350" y1="624" x2="382" y2="624" stroke="#98a2b3" stroke-width="4" stroke-dasharray="8 7"/><text x="390" y="629" class="legend">quadratic reference</text>
<line x1="716" y1="624" x2="748" y2="624" stroke="#dc2626" stroke-width="4"/><text x="756" y="629" class="legend">norm-gcd FE</text>
<line x1="936" y1="624" x2="968" y2="624" stroke="#059669" stroke-width="4"/><text x="976" y="629" class="legend">batched direct FE</text>
"##,
    );

    let last = rows.last().unwrap();
    let norm_y = log_y(last.norm, top, height);
    let direct_y = log_y(last.direct, top, height);
    writeln!(
        svg,
        r##"<line x1="1177" y1="{:.1}" x2="1177" y2="{:.1}" stroke="#7f1d1d" stroke-width="2"/><path d="M1171 {:.1} L1177 {:.1} L1183 {:.1} M1171 {:.1} L1177 {:.1} L1183 {:.1}" fill="none" stroke="#7f1d1d" stroke-width="2"/><rect x="1028" y="{:.1}" width="130" height="34" rx="8" fill="#fff1f2" stroke="#fecdd3"/><text x="1093" y="{:.1}" text-anchor="middle" class="pill" fill="#b91c1c">{:.1}× slower</text>"##,
        norm_y + 7.0,
        direct_y - 7.0,
        norm_y + 7.0,
        norm_y,
        norm_y + 7.0,
        direct_y - 7.0,
        direct_y,
        direct_y - 7.0,
        (norm_y + direct_y) / 2.0 - 17.0,
        (norm_y + direct_y) / 2.0 + 5.0,
        last.slowdown
    )
    .unwrap();

    svg.push_str(
        r##"<rect x="54" y="660" width="1172" height="66" rx="12" fill="#172033"/>
<text x="76" y="686" class="footer-title">Verdict: STOP — correct two-leaf primitive only</text>
<text x="76" y="710" class="footer-note">Optimistic cold primitive projection: 6.282–6.303 billion FE vs 1.6 million ceiling; concrete four-leaf composition absent. No cross-target amortization.</text>
</svg>
"##,
    );
    fs::write(&args[2], svg).expect("cannot write SVG");
}
