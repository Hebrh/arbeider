"""Remote run python function in rust executor."""
import pickle


class Remote:
    """Remote is a remote executor.

    :param func: function to be executed
    :param args: args of function
    :param kwargs: kwargs of function
    """

    def __init__(self, func, args, kwargs):
        # Picket code to bytes
        self._func = pickle.dumps(func)
        self._args = args
        self._kwargs = kwargs

    def run(self):
        """Run function in rust executor and get result."""

        pass
