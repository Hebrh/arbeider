def start():
    """Start the arbeider daemon."""
    print("Starting arbeider worker...")


__all__ = ["start"]

from . import _version
__version__ = _version.get_versions()['version']
