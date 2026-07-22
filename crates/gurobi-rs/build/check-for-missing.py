#!/usr/bin/env -S uv run
import argparse
import asyncio

import pandas as pd


async def print_missing(impl, session, kind, full_list, implemented):
    if kind == "attribute":
        row_fmt = "{name},{dtype},{otype}"
        fetch_data = impl.fetch_attribute_data
        ignore = lambda data: data["otype"] in ("batch", "gconstr") or data["cl_only"]
    elif kind == "parameter":
        row_fmt = "{name},{dtype}"
        fetch_data = impl.fetch_parameter_data
        ignore = lambda data: data["cl_only"]
    else:
        raise ValueError

    extra = {p for p in implemented if p not in full_list}
    missing = await asyncio.gather(
        *(
            fetch_data(session, p, path)
            for p, path in full_list.items()
            if p not in implemented
        )
    )
    missing = {d["name"]: d for d in missing}
    append_csv = []

    for p, data in sorted(missing.items()):
        if ignore(data):
            print("IGNORE", p)
            continue
        print("MISSING", p)
        append_csv.append(row_fmt.format_map(data))

    for p in sorted(extra):
        print("EXTRA", p)

    if append_csv:
        print("\n\n" + f" New {kind.capitalize()}s ".center(100, "-"))
        print("\n".join(append_csv))


async def main(args):
    if args.gurobi13:
        import utils_v13 as impl

        print("[check-for-missing] using v13 (Sphinx) scraper")
    else:
        import utils as impl

        print("[check-for-missing] using legacy (refman 11.0) scraper")

    params_csv = pd.read_csv("params.csv")
    attr_csv = pd.read_csv("attrs.csv")
    implemented_parameters = set(params_csv["param"])
    implemented_attributes = set(attr_csv["attr"])

    async with impl.http_session() as session:
        parameters, attributes = await asyncio.gather(
            impl.fetch_parameter_list(session), impl.fetch_attribute_list(session)
        )

        await print_missing(
            impl, session, "parameter", parameters, implemented_parameters
        )
        await print_missing(
            impl, session, "attribute", attributes, implemented_attributes
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--gurobi13",
        action="store_true",
        help="Use the new docs.gurobi.com Sphinx site (utils_v13). "
        "Default: legacy gurobi.com/documentation/11.0/refman (utils.py).",
    )
    asyncio.run(main(parser.parse_args()))
