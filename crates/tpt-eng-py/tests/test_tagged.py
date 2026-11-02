import math

import pytest

import tpt_eng
from tpt_eng import air


def test_tagged_is_exposed_on_top_level_module():
    assert tpt_eng.Tagged.__name__ == "Tagged"


def test_in_range_water_saturation_has_value_source_and_no_warnings():
    t = tpt_eng.water_saturation_pressure_tagged(373.15)
    assert t.value == pytest.approx(101_325, rel=1e-3)
    assert t.in_range is True
    assert t.warnings == []
    assert t.source == "IAPWS-IF97 (2007)"
    assert ("temperature", "K", 273.15, 647.096) in t.validity


def test_out_of_range_water_saturation_warns_and_keeps_value():
    t = tpt_eng.water_saturation_pressure_tagged(700.0)
    assert t.in_range is False
    assert len(t.warnings) == 1
    assert "temperature = 700 K" in t.warnings[0]
    assert math.isfinite(t.value) and t.value > 0


def test_strict_raises_value_error_naming_the_input():
    with pytest.raises(ValueError, match="temperature"):
        tpt_eng.water_saturation_pressure_tagged(700.0, strict=True)


def test_strict_passes_for_in_range_input():
    t = tpt_eng.water_saturation_pressure_tagged(300.0, strict=True)
    assert t.in_range


def test_state_above_range_warns_and_keeps_region_and_value():
    t = tpt_eng.water_state_tagged(1100.0, 1.0e5)
    assert t.value.region == "vapour"
    assert t.in_range is False
    assert any(w.startswith("temperature = 1100 K") for w in t.warnings)


def test_state_pressure_above_range_warns_on_pressure():
    t = tpt_eng.water_state_tagged(300.0, 120.0e6)
    assert any(w.startswith("pressure = ") for w in t.warnings)
    with pytest.raises(ValueError, match="pressure"):
        tpt_eng.water_state_tagged(300.0, 120.0e6, strict=True)


def test_state_hard_errors_still_raise():
    with pytest.raises(ValueError):
        tpt_eng.water_state_tagged(200.0, 1.0e5)
    with pytest.raises(ValueError):
        tpt_eng.water_state_tagged(300.0, -1.0)


def test_air_relative_humidity_matches_untagged_in_range():
    t = air.relative_humidity_tagged(1000.0, 293.15)
    assert t.value == pytest.approx(air.relative_humidity(1000.0, 293.15))
    assert t.in_range is True
    assert t.source.startswith("ASHRAE")


def test_air_relative_humidity_out_of_range_warns_and_strict_raises():
    t = air.relative_humidity_tagged(1000.0, 500.0)
    assert t.in_range is False
    assert math.isfinite(t.value)
    with pytest.raises(ValueError, match="temperature"):
        air.relative_humidity_tagged(1000.0, 500.0, strict=True)


def test_air_saturation_out_of_range_strict_raises():
    with pytest.raises(ValueError, match="temperature"):
        air.saturation_pressure_water_tagged(600.0, strict=True)


@pytest.mark.parametrize("bad", [float("nan"), float("inf"), 0.0, -5.0])
def test_non_physical_temperature_raises_value_error(bad):
    with pytest.raises(ValueError):
        tpt_eng.water_saturation_pressure_tagged(bad)
    with pytest.raises(ValueError):
        air.relative_humidity_tagged(1000.0, bad)


def test_untagged_functions_are_unchanged():
    # The tagged additions must not change the existing float-returning API.
    assert isinstance(tpt_eng.water_saturation_pressure_pa(373.15), float)
    assert isinstance(air.relative_humidity(1000.0, 293.15), float)
