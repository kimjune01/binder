import { z } from "zod";

const receiptSchema = z.object({
  kind: z.enum(["source", "deployment", "audit", "documentation"]),
  claim: z.string().min(1),
  url: z.string().url(),
  revision: z.string().min(1).optional(),
  boundary: z.string().min(1),
}).strict();

export const demoContractSchema = z.object({
  id: z.string().regex(/^[a-z0-9]+(?:-[a-z0-9]+)*$/),
  title: z.string().min(1),
  provider: z.string().min(1),
  use_case: z.string().min(1),
  summary: z.string().min(1),
  status: z.enum(["deployed", "source-template"]),
  runtime: z.string().min(1),
  deployment: z.object({ chain: z.string().min(1), address: z.string().min(1), explorer: z.string().url() }).strict().optional(),
  interface: z.object({ name: z.string().min(1), functions: z.array(z.string().min(1)).min(1) }).strict(),
  parameters: z.array(z.string().min(1)).min(1),
  receipts: z.array(receiptSchema).min(1),
  gaps: z.array(z.string().min(1)).min(1),
  demo_url: z.string().min(1).optional(),
}).strict();

export type DemoContract = z.infer<typeof demoContractSchema>;

export async function loadDemoContracts(): Promise<DemoContract[]> {
  const module = await import("../data/contracts.json");
  return z.array(demoContractSchema).parse(module.default);
}
