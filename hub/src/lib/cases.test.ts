import { describe, expect, it } from "vitest";

import { caseClasses, loadCases } from "./cases";
import { loadActionGuides } from "./actions";

describe("public case corpus", () => {
  it("contains one valid fixture for every roadmap class", async () => {
    const cases = await loadCases();

    expect(cases).toHaveLength(5);
    expect(new Set(cases.map((item) => item.class))).toEqual(new Set(caseClasses));
  });

  it("cites facts and exposes gaps", async () => {
    const cases = await loadCases();

    for (const item of cases) {
      expect(item.decision.question).not.toBe("");
      expect(item.decision.expected).not.toBe("");
      expect(item.case_study.situation).not.toBe("");
      expect(item.case_study.turning_point).not.toBe("");
      expect(item.case_study.lesson).not.toBe("");
      expect(item.recommendations.length).toBeGreaterThan(0);
      for (const recommendation of item.recommendations) {
        expect(recommendation.action).not.toBe("");
        expect(recommendation.because).not.toBe("");
      }
      expect(item.missing_edges.length).toBeGreaterThan(0);
      for (const edge of item.edges) {
        if (edge.status === "sourced") expect(edge.citations.length).toBeGreaterThan(0);
      }
    }
  });

  it("uses unique artifact IDs and valid citation references", async () => {
    const cases = await loadCases();

    for (const item of cases) {
      const ids = item.artifacts.map((artifact) => artifact.id);
      expect(new Set(ids).size).toBe(ids.length);
      for (const edge of item.edges) {
        for (const citation of edge.citations) expect(ids).toContain(citation);
      }
    }
  });

  it("links every recommendation to a generic action guide", async () => {
    const cases = await loadCases();
    const guideIds = new Set((await loadActionGuides()).map((guide) => guide.id));

    for (const item of cases) {
      for (const recommendation of item.recommendations) {
        expect(guideIds).toContain(recommendation.guide);
      }
    }
  });
});
