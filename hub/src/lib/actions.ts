import { z } from "zod";

export const actionGuideSchema = z.object({
  id: z.string().regex(/^[a-z0-9]+(?:-[a-z0-9]+)*$/),
  title: z.string().min(1),
  summary: z.string().min(1),
  when: z.string().min(1),
  steps: z.array(z.string().min(1)).min(1),
  produces: z.array(z.string().min(1)).min(1),
  limits: z.string().min(1),
}).strict();

export type ActionGuide = z.infer<typeof actionGuideSchema>;

export async function loadActionGuides(): Promise<ActionGuide[]> {
  const module = await import("../data/actions.json");
  return z.array(actionGuideSchema).parse(module.default);
}
