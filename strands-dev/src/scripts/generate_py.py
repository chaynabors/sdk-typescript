"""Regenerate Python type bindings from wit/agent.wit using componentize-py.

Copies generated .py files (excluding WASI runtime support) into
strands-py/strands/generated/ with fixed import paths and @generated headers.

Usage: python generate_py.py <repo-root>
"""

import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HEADER = "# @generated from wit/agent.wit -- do not edit"

IMPORT_REWRITES: list[tuple[str, str]] = [
    ("from componentize_py_types ", "from strands.generated.componentize_py_types "),
    ("from ..imports ", "from strands.generated.wit_world.imports "),
]


def main() -> None:
    root = Path(sys.argv[1])
    py_dir = root / "strands-py"
    dest = py_dir / "strands" / "generated"

    with tempfile.TemporaryDirectory() as tmp:
        subprocess.run(
            [
                str(py_dir / ".venv/bin/componentize-py"),
                "-d", str(root / "wit"),
                "-w", "agent",
                "bindings", tmp,
            ],
            check=True,
            cwd=str(py_dir),
        )

        # Clean stale artefacts
        for name in ("componentize_py_types.py", "wit_world"):
            target = dest / name
            if target.is_file():
                target.unlink()
            elif target.is_dir():
                shutil.rmtree(target)

        tmp_path = Path(tmp)
        for src in tmp_path.rglob("*.py"):
            rel = src.relative_to(tmp_path)

            if "componentize_py_async_support" in rel.parts:
                continue
            if rel.name == "poll_loop.py":
                continue

            lines = src.read_text().splitlines(keepends=True)
            out_lines: list[str] = []
            for line in lines:
                for old, new in IMPORT_REWRITES:
                    if line.startswith(old):
                        line = line.replace(old, new, 1)
                        break
                out_lines.append(line)

            target = dest / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(f"{HEADER}\n{''.join(out_lines)}")

    print(f"generated {dest}/")


if __name__ == "__main__":
    main()
