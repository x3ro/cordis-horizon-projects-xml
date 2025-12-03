import sys
from datetime import date, datetime
from glob import glob

import dacite
import prettyprinter
import xmltodict
from dacite import Config, from_dict
from prettyprinter import cpprint
from tqdm import tqdm

import model

prettyprinter.install_extras(
    exclude=["ipython_repr_pretty", "attrs", "numpy", "django", "ipython"]  # pyright: ignore[reportArgumentType]
)


def parse_datetime(xs):
    return datetime.fromisoformat(xs)


def parse_date(xs):
    return date.fromisoformat(xs)


def parse_float(xs):
    return float(xs)


def parse_int(xs):
    return int(xs)


def parse_bool(xs):
    return bool(xs)


config = Config(
    type_hooks={
        date: parse_date,
        datetime: parse_datetime,
        float: parse_float,
        bool: parse_bool,
        int: parse_int,
    },
    cast=[model.AssociationType],
    strict=True,
)

DEBUG_MODE = True


# There are a bunch of entries in the XML that have a `readOnly` attribute
# and then just text as a child. We don't really care about that attribute,
# so we "lift up" the text, so that xmltodict creates
# { "availableLanguages": "en" } instead of
# {"availableLanguages": { "readOnly": True, "#text": "en"}}
READONLY_KEYS = [
    "displayCode",
    "lastUpdateDate",
    "contentCreationDate",
    "availableLanguages",
    "size",
    "mimetype",
    "hashValue",
]


def postprocessor(path, key, value):
    if key == "title" and value is None:
        return key, ""

    if key in READONLY_KEYS:
        return key, value.get("#text", "")
    else:
        return key, value


def check_file(path):
    with open(path, mode="rb") as f:
        data = xmltodict.parse(
            f,
            postprocessor=postprocessor,
            force_list=(
                "organization",
                "category",
                "call",
                "article",
                "regions",
                "programme",
                "result",
                "region",
                "webItem",
                "webLink",
            ),
            attr_prefix="",
            process_comments=False,
        )
        data = data["project"]
        project = from_dict(data_class=model.Project, data=data, config=config)
        if DEBUG_MODE:
            cpprint(project)


def main():
    global DEBUG_MODE

    files = list(sys.argv)
    files.pop(0)  # The name of the script
    if len(files) < 1:
        DEBUG_MODE = False
        files = glob("data/*.xml")

    print_fn = print
    if not DEBUG_MODE:
        files = tqdm(files)
        print_fn = tqdm.write

    for file in files:
        try:
            check_file(file)
        except dacite.DaciteError as e:  # noqa: E722
            print_fn(f"{file}: Fail ({e})")


if __name__ == "__main__":
    main()
