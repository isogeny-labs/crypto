# Fixed inputs for the census and streaming rank checks

These are exact finite computations on synthetic inputs. Membership probes and stream event costs define the comparisons. The replay has zero timed metrics and uses no sampled cohort.

## Coverage

Enumerate all 6,188 labelled five-element subsets of the residues modulo 17 in ascending order. Each supplies 25 ordered pairs, with repeats. Compare five uniform residual draws that each use two membership probes with two planted draws that each use five probes to count all representations. The planted score uses inverse multiplicity on residuals that pass the same gate. Compare two gates, one using the first two positions and one taking a fresh uniform choice from the ten position pairs for each draw. The latter is evaluated by exact finite averaging. Every base gets both gates. Cross-check the variance signs using separate enumeration of elementary outcomes. Tabulate wins, losses and ties with raw support and gate changes. These budgets omit draw generation, subset selection and bookkeeping.

## Rank

Use four synthetic shapes over fields of orders 2, 3, 5 and 101 with corresponding primary and auxiliary dimensions (3,2), (5,4), (7,5), and (9,6). In each, reorder the same distinct dense anchor and perturbation rows to vary when auxiliary directions appear. Keep the attempt cost schedule, duplicate and rejection positions fixed. A 48-attempt deterministic stress stream per field adds zero rows, repeats, field-equivalent repeats, primary-only and auxiliary-only rows, and pseudorandom integer coefficients with fixed seeds. Verify every prefix, including the empty one, using two streaming implementations and batch elimination that extracts rows with zero auxiliary coordinates. Compare readiness signals to full primary rank. Record stored pivot coefficients separately from the stream's charged attempts. Assigned work measures the fixtures and coefficient counts exclude container overhead.

From this directory, build each program `g++ -std=c++17 -Wall -Wextra -Werror -pedantic -O2 coverage_census.cpp -o /tmp/ic-extension-coverage` and `g++ -std=c++17 -Wall -Wextra -Werror -pedantic -O2 rank_incremental.cpp -o /tmp/ic-extension-rank`. Run the two executables without arguments and compare stdout to `coverage.txt` and `rank.txt`.

Documentation clarification after replay. The frozen rank source also includes a one-primary, one-auxiliary control over fields of orders 2 and 3. The coefficient-2 primary entry vanishes in characteristic two, making that row a duplicate. Both controls count toward the total of 388 checked prefixes.
