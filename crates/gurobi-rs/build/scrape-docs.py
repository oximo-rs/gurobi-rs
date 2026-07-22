#!/usr/bin/env -S uv run
"""Scrape parameter/attribute documentation from the Gurobi reference manual.

Dependencies are managed with `uv` via `pyproject.toml` (see build/README.md).

Usage:
    uv run scrape-docs.py            # legacy gurobi.com/documentation/11.0/refman site
    uv run scrape-docs.py --gurobi13 # updated docs.gurobi.com Sphinx site
"""

import asyncio
import json
import textwrap
from pathlib import Path
from typing import Dict

import pandas as pd

DOC_PATH = Path(__file__).parent / "docstrings"
MD_WORD_WRAP = 120


class DocumentationFiles:
    def __init__(self, name: str):
        self.body = DOC_PATH / "body" / f"{name}.md"
        self.metadata = DOC_PATH / "metadata" / f"{name}.json"

    def all_exist(self):
        return self.body.exists() and self.metadata.exists()


def _postprocess_doc_paragraph(params: Dict[str, str], s: str) -> str:
    words = s.split()
    for i, w in enumerate(words):
        # strip trailing punctuation when matching the symbol name
        stripped = w.rstrip(",.;:)")
        if stripped in params:
            suffix = w[len(stripped) :]
            words[i] = f"`{stripped}`{suffix}"
    return " ".join(words)


def _wrap_doc_paragraph(parameters: Dict[str, str], paragraph: str) -> str:
    """Wrap prose while preserving Markdown lists emitted by the v13 scraper."""
    wrapped_lines = []
    for line in paragraph.splitlines():
        if line.startswith("|"):
            wrapped_lines.append(line)
        elif line.startswith("- "):
            text = _postprocess_doc_paragraph(parameters, line[2:])
            wrapped_lines.extend(
                textwrap.wrap(
                    text,
                    MD_WORD_WRAP,
                    initial_indent="- ",
                    subsequent_indent="  ",
                )
            )
        else:
            text = _postprocess_doc_paragraph(parameters, line)
            wrapped_lines.extend(textwrap.wrap(text, MD_WORD_WRAP))
    return "\n".join(wrapped_lines)


def create_body_file(path: Path, parameters: Dict[str, str], pdata: dict):
    paragraphs = (
        _wrap_doc_paragraph(parameters, para)
        for para in pdata["doc"]
    )
    body = "\n\n".join(paragraphs)
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8") as fp:
        fp.write(body)
    print("wrote", path)


def create_metadata_file(path: Path, data: dict):
    data = data.copy()
    data.pop("doc", None)
    data.pop("cl_only", None)
    data.pop("ty", None)
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8") as fp:
        json.dump(data, fp, indent="  ")
    print("wrote", path)


async def create_documentation(impl, session, kind, full_list, name, args=None):
    if kind == "attribute":
        files = DocumentationFiles(name + "_attr")
        fetch = impl.fetch_attribute_data
    elif kind == "parameter":
        files = DocumentationFiles(name + "_param")
        fetch = impl.fetch_parameter_data
    else:
        raise ValueError("kind must be attribute or parameter")

    write_body = (args is not None and args.overwrite_body) or not files.body.exists()
    write_meta = (
        args is not None and args.overwrite_meta
    ) or not files.metadata.exists()
    if not (write_body or write_meta):
        return
    if name not in full_list:
        return

    try:
        data = await fetch(session, name, full_list[name])
    except Exception as e:
        print(f"FAIL {kind} {name}: {e}")
        return

    if write_body:
        create_body_file(files.body, full_list, data)
    if write_meta:
        create_metadata_file(files.metadata, data)


async def main(args):
    (DOC_PATH / "body").mkdir(parents=True, exist_ok=True)
    (DOC_PATH / "metadata").mkdir(parents=True, exist_ok=True)

    if args.gurobi13:
        import utils_v13 as impl

        print("[scrape-docs] using v13 (Sphinx) scraper")
    else:
        import utils as impl

        print("[scrape-docs] using legacy (refman 11.0) scraper")

    here = Path(__file__).parent
    param_csv = pd.read_csv(here / "params.csv")
    attr_csv = pd.read_csv(here / "attrs.csv")

    async with impl.http_session() as session:
        attributes = await impl.fetch_attribute_list(session)
        await asyncio.gather(
            *(
                create_documentation(impl, session, "attribute", attributes, a, args)
                for a in attr_csv["attr"]
            )
        )
        parameters = await impl.fetch_parameter_list(session)
        await asyncio.gather(
            *(
                create_documentation(impl, session, "parameter", parameters, p, args)
                for p in param_csv["param"]
            )
        )


if __name__ == "__main__":
    import argparse

    p = argparse.ArgumentParser()
    p.add_argument(
        "--gurobi13",
        action="store_true",
        help="Use the new docs.gurobi.com Sphinx site (utils_v13). "
        "Default: legacy gurobi.com/documentation/11.0/refman (utils.py).",
    )
    p.add_argument(
        "-m",
        "--overwrite-meta",
        action="store_true",
        help="Clobber parameter and attribute metadata. Usually fine.",
    )
    p.add_argument(
        "-b",
        "--overwrite-body",
        action="store_true",
        help="Overwrite documentation body (the main text). THIS WILL DELETE ANY MANUAL EDITS.",
    )
    args = p.parse_args()
    asyncio.run(main(args))
