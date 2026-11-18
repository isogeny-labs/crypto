# Prime-field index-calculus follow-ups

These development experiments use public synthetic fixtures. The coverage census used 48 curves. The Split4 and RR9 pilots used the fixed public prime31 curve. The analysis started from crypto commit `833a35d21ec2368ab081b2083d7762ed0bfd2bbc`.

| Study | Result |
| --- | --- |
| [Informative coverage](coverage/RESULT.txt) | Dickson D4 trailed the random-base mean by 0.91 percentage points at the registered threshold. The paired 95% interval was −2.38 to +0.52 points. |
| [Implicit Split4](split4/REPORT.md) | The x-only join matched 60 fixtures and halved peak keys at B = 128. The support grew quadratically on the tested panel. |
| [RR9 certificates](rr9/RESULT.md) | All 54 sign-independent runs replayed on the three frozen B8 residuals. Each exact negative identity packed to 86,124 bytes. |
| [APS parity weights](label-proof/PROTOCOL.txt) | Four-slot weights (1,1,1,2) give a nonzero signed denominator for subgroup order greater than five at the infinity endpoint. |

Measurements are local component diagnostics. The studies make no end-to-end, scaling, or ledger claim. The coverage replay regenerates the full census and checks its digest. The RR9 replay regenerates the expanded and packed identities from the checked-in public inputs. The original large raw files are retained in the local replay bundle. Tested with GCC 14.2 and Singular 4.4.1.

For the parity check, run from `label-proof`.

```sh
g++ -std=c++20 -Wno-return-type -O2 -Wall -Wextra -Werror parity.cpp -o parity
./parity | diff - result.txt
```
