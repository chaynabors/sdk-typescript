class MaxTokensReachedException(Exception):
    pass


class _ContextOverflowError(Exception):
    """Internal: raised when the model context window is exceeded."""

    pass
