# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
"""The shared golden vectors, checked against the Python bindings."""

from pathlib import Path

import pitch_math
import pytest
from evidence_kernel.vectors import check_dir

VECTORS = Path(__file__).resolve().parents[3] / "vectors"


def test_golden_vectors():
    assert check_dir(VECTORS, pitch_math) > 0


def test_tuples_are_lists_and_errors_raise_value_error():
    assert pitch_math.octave_fold(1 / 600, 220) == [18, 436.9066666666667]
    assert isinstance(pitch_math.nearest_note(440.0), list)
    with pytest.raises(ValueError, match="hz must be a finite number greater than 0"):
        pitch_math.nearest_note(float("nan"))
