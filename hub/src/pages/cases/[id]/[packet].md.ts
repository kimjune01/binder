import type { APIRoute } from "astro";
import { loadCases } from "../../../lib/cases";

const packets = ["control", "curated", "questions", "answer-key"] as const;
const packetFiles = import.meta.glob("../../../data/packets/**/*.md", {
  eager: true,
  import: "default",
  query: "?raw",
}) as Record<string, string>;

export async function getStaticPaths() {
  const cases = await loadCases();
  return cases.flatMap((item) => packets.map((packet) => ({
    params: { id: item.id, packet },
    props: { id: item.id, packet },
  })));
}

export const GET: APIRoute = async ({ props }) => {
  const key = `../../../data/packets/${props.id}/${props.packet}.md`;
  const markdown = packetFiles[key];
  if (!markdown) return new Response("Packet not found", { status: 404 });
  return new Response(markdown, {
    headers: { "Content-Type": "text/markdown; charset=utf-8" },
  });
};
