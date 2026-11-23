#!/usr/bin/env python3
"""Surgical JSON block replacement that preserves every other byte.

Usage: splice.py FILE PATH_SPEC NEW_JSON_TEXT
PATH_SPEC is a '/'-separated key path; the block replaced is the value at that
path (object or array), located by brace matching from its key.
"""
import json
import sys


def find_value_span(text, path):
    """Return (start, end) of the value block for a '/'-separated key path."""
    global idx  # the key's own offset, for indentation
    # Walk to the parent object text, then find `"key":` and match braces.
    keys = path.split("/")
    start = 0
    for key in keys[:-1]:
        idx = text.index('"%s"' % key, start)
        start = idx
    key = keys[-1]
    idx = text.index('"%s"' % key, start)
    colon = text.index(":", idx)
    i = colon + 1
    while text[i] in " \n\t":
        i += 1
    opener = text[i]
    closer = "}" if opener == "{" else "]"
    depth = 0
    j = i
    in_str = False
    while j < len(text):
        c = text[j]
        if in_str:
            if c == "\\":
                j += 2
                continue
            if c == '"':
                in_str = False
        elif c == '"':
            in_str = True
        elif c == opener:
            depth += 1
        elif c == closer:
            depth -= 1
            if depth == 0:
                return i, j + 1
        j += 1
    raise SystemExit("unbalanced: %s" % path)


def main():
    path, spec, new_text_file = sys.argv[1], sys.argv[2], sys.argv[3]
    text = open(path).read()
    new_block = open(new_text_file).read().rstrip("\n")
    # Validate the replacement parses as JSON on its own.
    json.loads(new_block)
    i, j = find_value_span(text, spec)
    key_line_start = text.rindex("\n", 0, idx) + 1
    leading = text[key_line_start:]
    indent = " " * (len(leading) - len(leading.lstrip(" ")))
    # Re-indent the new block to the original column: json.dumps was produced
    # with two spaces starting at column 0, shift every line by `indent`.
    lines = new_block.split("\n")
    shifted = lines[0] + "\n" + "\n".join(indent + ln for ln in lines[1:]) if len(lines) > 1 else lines[0]
    out = text[:i] + shifted + text[j:]
    json.loads(out)  # whole file must still parse
    open(path, "w").write(out)
    print("spliced %s (%d -> %d bytes)" % (spec, j - i, len(shifted)))


if __name__ == "__main__":
    main()
