import sys

import pytest

from tpt_eng.controls import Pid


def test_submodule_is_importable_by_dotted_name():
    import tpt_eng.controls

    assert sys.modules["tpt_eng.controls"] is tpt_eng.controls


def test_pid_drives_first_order_plant_to_setpoint():
    # Same plant as the tpt-eng-controls doc example: gain 1, time constant 1 s.
    pid = Pid(2.0, 1.0, 0.0)
    pid.set_setpoint(10.0)
    y = 0.0
    for _ in range(600):
        u = pid.update(y, 0.01)
        y += 0.01 * u
    assert y == pytest.approx(10.0, abs=0.5)


def test_output_limits_are_respected():
    pid = Pid(100.0, 0.0, 0.0, output_min=-1.0, output_max=1.0)
    pid.set_setpoint(50.0)
    assert pid.update(0.0, 0.01) == pytest.approx(1.0)
    assert pid.update(100.0, 0.01) == pytest.approx(-1.0)


def test_integral_accumulates():
    pid = Pid(0.0, 1.0, 0.0)
    pid.set_setpoint(1.0)
    pid.update(0.0, 0.5)
    assert pid.integral == pytest.approx(0.5)


@pytest.mark.parametrize("dt", [0.0, -0.01, float("nan")])
def test_non_positive_dt_raises_value_error(dt):
    with pytest.raises(ValueError):
        Pid(1.0).update(0.0, dt)


def test_only_one_output_limit_raises_value_error():
    with pytest.raises(ValueError):
        Pid(1.0, output_max=1.0)


def test_inverted_output_limits_raise_value_error():
    with pytest.raises(ValueError):
        Pid(1.0, output_min=2.0, output_max=-2.0)
