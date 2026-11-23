#!/bin/sh
set -eu
cd "$(dirname "$0")"
cp ../feature-native-triple-modes/*.rs .
sed 's/^fn main()/fn signed_predecessor_triple_main()/' ../feature-native-triple-modes/triple-checks.rs > triple-checks.rs
cp modes.rs driver.rs
printf '\ninclude!("signed.rs");\ninclude!("checks-signed.rs");\nfn main(){if std::env::args().any(|a|a=="--inherited"){inherited_checks()}else{signed_checks();}}\n' >> driver.rs
