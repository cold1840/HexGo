import assert from "node:assert/strict";
import test from "node:test";

import {
    calculateTransferRate,
    estimateRemainingSeconds,
    formatBytes,
    formatDuration,
    normalizeProgress,
} from "./loader.mjs";

test("formatBytes selects readable binary units", () => {
    assert.equal(formatBytes(0), "0 B");
    assert.equal(formatBytes(512), "512 B");
    assert.equal(formatBytes(1_536), "1.5 KiB");
    assert.equal(formatBytes(12 * 1024 * 1024), "12.0 MiB");
});

test("formatDuration rounds up and preserves minutes", () => {
    assert.equal(formatDuration(0), "0 秒");
    assert.equal(formatDuration(8.2), "9 秒");
    assert.equal(formatDuration(125), "2 分 5 秒");
    assert.equal(formatDuration(Number.NaN), "未知");
});

test("calculateTransferRate uses the sample window endpoints", () => {
    const rate = calculateTransferRate([
        { time: 1_000, bytes: 1_024 },
        { time: 3_000, bytes: 5_120 },
    ]);
    assert.equal(rate, 2_048);
    assert.equal(calculateTransferRate([{ time: 0, bytes: 0 }]), 0);
});

test("estimateRemainingSeconds handles known and unknown totals", () => {
    assert.equal(estimateRemainingSeconds(4_000, 10_000, 2_000), 3);
    assert.equal(estimateRemainingSeconds(4_000, 0, 2_000), null);
    assert.equal(estimateRemainingSeconds(4_000, 10_000, 0), null);
});

test("normalizeProgress prefers the final response size", () => {
    assert.deepEqual(normalizeProgress(25, 100, 50), { current: 25, total: 50 });
    assert.deepEqual(normalizeProgress(100, 100, 50), { current: 50, total: 50 });
    assert.deepEqual(normalizeProgress(25, 100, 0), { current: 25, total: 100 });
});
