import subprocess, os
U = "s3://isolab-results-104186811093-us-east-2/inputs/exotic-h1/run.py"
subprocess.run(["aws", "s3", "cp", U, "/tmp/h1_run.py", "--only-show-errors"], check=True)
src = open("/tmp/h1_run.py").read()
exec(compile(src, "h1_run.py", "exec"), globals())
