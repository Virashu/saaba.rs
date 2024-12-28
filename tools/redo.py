from __future__ import annotations

from pathlib import Path

root = Path(__file__).parent

res: list[str] = []

with Path(root / "mime_types.txt").open() as f:
    lines = f.read().split("\n")

    for line in lines:
        ext, mime = line.split(maxsplit=1)
        res.append(f'"{ext}" => "{mime}",')

with Path(root / "code.rs").open("w") as f:
    f.write("\n".join(res))
