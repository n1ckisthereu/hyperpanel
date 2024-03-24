import base64


class Utils:
    def __init__(self) -> None:
        pass

    def return_rps(self, content: str):
        return base64.b64encode(content.encode("utf-8")).decode("utf-8")

    def try_parse_int(self, value):
        try:
            return int(value)
        except ValueError:
            return float("inf")  # or float('-inf') to negative values
