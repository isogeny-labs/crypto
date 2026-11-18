#!/bin/sh
set -eu
cd "$(dirname "$0")"
sed -e 's@include!("@include!("../feature-native-triple-modes/@g' -e 's@path="@path="../feature-native-triple-modes/@g' -e 's@^mod meter;@#[path="../feature-native-triple-modes/meter.rs"] mod meter;@' -e 's/^fn main()/fn inherited_main()/' ../feature-native-triple-modes/modes.rs > driver.rs
printf '\ninclude!("signed.rs");\ninclude!("checks.rs");\nfn main(){signed_checks();}\n' >> driver.rs
