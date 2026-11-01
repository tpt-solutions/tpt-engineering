import pytest

from tpt_eng.structural import SimplySupportedBeam, section_utilization


def test_point_load_at_midspan():
    # P = 10 kN at the centre of a 4 m span: R = P/2 each, M_max = P·L/4.
    beam = SimplySupportedBeam(4.0)
    beam.add_point_load(2.0, 10_000.0)
    assert beam.reaction_a_n() == pytest.approx(5_000.0)
    assert beam.reaction_b_n() == pytest.approx(5_000.0)
    assert beam.max_bending_moment_nm() == pytest.approx(10_000.0, rel=1e-3)


def test_uniform_load_maximum_moment():
    # Total 8 kN over 4 m: w = 2 kN/m, M_max = w·L²/8 = 4 kN·m at midspan.
    beam = SimplySupportedBeam(4.0)
    beam.add_uniform_load(0.0, 4.0, 8_000.0)
    assert beam.reaction_a_n() == pytest.approx(4_000.0)
    assert beam.max_bending_moment_nm(resolution=400) == pytest.approx(4_000.0, rel=1e-3)


def test_shear_and_moment_sign_convention_at_point():
    beam = SimplySupportedBeam(4.0)
    beam.add_point_load(2.0, 10_000.0)
    assert beam.shear_at_n(1.0) == pytest.approx(5_000.0)
    assert beam.moment_at_nm(1.0) == pytest.approx(5_000.0)


def test_load_outside_span_raises_value_error():
    beam = SimplySupportedBeam(4.0)
    with pytest.raises(ValueError):
        beam.add_point_load(5.0, 1_000.0)


def test_non_positive_span_raises_value_error():
    with pytest.raises(ValueError):
        SimplySupportedBeam(0.0)


def test_section_utilization_is_demand_over_capacity():
    # |M| / (Z · σ_allow) = 10 kN·m / (1e-4 m³ · 200 MPa) = 0.5
    assert section_utilization(10_000.0, 1.0e-4, 200.0e6) == pytest.approx(0.5)


def test_section_utilization_rejects_zero_modulus():
    with pytest.raises(ValueError):
        section_utilization(10_000.0, 0.0, 200.0e6)
