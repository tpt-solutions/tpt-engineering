"""Compare the public function signatures of two source trees.

Usage: pub_api_diff.py <published_src_dir> <current_src_dir>

Both arguments are a crate's `src/` directory. The published directory is the
crates.io source, which cargo extracts under ~/.cargo/registry/src after a
`cargo fetch` of the exact version. The script lists every `pub fn` whose
signature was removed or changed, and every new one.

This is a signature check, not a semver checker. It catches parameter and
return-type changes that cargo-semver-checks has missed, but it does not judge
behaviour changes. Read the diff before choosing a version bump.
"""

import pathlib
import re
import sys

PUB_FN = re.compile(r"\bpub\s+fn\s+(\w+)([^{;]*)", re.DOTALL)


def signatures(src: pathlib.Path) -> dict[str, str]:
    found: dict[str, str] = {}
    for path in sorted(src.rglob("*.rs")):
        text = path.read_text(encoding="utf-8")
        text = "\n".join(line for line in text.splitlines() if not line.lstrip().startswith("//"))
        for match in PUB_FN.finditer(text):
            name = match.group(1)
            signature = " ".join(match.group(0).split())
            found[f"{path.relative_to(src)}::{name}"] = signature
    return found


def main() -> int:
    if len(sys.argv) != 3:
        print(__doc__)
        return 2
    old = signatures(pathlib.Path(sys.argv[1]))
    new = signatures(pathlib.Path(sys.argv[2]))
    removed = sorted(k for k in old if k not in new)
    changed = sorted(k for k in old if k in new and old[k] != new[k])
    added = sorted(k for k in new if k not in old)
    for key in changed:
        print(f"CHANGED {key}\n  was: {old[key]}\n  now: {new[key]}")
    for key in removed:
        print(f"REMOVED {key}\n  was: {old[key]}")
    print(f"added: {len(added)} public functions")
    print(f"summary: {len(changed)} changed, {len(removed)} removed")
    return 1 if changed or removed else 0


if __name__ == "__main__":
    sys.exit(main())
