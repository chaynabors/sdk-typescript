from dataclasses import fields as dc_fields

from strands.generated.wit_world.imports.types import BedrockConfig

# Fields derived from the generated WIT dataclass — stays in sync automatically.
_WIT_FIELDS = {f.name for f in dc_fields(BedrockConfig)}


class BedrockModel:
    """Config wrapper for Bedrock models.

    Keyword arguments match the fields of the WIT bedrock-config record.
    """

    def __init__(
        self, model_id: str = "us.anthropic.claude-sonnet-4-20250514", **kwargs
    ):
        if "region_name" in kwargs:
            kwargs["region"] = kwargs.pop("region_name")
        self._config = {"provider": "bedrock", "model_id": model_id}
        for k, v in kwargs.items():
            if k in _WIT_FIELDS and v is not None:
                self._config[k] = v

    def _to_config_dict(self):
        return self._config
