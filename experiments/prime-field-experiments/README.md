# Prime-field root and map experiments

Rust library and CLI for distinct polynomial roots over word-size prime fields and rational-map degree and fiber measurements. Rayon distributes independent benchmark instances and maps across threads. FLINT provides native finite-field arithmetic through a small owning Rust wrapper.

## Run

Build with Cargo on 64-bit Linux. The bundled FLINT dependencies require a C compiler, Clang, Make, and GNU M4 for the initial build.

```sh
cargo build --locked --release
cargo test --locked --release
target/release/prime-field-experiments benchmark --workers 3 --instances 3 --repeats 9 > root_benchmark_rust.csv
target/release/prime-field-experiments measure example_maps.json --workers 3 > map_measurements_rust.json
```

The benchmark checks every root set against FLINT. Its CSV includes median runtimes for reflection-enabled extraction, the fallback, and FLINT's native root finder. The benchmark uses the original fixture families, field sizes, and degree ranges with Rust's seeded generator.

## Root extraction

The fallback extracts the distinct linear factors with $\gcd(f,X^p-X)$ and splits them using quadratic characters of randomly shifted linear probes. When the coefficients certify $f(X)=g((X-c)^2)$, the solver works at half the degree. It extracts admissible compressed roots with $\gcd(g,U^{(p-1)/2}-1)$, then lifts them to $c\pm\sqrt U$. It adds $c$ once if $g(0)=0$.

For polynomial moduli of degree 32 or greater, FLINT's specialized power of $X$ uses the reversed modulus inverse. A Taylor shift translates linear probes $X+s$ to this form.

## Map input and measurements

Use the format in [example_maps.json](example_maps.json). Supply an odd prime `p`, the coefficients of a square-free odd-degree `curve` polynomial $F$ for $y^2=F(x)$, and a list of nonconstant maps $(A+yB)/(C+yD)$. The two numerator arrays are $[A,B]$, the denominator arrays are $[C,D]$, and each polynomial is listed constant coefficient first. An empty array represents zero.

The optional `targets` array lists field elements and uses `null` for infinity. Omitting it measures the entire projective target line when $p\le10{,}000$. Supply explicit targets for larger fields.

The profiler computes the generic norm $(A-TC)^2-F(B-TD)^2$. Its degree in $x$ after removing the fixed content gives the degree of the map, including inseparable multiplicity. It also reports separability, the number of rational points in each measured fiber, and the polynomial degrees before and after reflection. When a map factors through $x$, the solver uses the reduced rational pencil and the norm has twice its degree.

The supplied synthetic example has 109 rational points and six maps of degrees 2, 3, 4, 8, 2, and 2. Its quartic map yields degrees 8, 4, and 2 at the norm, elimination, and reflection stages. The profiler checks all 612 fibers against direct curve enumeration. The project-specific auxiliary curve and LOE equations have yet to be supplied.
