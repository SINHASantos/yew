from typing import Dict, List, Optional
from pathlib import Path

import glob
import os
import json


def find_example_sizes(parent_dir: Path) -> Dict[str, int]:
    example_sizes: Dict[str, int] = {}

    for example_dist_dir in (parent_dir / "dist").iterdir():

        total_size = 0

        # For examples with multiple bundles, we add them together.
        for bundle in example_dist_dir.glob(f"*.wasm"):
            size = bundle.stat().st_size

            print(f"{bundle} has a size of {size}.")

            total_size += size

        if total_size > 0:
            example_sizes[example_dist_dir.name] = total_size

    return example_sizes


def main() -> None:
    sizes = find_example_sizes(Path.cwd())

    size_cmp_info = {
        "sizes": sizes,
        "issue_number": os.environ["ISSUE_NUMBER"],
    }

    with open(".SIZE_CMP_INFO", "w+") as f:
        f.write(json.dumps(size_cmp_info, indent=4))


if __name__ == "__main__":
    main()
