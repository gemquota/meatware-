import json, math, sys
from pathlib import Path
cpp=json.loads(Path(sys.argv[1]).read_text())
rust=json.loads(Path(sys.argv[2]).read_text())
for key in ("entities","relations","steps"):
    if cpp[key] != rust[key]:
        raise SystemExit(f"workload mismatch: {key}")
if not math.isclose(cpp["checksum"],rust["checksum"],rel_tol=1e-11,abs_tol=1e-9):
    raise SystemExit(f"checksum mismatch: {cpp['checksum']} vs {rust['checksum']}")
print(f"Parity passed: {cpp['entities']:,} entities, {cpp['relations']:,} relations, {cpp['steps']} steps.")
print(f"Informational timing: C++ {cpp['elapsed_ms']:.3f} ms; Rust {rust['elapsed_ms']:.3f} ms.")
