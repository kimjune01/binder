import { describe, expect, it } from "vitest";

import {
  parseQuestions,
  validationResultSchema,
  chooseCondition,
} from "./validation";

describe("validation task", () => {
  it("extracts numbered questions from a packet", () => {
    expect(parseQuestions("# Questions\n\n1. First?\n2. Second?\n")).toEqual([
      "First?",
      "Second?",
    ]);
  });

  it("rejects incomplete and implausible results", () => {
    const base = {
      schema_version: 1,
      case_id: "solana-stale-cancel",
      condition: "control",
      started_at: "2026-08-29T12:00:00.000Z",
      completed_at: "2026-08-29T12:05:00.000Z",
      elapsed_seconds: 300,
      sources_opened: 2,
      confidence: 3,
      answers: ["a", "b", "c", "d", "e"],
    };

    expect(validationResultSchema.safeParse(base).success).toBe(true);
    expect(validationResultSchema.safeParse({ ...base, answers: ["a"] }).success).toBe(false);
    expect(validationResultSchema.safeParse({ ...base, confidence: 6 }).success).toBe(false);
    expect(validationResultSchema.safeParse({ ...base, elapsed_seconds: -1 }).success).toBe(false);
  });

  it("assigns either experimental condition from a random draw", () => {
    expect(chooseCondition(0.2)).toBe("control");
    expect(chooseCondition(0.8)).toBe("curated");
  });
});
