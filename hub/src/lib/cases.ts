import { z } from "zod";

export const caseClasses = [
  "authorization-replay",
  "upgrade-migration",
  "build-deployment-identity",
  "invariant-scope",
  "postmortem-remediation",
] as const;

const artifactSchema = z.object({
  id: z.string().min(1),
  kind: z.string().min(1),
  title: z.string().min(1),
  url: z.string().url(),
  revision: z.string().min(1).optional(),
}).strict();

const edgeSchema = z.object({
  id: z.string().min(1),
  relation: z.string().min(1),
  from: z.string().min(1),
  to: z.string().min(1),
  status: z.enum(["sourced", "inferred", "missing"]),
  citations: z.array(z.string().min(1)),
}).strict();

export const publicCaseSchema = z.object({
  schema_version: z.literal(1),
  id: z.string().regex(/^[a-z0-9]+(?:-[a-z0-9]+)*$/),
  title: z.string().min(1),
  ecosystem: z.string().min(1),
  class: z.enum(caseClasses),
  summary: z.string().min(1),
  case_study: z.object({
    situation: z.string().min(1),
    turning_point: z.string().min(1),
    lesson: z.string().min(1),
  }).strict(),
  claim: z.string().min(1),
  evidence_boundary: z.string().min(1),
  decision: z.object({
    question: z.string().min(1),
    expected: z.string().min(1),
  }).strict(),
  artifacts: z.array(artifactSchema).min(1),
  edges: z.array(edgeSchema).min(1),
  missing_edges: z.array(z.string().min(1)).min(1),
  packets: z.object({
    control: z.string().min(1),
    curated: z.string().min(1),
    questions: z.string().min(1),
    answer_key: z.string().min(1),
  }).strict(),
}).strict();

export type PublicCase = z.infer<typeof publicCaseSchema>;

const fixtureModules = import.meta.glob("../data/cases/*.json", {
  eager: true,
  import: "default",
});

export async function loadCases(): Promise<PublicCase[]> {
  return Object.entries(fixtureModules)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([, value]) => publicCaseSchema.parse(value));
}

export async function findCase(id: string): Promise<PublicCase | undefined> {
  return (await loadCases()).find((item) => item.id === id);
}
