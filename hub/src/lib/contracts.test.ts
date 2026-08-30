import { describe, expect, it } from "vitest";

import { loadDemoContracts } from "./contracts";

describe("demo contract catalog", () => {
  it("contains a handful of agent-inspectable contracts", async () => {
    const contracts = await loadDemoContracts();
    expect(contracts).toHaveLength(4);

    for (const contract of contracts) {
      expect(contract.receipts.some((receipt) => receipt.kind === "source")).toBe(true);
      expect(contract.interface.functions.length).toBeGreaterThan(0);
      expect(contract.parameters.length).toBeGreaterThan(0);
      expect(contract.gaps.length).toBeGreaterThan(0);
      if (contract.status === "deployed") {
        expect(contract.deployment?.address).toMatch(/^(0x[a-fA-F0-9]{40}|[1-9A-HJ-NP-Za-km-z]{32,44})$/);
        expect(contract.receipts.some((receipt) => receipt.kind === "deployment")).toBe(true);
      }
    }
  });
});
