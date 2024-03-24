#!/usr/bin/python3

import sys
import json
import os
import re
from utils import Utils
import glob

hclient = Utils()

HOME = os.environ["HOME"]
consts = {"PICTURE_DIR": f"{HOME}/Pictures", "CACHE_DIR": f"{HOME}/.cache"}


def parse_pywal():
    with open(f"{consts['CACHE_DIR']}/wal/colors.json") as f:
        return json.loads(f.read())


def get_pictures():
    pattern = re.compile(r"^.*\.(jpg|jpeg|png|gif)$", re.IGNORECASE)
    wallpapers_dir = os.path.join(consts["PICTURE_DIR"], "Wallpapers")

    files = glob.glob(os.path.join(wallpapers_dir, "*"))
    files = [f for f in files if pattern.match(os.path.basename(f))]

    files.sort(
        key=lambda x: (
            hclient.try_parse_int(os.path.splitext(os.path.basename(x))[0]),
            x,
        )
    )

    return files


def get_infos():
    wal = parse_pywal()

    return hclient.return_rps(
        json.dumps(
            {
                "current_wallpaper": wal["wallpaper"],
                "color_scheme": [
                    wal["special"]["background"],
                    wal["special"]["foreground"],
                    wal["special"]["cursor"],
                    wal["colors"]["color1"],
                    wal["colors"]["color2"],
                    wal["colors"]["color10"],
                ],
                "pictures": get_pictures(),
            }
        )
    )


def main():
    if sys.argv[1] == "get_infos":
        print(get_infos())


if __name__ == "__main__":
    main()
