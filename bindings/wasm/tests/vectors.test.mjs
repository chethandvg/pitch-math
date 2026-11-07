// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Chethan Dandebala Venkatarama
// The shared golden vectors, checked against the WebAssembly build.

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
