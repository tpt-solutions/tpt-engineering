import pytest

from tpt_eng.fuels import BlendedFuel, Fuel


def test_heating_values():
    assert Fuel.Methane.lhv_mj_kg() == pytest.approx(50.0)
    assert Fuel.Hydrogen.lhv_mj_kg() == pytest.approx(119.96)
    assert Fuel.Hydrogen.hhv_mj_kg() == pytest.approx(141.8)


def test_density_methane_and_diesel():
    assert Fuel.Methane.density_kg_m3() == pytest.approx(0.678)
    assert Fuel.Diesel.density_kg_m3() == pytest.approx(832.0)


def test_stoichiometric_air_fuel_ratios():
    # CH4: 2 mol O2 per mol fuel gives 64/16 = 4 kg O2/kg, over 0.232 kg O2/kg air.
    assert Fuel.Methane.stoichiometric_air_fuel_ratio() == pytest.approx(17.24, abs=0.01)
    assert Fuel.Hydrogen.stoichiometric_air_fuel_ratio() == pytest.approx(34.48, abs=0.01)
    assert Fuel.Diesel.stoichiometric_air_fuel_ratio() == pytest.approx(14.66, abs=0.01)


def test_co2_factor_for_methane():
    # 44/16 kg CO2 per kg CH4, over 50 MJ/kg.
    assert Fuel.Methane.co2_kg_per_mj() == pytest.approx(0.055)


def test_pure_blend_matches_end_members():
    natural_gas = BlendedFuel(0.0)
    assert natural_gas.lhv_mj_kg == pytest.approx(Fuel.NaturalGas.lhv_mj_kg())
    assert natural_gas.molar_mass_kg_per_kmol == pytest.approx(16.0)

    hydrogen = BlendedFuel(1.0)
    assert hydrogen.lhv_mj_kg == pytest.approx(Fuel.Hydrogen.lhv_mj_kg())
    assert hydrogen.co2_kg_per_mj() == pytest.approx(0.0)


def test_blend_properties_are_between_end_members():
    blend = BlendedFuel(0.2)
    assert Fuel.Hydrogen.lhv_mj_kg() > blend.lhv_mj_kg > Fuel.Methane.lhv_mj_kg()
    assert Fuel.Methane.co2_kg_per_mj() > blend.co2_kg_per_mj() > 0.0
    assert blend.h2_fraction == pytest.approx(0.2)


@pytest.mark.parametrize("fraction", [-0.1, 1.5, float("nan")])
def test_blend_fraction_outside_unit_interval_raises_value_error(fraction):
    with pytest.raises(ValueError):
        BlendedFuel(fraction)
