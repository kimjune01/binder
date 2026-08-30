import { readFile } from "node:fs/promises";

import { describe, expect, it } from "vitest";

describe("home page framing", () => {
  it("leads with the complete agreement and accurately labels the public cases", async () => {
    const page = await readFile(new URL("../pages/index.astro", import.meta.url), "utf8");

    expect(page).toContain("Put the whole agreement in one place.");
    expect(page).toContain("contract, parameters, dependencies, evidence, reviews, signatures, and deployment record");
    expect(page).toContain("See what belongs in the binder");
    expect(page).toContain("Five public case studies");
  });

  it("uses direct case-study headings without label repetition", async () => {
    const page = await readFile(new URL("../pages/cases/[id]/index.astro", import.meta.url), "utf8");

    expect(page).toContain("<h2>Situation</h2>");
    expect(page).toContain("<h2>Response</h2>");
    expect(page).not.toContain("The situation");
    expect(page).not.toContain("What happened");
    expect(page).not.toContain("The turning point");
    expect(page).not.toContain("What changed");
  });

  it("offers a bounded end-to-end agreement package demo", async () => {
    const home = await readFile(new URL("../pages/index.astro", import.meta.url), "utf8");
    const demo = await readFile(new URL("../pages/agreements/escrow.astro", import.meta.url), "utf8");

    expect(home).toContain("/agreements/escrow/");
    expect(demo).toContain("Assemble a two-party escrow");
    expect(demo).toContain("Review as buyer");
    expect(demo).toContain("Review as recipient");
    expect(demo).toContain("Export package");
    expect(demo).toContain("Binder does not sign or deploy this package");
  });
});
