import pytest

import tpt_eng


def test_saturation_pressure_at_100c_is_about_one_atmosphere():
    # 373.15 K is 100 °C; p_sat is about 101.3 kPa.
    assert tpt_eng.water_saturation_pressure_pa(373.15) == pytest.approx(
        101_325, rel=1e-3
    )


def test_saturation_temperature_round_trips():
    p = tpt_eng.water_saturation_pressure_pa(450.0)
    assert tpt_eng.water_saturation_temperature_k(p) == pytest.approx(450.0, rel=1e-6)


def test_state_matches_iapws_if97_reference_point():
    # IAPWS-IF97 Region 1 reference values at 300 K, 3 MPa.
    s = tpt_eng.water_state(300.0, 3.0e6)
    assert s.region == "liquid"
    assert s.specific_volume_m3_per_kg == pytest.approx(0.00100215168, rel=1e-5)
    assert s.enthalpy_kj_per_kg == pytest.approx(115.331273, rel=1e-5)
    assert s.entropy_kj_per_kg_k == pytest.approx(0.392294792, rel=1e-5)
    assert s.density_kg_per_m3 == pytest.approx(1.0 / 0.00100215168, rel=1e-5)


def test_state_vapour_region():
    assert tpt_eng.water_state(500.0, 1.0e5).region == "vapour"


def test_temperature_below_triple_point_raises_value_error():
    with pytest.raises(ValueError):
        tpt_eng.water_state(200.0, 1.0e5)


def test_negative_pressure_raises_value_error():
    with pytest.raises(ValueError):
        tpt_eng.water_state(300.0, -1.0)


def test_saturation_pressure_outside_range_raises_value_error():
    with pytest.raises(ValueError):
        tpt_eng.water_saturation_pressure_pa(700.0)


def test_saturation_temperature_outside_range_raises_value_error():
    with pytest.raises(ValueError):
        tpt_eng.water_saturation_temperature_k(1.0e8)


def test_region3_raises_not_implemented():
    # 640 K at 18 MPa is below p_sat (20.3 MPa) and above the B23 boundary
    # (16.5 MPa), so it falls in Region 3.
    with pytest.raises(NotImplementedError):
        tpt_eng.water_state(640.0, 18.0e6)
