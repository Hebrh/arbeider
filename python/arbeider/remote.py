"""Remote run python function in rust executor."""
import cloudpickle
from arbeider.internal import remote_func


class Remote:
    """Remote is a remote executor.

    :param func: function to be executed
    :type func: function
    :param name: remote name, default is function name
    :type name: str
    """
    def __init__(self, func, name):
        # Picket code to bytes
        self._func = cloudpickle.dumps(func)
        self._name = name

    def __call__(self, *args, **kwargs):
        """Call function in rust executor and get result."""
        remote_func(self._func, args, kwargs)
        cloudpickle.loads(self._func)(*args, **kwargs)


def remote(func=None, name=""):
    """Decorator for a python function to be executed in rust executor.

    :param func: function to be executed
    :type func: function
    :param name: remote name, default is function name
    :type name: str
    """
    # If func is not None, return a Remote object
    if func:
        return Remote(func, name)

    # If func is None, return a decorator
    return lambda f: Remote(f, name)
