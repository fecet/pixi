import glob
import json
import tomllib
from pathlib import Path

import pytest
import jsonschema

HERE = Path(__file__).parent
EXAMPLES = HERE / "examples"
DOC_EXAMPLES = HERE.joinpath("..", "docs", "source_files", "pixi_tomls")
VALID = {ex.stem: ex for ex in (EXAMPLES / "valid").glob("*.toml")} | {
    ex.stem: ex for ex in DOC_EXAMPLES.glob("*.toml")
}
INVALID = {ex.stem: ex for ex in (EXAMPLES / "invalid").glob("*.toml")}


@pytest.fixture(scope="module", params=VALID)
def valid_manifest(request) -> str:
    manifest = VALID[request.param].read_text()
    manifest_toml = tomllib.loads(manifest)
    return manifest_toml


@pytest.fixture(scope="module", params=INVALID)
def invalid_manifest(request) -> str:
    manifest = INVALID[request.param].read_text()
    manifest_toml = tomllib.loads(manifest)
    return manifest_toml


def _real_manifest_path():
    # Get all `pixi.toml` files from the project
    for manifest in glob.glob("../**/**/pixi.toml"):
        if "invalid" in manifest:
            continue
        yield manifest


@pytest.fixture(params=_real_manifest_path())
def real_manifest_path(request):
    return request.param


@pytest.fixture(scope="session")
def manifest_schema():
    with open("schema.json") as f:
        schema = json.load(f)
    return schema


@pytest.fixture(scope="session")
def validator(manifest_schema):
    validator_cls = jsonschema.validators.validator_for(manifest_schema)
    return validator_cls(manifest_schema)


def test_manifest_schema_valid(validator, valid_manifest):
    validator.validate(valid_manifest)


def test_manifest_schema_invalid(validator, invalid_manifest):
    with pytest.raises(jsonschema.ValidationError):
        validator.validate(invalid_manifest)


def test_real_manifests(real_manifest_path, validator):
    print(real_manifest_path)
    with open(real_manifest_path) as f:
        manifest = f.read()
    manifest_toml = tomllib.loads(manifest)
    validator.validate(manifest_toml)


def test_interpreter_field_validation(validator):
    """Test that the interpreter field validation works correctly."""

    # Test 1: Missing interpreter (should be valid - uses deno-task-shell)
    manifest_no_interpreter = {
        'project': {
            'name': 'test',
            'description': 'Test project',
            'channels': ['conda-forge'],
            'platforms': ['linux-64'],
            'version': '0.1.0'
        },
        'tasks': {
            'test': {'cmd': 'echo test'}  # No interpreter field
        }
    }
    validator.validate(manifest_no_interpreter)  # Should not raise

    # Test 2: String interpreter (should be valid)
    manifest_string_interpreter = {
        'project': {
            'name': 'test',
            'description': 'Test project',
            'channels': ['conda-forge'],
            'platforms': ['linux-64'],
            'version': '0.1.0'
        },
        'tasks': {
            'test': {'cmd': 'echo test', 'interpreter': 'bash'}
        }
    }
    validator.validate(manifest_string_interpreter)  # Should not raise

    # Test 3: Array interpreter (should be valid)
    manifest_array_interpreter = {
        'project': {
            'name': 'test',
            'description': 'Test project',
            'channels': ['conda-forge'],
            'platforms': ['linux-64'],
            'version': '0.1.0'
        },
        'tasks': {
            'test': {'cmd': 'echo test', 'interpreter': ['python', '-u']}
        }
    }
    validator.validate(manifest_array_interpreter)  # Should not raise

    # Test 4: Empty string interpreter (should be invalid)
    manifest_empty_string = {
        'project': {
            'name': 'test',
            'description': 'Test project',
            'channels': ['conda-forge'],
            'platforms': ['linux-64'],
            'version': '0.1.0'
        },
        'tasks': {
            'test': {'cmd': 'echo test', 'interpreter': ''}
        }
    }
    with pytest.raises(jsonschema.ValidationError):
        validator.validate(manifest_empty_string)

    # Test 5: Empty array interpreter (should be invalid)
    manifest_empty_array = {
        'project': {
            'name': 'test',
            'description': 'Test project',
            'channels': ['conda-forge'],
            'platforms': ['linux-64'],
            'version': '0.1.0'
        },
        'tasks': {
            'test': {'cmd': 'echo test', 'interpreter': []}
        }
    }
    with pytest.raises(jsonschema.ValidationError):
        validator.validate(manifest_empty_array)

    # Test 6: Array with empty string (should be invalid)
    manifest_array_empty_string = {
        'project': {
            'name': 'test',
            'description': 'Test project',
            'channels': ['conda-forge'],
            'platforms': ['linux-64'],
            'version': '0.1.0'
        },
        'tasks': {
            'test': {'cmd': 'echo test', 'interpreter': ['']}
        }
    }
    with pytest.raises(jsonschema.ValidationError):
        validator.validate(manifest_array_empty_string)
