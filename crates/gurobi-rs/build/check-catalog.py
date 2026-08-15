#!/usr/bin/env python3
# Validate the generated catalog inputs and compile every version feature.

from __future__ import annotations

import argparse
import csv
import subprocess
import sys
from pathlib import Path


SUPPORTED_FEATURES = ("gurobi10", "gurobi11", "gurobi12", "gurobi13")
CATALOG_FEATURES = ("", "gurobi13")
ATTRIBUTE_DTYPES = {"dbl", "int", "chr", "str", "custom"}
PARAMETER_DTYPES = {"dbl", "int", "str"}
OBJECT_TYPES = {"model", "var", "constr", "gconstr", "qconstr", "sos"}


def read_catalog(path: Path, fields: list[str], dtypes: set[str], kind: str):
    with path.open(newline="", encoding="utf-8") as stream:
        reader = csv.DictReader(stream)
        if reader.fieldnames != fields:
            raise ValueError(
                f"{path}: expected columns {fields}, got {reader.fieldnames}"
            )

        rows = []
        names = set()
        for line_number, row in enumerate(reader, start=2):
            name = row[fields[0]]
            dtype = row["dtype"]
            feature = row["feature"] or ""
            if not name:
                raise ValueError(f"{path}:{line_number}: empty {kind} name")
            if name in names:
                raise ValueError(f"{path}:{line_number}: duplicate {kind} {name}")
            if dtype not in dtypes:
                raise ValueError(f"{path}:{line_number}: invalid dtype {dtype!r}")
            if feature not in CATALOG_FEATURES:
                raise ValueError(
                    f"{path}:{line_number}: unsupported catalog feature {feature!r}"
                )
            if kind == "attribute" and row["otype"] not in OBJECT_TYPES:
                raise ValueError(
                    f"{path}:{line_number}: invalid object type {row['otype']!r}"
                )
            names.add(name)
            rows.append(row)
        return rows


def check_catalog_inputs(build_dir: Path):
    attrs = read_catalog(
        build_dir / "attrs.csv",
        ["attr", "dtype", "otype", "feature"],
        ATTRIBUTE_DTYPES,
        "attribute",
    )
    params = read_catalog(
        build_dir / "params.csv",
        ["param", "dtype", "feature"],
        PARAMETER_DTYPES,
        "parameter",
    )

    for feature in SUPPORTED_FEATURES:
        active_attrs = [
            row for row in attrs if not row["feature"] or row["feature"] == feature
        ]
        active_params = [
            row for row in params if not row["feature"] or row["feature"] == feature
        ]
        print(
            f"{feature}: {len(active_attrs)} attributes, "
            f"{len(active_params)} parameters"
        )

    return attrs, params


def check_generated_code(repo_root: Path, runtime: bool):
    for feature in SUPPORTED_FEATURES:
        command = [
            "cargo",
            "test" if runtime else "check",
            "-p",
            "gurobi-rs",
            "--lib",
            "--features",
            feature,
        ]
        if runtime:
            command.append("names")
        else:
            command.insert(4, "--tests")
        print("+", " ".join(command))
        result = subprocess.run(command, cwd=repo_root)
        if result.returncode:
            return result.returncode
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--skip-cargo",
        action="store_true",
        help="validate CSV inputs without compiling each supported feature",
    )
    parser.add_argument(
        "--runtime",
        action="store_true",
        help="run the attribute_names and parameter_names tests against each library",
    )
    args = parser.parse_args()

    script = Path(__file__).resolve()
    repo_root = script.parents[3]
    build_dir = script.parent
    try:
        check_catalog_inputs(build_dir)
    except (OSError, ValueError) as error:
        print(f"catalog check failed: {error}", file=sys.stderr)
        return 1

    if not args.skip_cargo:
        return check_generated_code(repo_root, args.runtime)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
