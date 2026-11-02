import pytest

from tpt_eng import air


def test_saturation_pressure_at_20c_matches_ashrae():
    # ASHRAE Fundamentals: about 2339 Pa at 20 °C.
    assert air.saturation_pressure_water_pa(293.15) == pytest.approx(2339.0, rel=2e-3)


def test_humidity_ratio_formula():
    assert air.humidity_ratio(1000.0, 101_325.0) == pytest.approx(
        0.621945 * 1000.0 / (101_325.0 - 1000.0)
    )


def test_vapour_pressure_round_trips_humidity_ratio():
    w = air.humidity_ratio(1500.0, 101_325.0)
    assert air.vapour_pressure_from_ratio_pa(w, 101_325.0) == pytest.approx(1500.0)


def test_relative_humidity_is_one_at_saturation():
    p_sat = air.saturation_pressure_water_pa(293.15)
    assert air.relative_humidity(p_sat, 293.15) == pytest.approx(1.0)


def test_dew_point_of_saturated_vapour_is_its_temperature():
    p_sat = air.saturation_pressure_water_pa(293.15)
    assert air.dew_point_k(p_sat) == pytest.approx(293.15, abs=0.01)


def test_moist_air_enthalpy_dry_and_humid():
    # h = 1.006·Tc + w·(2501 + 1.86·Tc), with Tc = 20 °C.
    assert air.moist_air_enthalpy_kj_per_kg(293.15, 0.0) == pytest.approx(20.12)
    assert air.moist_air_enthalpy_kj_per_kg(293.15, 0.01) == pytest.approx(
        20.12 + 0.01 * (2501.0 + 1.86 * 20.0)
    )


def test_vapour_pressure_equal_to_total_raises_value_error():
    with pytest.raises(ValueError):
        air.humidity_ratio(101_325.0, 101_325.0)


def test_negative_humidity_ratio_raises_value_error():
    with pytest.raises(ValueError):
        air.vapour_pressure_from_ratio_pa(-0.1, 101_325.0)


def test_temperature_outside_range_raises_value_error():
    with pytest.raises(ValueError):
        air.saturation_pressure_water_pa(500.0)
    with pytest.raises(ValueError):
        air.moist_air_enthalpy_kj_per_kg(100.0, 0.0)
