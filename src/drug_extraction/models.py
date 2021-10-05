from __future__ import annotations
from typing import Any, Optional
import pkg_resources
from collections import defaultdict
from ruamel.yaml import YAML
from pydantic import BaseModel, Field

yaml = YAML()


class Drug(BaseModel):
    name: str
    tags: set[str] = Field(default_factory=set[str])
    search_terms: set[str] = Field(default_factory=set[str])

    def __str__(self) -> str:
        return f"name={self.name}, tags={self.tags}"

    def __repr__(self) -> str:
        return f"Drug(name={self.name}, tags={self.tags})"

    # TODO: validation of tags and search terms


class DrugExtractor(BaseModel):
    _drugs: dict[str, Drug] = Field(default_factory=dict[str, Drug])

    class Config:
        underscore_attrs_are_private = True

    def __init__(self, **data: Any) -> None:
        super().__init__(**data)
        self.load_from_file()

    def load_from_file(
        self, file_path: str = "./data/drug_classes.yaml"
    ) -> dict[str, Drug]:
        """Load drug classifications from file."""
        filename = pkg_resources.resource_filename(__name__, file_path)
        with open(filename, "r", encoding="utf-8-sig") as f:
            yml_data = yaml.load(f)
            yml_drugs = {k.lower(): Drug(**v) for k, v in yml_data.items()}
            self._drugs = yml_drugs
            return yml_drugs

    def extract(self, sent: str) -> dict[str, int]:
        data = defaultdict(int)
        for word in sent.split():
            for name, info in self._drugs.items():
                if word.lower() in {x.lower() for x in info.search_terms}:
                    data[name] += 1
        return data

    def drug_info(self, name: str) -> Drug | None:
        return next(
            (info for drug_name, info in self._drugs.items() if drug_name == name), None
        )
