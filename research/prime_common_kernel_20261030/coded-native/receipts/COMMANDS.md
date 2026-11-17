# Executed commands

First strict build (exit1), then after indentation correction second strict
build (exit0), run001 (exit134):

```sh
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic -Werror /workspace/audit/coded-operator-native.cpp -lgmpxx -lgmp -o /workspace/audit/coded-operator-native
/workspace/audit/coded-operator-native
```

Initial sanitizer localization (exit1):

```sh
g++ -std=c++20 -O1 -g -fsanitize=address,undefined -fno-omit-frame-pointer /workspace/audit/coded-operator-native.cpp -lgmpxx -lgmp -o /workspace/audit/coded-operator-native-asan
/workspace/audit/coded-operator-native-asan
```

After canonicalizing each rationalfactor, strict build003 and run002 exit0.
The same sanitizer command then exits0 and its output is byte-identical to
run002 (`diff -u` exit0). The portable helper subsequently repeats both
final configurations, with their outputs retained separately. No clock
measurement or scientific timing claim is attached to these commands.
