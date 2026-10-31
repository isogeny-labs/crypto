#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= Two-dimensional invariant-space census
Run ID: `run-20261022T0621Z-quartic-2d-census`. Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0. Every nonzero `a+b t²` was tested on eight canonical quartic twists. The independent norm and point-orbit audit passed on all 265,008 candidates.

#table(columns: (.5fr,.8fr,.9fr,.9fr,1.2fr), inset: 4pt,
  [$p$], [two axes], [interior], [full base], [signed pair entries],
  [13], [0], [72], [72], [5,112],
  [37], [48], [576], [624], [388,752],
  [73], [96], [2,448], [2,544], [6,469,392],
  [97], [72], [4,464], [4,536], [20,570,760],
  [157], [192], [12,384], [12,576], [158,143,200],
  [229], [192], [26,424], [26,616], [708,384,840],
  [241], [192], [28,512], [28,704], [823,890,912],
  [337], [360], [56,016], [56,376], [3,178,197,000],
)

Every nonempty abscissa orbit has size 12 and signed point orbit size 24. The p=13 empty-line control has 72 interior abscissae. Pair entries are a derived construction count, not measured memory. Source, commands and raw files are linked in `README.md`.

#image("space_census.png", width: 100%)
