"""Scraper for the Gurobi 13 documentation site.

The Gurobi 13 reference manual lives at https://docs.gurobi.com/projects/optimizer/en/current/
and uses a Sphinx-style DOM (one `<section id="...">` per item).

This module provides v13-specific replacements with the same return shapes as the old
`fetch_*_data`/`fetch_*_list` functions, so `scrape-docs.py` can swap implementations
based on a `--gurobi13` flag.
"""

from __future__ import annotations

import re
import urllib.parse
from pathlib import Path

import aiohttp
from bs4 import BeautifulSoup, Tag

CACHE_DIR = Path("cache")

GUROBI_REF_MAN_URL = urllib.parse.urlparse(
    "https://docs.gurobi.com/projects/optimizer/en/current/reference/"
)

# parameters live on a single page; attributes are split across category pages.
PARAMETERS_PAGE = "parameters.html"
ATTRIBUTES_INDEX = "attributes.html"

_DTYPES = {
    "double": "dbl",
    "string": "str",
    "int": "int",
    "char": "chr",
}

# attribute category file stem -> otype CSV value
_OTYPES_BY_PAGE = {
    "batch": "batch",
    "constraintgeneral": "gconstr",
    "constraintlinear": "constr",
    "constraintquadratic": "qconstr",
    "model": "model",
    "multiobjective": "model",
    "multiscenario": "model",
    "quality": "model",
    "sos": "sos",
    "variable": "var",
}


def get_url(path: str) -> str:
    return GUROBI_REF_MAN_URL._replace(path=GUROBI_REF_MAN_URL.path + path).geturl()


async def fetch_html(session: aiohttp.ClientSession, path: str) -> str:
    cache_path = CACHE_DIR / "v13" / path
    if cache_path.exists():
        return cache_path.read_text(encoding="utf-8")
    url = get_url(path)
    async with session.get(url) as res:
        print("GET", url, res.status)
        if res.status != 200:
            raise Exception(f"bad request: {url} -> {res.status}")
        text = await res.text()
    cache_path.parent.mkdir(parents=True, exist_ok=True)
    cache_path.write_text(text, encoding="utf-8")
    return text


def _remove_newlines(s: str) -> str:
    return " ".join(filter(bool, s.splitlines()))


def _strip_trailing_examples_note(paragraphs: list[str]) -> list[str]:
    """Drop the boilerplate 'For examples ... refer to ... Examples.' tail paragraph."""
    drop_patterns = [
        re.compile(r"For examples of how to query or modify .* refer to .*\."),
        re.compile(r"One important note about integer-valued parameters: while .*"),
        re.compile(r"Please refer to .* for more information on SOS constraints\."),
    ]
    return [p for p in paragraphs if not any(r.fullmatch(p) for r in drop_patterns)]


def _section_paragraphs(section: Tag) -> list[str]:
    """Collect descriptive text from a Sphinx section, excluding its metadata list."""
    out: list[str] = []
    metadata_list = section.find("ul", class_="simple", recursive=False)
    for child in section.children:
        if not isinstance(child, Tag):
            continue
        if child.name == "p":
            text = _remove_newlines(child.get_text(" ", strip=True))
            if text:
                out.append(text)
        elif child.name in ("ul", "ol") and child is not metadata_list:
            items = [
                _remove_newlines(item.get_text(" ", strip=True))
                for item in child.find_all("li", recursive=False)
            ]
            items = [item for item in items if item]
            if items:
                out.append("\n".join(f"- {item}" for item in items))
        elif child.name == "div" and "admonition" in (child.get("class") or []):
            # admonition note (e.g. "Barrier only") — keep its body text as a paragraph
            body = child.find_all("p")
            for p in body[1:]:  # skip the title
                text = _remove_newlines(p.get_text(" ", strip=True))
                if text:
                    out.append(text)
        elif child.name == "div" and "deprecated" in (child.get("class") or []):
            text = _remove_newlines(child.get_text(" ", strip=True))
            if text:
                out.append(text)
        elif child.name == "div" and "math-wrapper" in (child.get("class") or []):
            text = _remove_newlines(child.get_text(" ", strip=True))
            if text.startswith(r"\[") and text.endswith(r"\]"):
                text = f"${text[2:-2].strip()}$"
            if text:
                out.append(text)
        elif child.name == "div" and "table-wrapper" in (child.get("class") or []):
            rows = []
            for tr in child.find_all("tr"):
                cells = [
                    _remove_newlines(cell.get_text(" ", strip=True)).replace("|", r"\|")
                    for cell in tr.find_all(("th", "td"), recursive=False)
                ]
                if cells:
                    rows.append(cells)
            if rows:
                width = max(len(row) for row in rows)
                rows = [row + [""] * (width - len(row)) for row in rows]
                table = [
                    "| " + " | ".join(rows[0]) + " |",
                    "| " + " | ".join(["---"] * width) + " |",
                ]
                table.extend("| " + " | ".join(row) + " |" for row in rows[1:])
                out.append("\n".join(table))
    return _strip_trailing_examples_note(out)


def _parse_meta_list(section: Tag) -> dict[str, str]:
    """Extract the `<ul class="simple">` key/value pairs (Type, Default, Modifiable, ...)."""
    ul = section.find("ul", class_="simple")
    pairs: dict[str, str] = {}
    if ul is None:
        return pairs
    for li in ul.find_all("li", recursive=False):
        text = li.get_text(" ", strip=True)
        if ":" not in text:
            continue
        key, _, val = text.partition(":")
        pairs[key.strip()] = val.strip()
    return pairs


async def fetch_parameter_list(session: aiohttp.ClientSession) -> dict[str, str]:
    """Return `{ParamName: anchor_path}` where anchor_path is `parameters.html#<id>`."""
    html_doc = await fetch_html(session, PARAMETERS_PAGE)
    soup = BeautifulSoup(html_doc, "html.parser")
    out: dict[str, str] = {}
    for section in soup.select("section[id]"):
        # the per-parameter sections live inside the top-level "parameter-reference"
        # section as direct children. Skip the wrapper.
        if section.get("id") == "parameter-reference":
            continue
        h2 = section.find("h2", recursive=False)
        if h2 is None:
            continue
        name = h2.contents[0].strip() if h2.contents else None
        if not name:
            continue
        out[name] = f"{PARAMETERS_PAGE}#{section['id']}"
    return out


async def fetch_parameter_data(
    session: aiohttp.ClientSession, name: str, anchor_path: str
) -> dict:
    page, _, anchor = anchor_path.partition("#")
    html_doc = await fetch_html(session, page)
    soup = BeautifulSoup(html_doc, "html.parser")
    section = soup.find("section", id=anchor)
    if section is None:
        raise Exception(f"missing section #{anchor} on {page}")

    meta = _parse_meta_list(section)
    ty = meta.get("Type", "").strip()
    if ty not in _DTYPES:
        raise Exception(f"unknown type {ty!r} for parameter {name}")

    paragraphs = _section_paragraphs(section)
    # remove the leading short title paragraph (right under <h2>) from the doc body
    # it duplicates the section heading and is not useful in rustdoc.
    if paragraphs:
        paragraphs = paragraphs[1:]

    blob = " ".join(paragraphs)
    data = {
        "name": name,
        "url": get_url(anchor_path),
        "ty": ty,
        "dtype": _DTYPES[ty],
        "default": meta.get("Default value", ""),
        "cl_only": "Note: Command-line only" in blob or "Command-line only" in blob,
        "doc": paragraphs,
    }
    if ty in ("int", "double"):
        data["min"] = meta.get("Minimum value", "")
        data["max"] = meta.get("Maximum value", "")
    return data


async def fetch_attribute_list(
    session: aiohttp.ClientSession,
) -> dict[str, str]:
    """Return `{AttrName: 'attributes/<category>.html#<id>'}`."""
    html_doc = await fetch_html(session, ATTRIBUTES_INDEX)
    soup = BeautifulSoup(html_doc, "html.parser")
    # the index page lists attribute names as `<a href="attributes/<cat>.html#anchor">Name</a>`
    href_re = re.compile(r"^attributes/[^/]+\.html#")
    out: dict[str, str] = {}
    for a in soup.find_all("a", href=href_re):
        text = a.get_text(strip=True)
        if not text or text.endswith("Attributes"):
            continue
        out.setdefault(text, a["href"])
    return out


async def fetch_attribute_data(
    session: aiohttp.ClientSession, name: str, anchor_path: str
) -> dict:
    page, _, anchor = anchor_path.partition("#")
    html_doc = await fetch_html(session, page)
    soup = BeautifulSoup(html_doc, "html.parser")
    section = soup.find("section", id=anchor)
    if section is None:
        raise Exception(f"missing section #{anchor} on {page}")

    meta = _parse_meta_list(section)
    ty = meta.get("Type", "").strip()
    if ty not in _DTYPES:
        raise Exception(f"unknown type {ty!r} for attribute {name}")

    modifiable = meta.get("Modifiable", "").strip().lower() == "yes"

    # derive object type from category page stem (model.html -> model, variable.html -> var, ...)
    stem = Path(page).stem
    otype = _OTYPES_BY_PAGE.get(stem, "model")

    paragraphs = _section_paragraphs(section)

    return {
        "name": name,
        "url": get_url(anchor_path),
        "ty": ty,
        "dtype": _DTYPES[ty],
        "otype": otype,
        "modifiable": modifiable,
        "cl_only": False,
        "doc": paragraphs,
    }


def http_session() -> aiohttp.ClientSession:
    http_headers = {
        "user-agent": "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0"
    }
    return aiohttp.ClientSession(
        connector=aiohttp.TCPConnector(limit=20), headers=http_headers
    )
