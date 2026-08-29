import { z } from "zod";

export const validationConditions = ["control", "curated"] as const;
export type ValidationCondition = (typeof validationConditions)[number];

export const validationResultSchema = z.object({
  schema_version: z.literal(1),
  case_id: z.string().regex(/^[a-z0-9]+(?:-[a-z0-9]+)*$/),
  condition: z.enum(validationConditions),
  started_at: z.string().datetime(),
  completed_at: z.string().datetime(),
  elapsed_seconds: z.number().int().nonnegative(),
  sources_opened: z.number().int().nonnegative(),
  confidence: z.number().int().min(1).max(5),
  answers: z.array(z.string().min(1)).length(5),
}).strict();

export type ValidationResult = z.infer<typeof validationResultSchema>;

export function parseQuestions(markdown: string): string[] {
  return [...markdown.matchAll(/^\d+\.\s+(.+)$/gm)].map((match) => match[1].trim());
}

export function chooseCondition(randomValue = Math.random()): ValidationCondition {
  return randomValue < 0.5 ? "control" : "curated";
}
