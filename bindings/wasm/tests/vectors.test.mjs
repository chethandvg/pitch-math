// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
// The shared golden vectors and the whole-number arguments, checked against the WebAssembly build.

import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { test } from "node:test";
import { checkDir } from "golden-vectors";

const impl = createRequire(import.meta.url)("../pkg/node/pitch_math_wasm.js");

test("golden vectors", () => {
  assert.ok(checkDir(new URL("../../../vectors/", import.meta.url), impl) > 0);
});

test("tuples are arrays of numbers and errors throw", () => {
  assert.deepEqual(impl.octave_fold(1 / 600, 220), [18, 436.9066666666667]);
  assert.equal(typeof impl.nearest_note(440)[0], "number");
  assert.throws(() => impl.nearest_note(Number.NaN), /hz must be a finite number greater than 0/);
});

test("whole-number arguments are whole numbers of at least -2^63 and less than 2^63, never truncated or wrapped", () => {
  const big = 2 ** 63;
  const top = big - 1024; // the largest number below 2^63; 2 ** 63 - 1 rounds to 2 ** 63

  // Accepted at the extremes, with the results Rust and Python give for the same i64.
  assert.equal(impl.note_name(-big), "E-768614336404564652");
  assert.equal(impl.note_name(top), "E768614336404564564");
  assert.equal(impl.note_name(-0), "C-1");
  assert.equal(impl.octave_transpose(1, 1023), 2 ** 1023);
  assert.equal(impl.octave_transpose(1, -1074), Number.MIN_VALUE);
  assert.equal(impl.harmonic(1e-300, top), 9.223372036854775e-282);
  assert.equal(impl.harmonic(1, 2 ** 32 + 2), 2 ** 32 + 2); // not wrapped to 2
  assert.equal(impl.is_octave_harmonic(2 ** 62), true);
  assert.equal(impl.is_octave_harmonic(top), false);

  // Accepted, then rejected by the core with the same errors as in Rust and Python.
  assert.throws(() => impl.octave_transpose(440, top), /^Error: result must be a finite number greater than 0, got inf$/);
  assert.throws(() => impl.octave_transpose(440, -big), /^Error: result must be a finite number greater than 0, got 0$/);
  assert.throws(() => impl.harmonic(440, -big), /^Error: n must be a whole number of at least 1, got -9223372036854776000$/);

  // Rejected before the core: fractions, NaN, infinities and the numbers just outside the range.
  assert.throws(() => impl.harmonic(440, 2.5), /^Error: n must be a whole number of at least -2\^63 and less than 2\^63, got 2\.5$/);
  const calls = {
    midi: [(x) => impl.note_name(x)],
    n: [(x) => impl.octave_transpose(440, x), (x) => impl.harmonic(440, x), (x) => impl.is_octave_harmonic(x)],
  };
  for (const [name, fns] of Object.entries(calls)) {
    const message = new RegExp(`^Error: ${name} must be a whole number of at least -2\\^63 and less than 2\\^63, got `);
    for (const call of fns) {
      for (const x of [2.5, -0.5, 2 ** 32 + 2.5, Number.NaN, Infinity, -Infinity, big, -big - 2048]) {
        assert.throws(() => call(x), message, `${name} = ${x}`);
      }
    }
  }
});
