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
});
