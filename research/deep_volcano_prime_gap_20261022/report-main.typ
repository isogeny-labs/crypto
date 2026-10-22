// Build: pandoc -f gfm -t typst README.md -o report.typ; pandoc -f gfm -t typst THEOREMS.md -o theorems.typ;
// then compile this file with Typst (python -c "import typst; typst.compile('report-main.typ', output='report.pdf')").
#include "report-preamble.typ"
#include "report.typ"
#pagebreak()
#include "theorems.typ"
