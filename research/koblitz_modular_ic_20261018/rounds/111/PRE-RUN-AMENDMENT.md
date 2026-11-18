# Pre-run source correction

The first frozen R111 source had a mistaken hand-entered assertion for the known R97 special fiber: y^4(y+1)^3 has exponents 4,5,6,7, not 3,4,5,6,7. The source was corrected before any R111 data run. An unused dead expression in the edge builder was also removed. The original source commit remains in Git history; the corrected source was committed before execution.
