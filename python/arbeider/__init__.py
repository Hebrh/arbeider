from . import _version
from ._internal import Stage

__version__ = _version.get_versions()['version']


def run():
    """Start the arbeider daemon."""
    print("Starting arbeider worker...")


__all__ = ["run"]

