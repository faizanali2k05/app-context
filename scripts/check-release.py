"""Check source and release metadata without reading credentials or publishing."""
from pathlib import Path
import argparse
import subprocess
import tomllib

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--source", action="store_true", help="Check source artifacts without release-only metadata")
args = parser.parse_args()
errors = []
for name in ("app-context", "app-context-derive"):
    folder = root / "crates" / name
    manifest = tomllib.loads((folder / "Cargo.toml").read_text())
    package = manifest["package"]
    for key in ("description", "documentation", "keywords", "categories", "readme"):
        if not package.get(key):
            errors.append(f"{name}: missing {key}")
    for file in ("README.md", "CHANGELOG.md", "LICENSE-MIT", "LICENSE-APACHE"):
        if not (folder / file).is_file():
            errors.append(f"{name}: missing {file}")
    if not args.source:
        repository = package.get("repository", "")
        if not repository.startswith("https://"):
            errors.append(f"{name}: supply a verified HTTPS repository URL")
        if package.get("publish") is False:
            errors.append(f"{name}: publication remains disabled")
    for source in (folder / "src").rglob("*.rs"):
        text = source.read_text()
        for forbidden in ("todo!", "unimplemented!", "TODO", "FIXME"):
            if forbidden in text:
                errors.append(f"{source.relative_to(root)}: unfinished marker {forbidden}")
if errors:
    raise SystemExit("\n".join(errors))
print("Source artifacts verified" if args.source else "Local release metadata verified; still verify URL ownership, credentials, package contents and all release checks")
