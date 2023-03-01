"""Init."""

from . import _version
from arbeider.remote import remote, Remote


__version__ = _version.get_versions()['version']


__all__ = ["remote", "Remote"]
