from dataclasses import fields as dc_fields

from strands.generated.wit_world.imports.types import AnthropicConfig

# Fields derived from the generated WIT dataclass — stays in sync automatically.
_WIT_FIELDS = {f.name for f in dc_fields(AnthropicConfig)}


class AnthropicModel:
    """Config wrapper for Anthropic models.

    Keyword arguments match the fields of the WIT anthropic-config record.
    """

    def __init__(
        self, model_id: str | None = None, api_key: str | None = None, **kwargs
    ):
        self._config: dict = {"provider": "anthropic"}
        if model_id:
            self._config["model_id"] = model_id
        if api_key:
            self._config["api_key"] = api_key
        for k, v in kwargs.items():
            if k in _WIT_FIELDS and v is not None:
                self._config[k] = v

    def _to_config_dict(self):
        return self._config
