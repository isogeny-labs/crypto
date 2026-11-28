if .kind == "case" then del(.input_sha256, .input_bytes)
elif .kind == "scope" then del(.case_keys.input_sha256, .case_keys.input_bytes)
else . end
