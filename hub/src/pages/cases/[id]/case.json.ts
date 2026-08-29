import type { APIRoute } from "astro";
import { loadCases } from "../../../lib/cases";

export async function getStaticPaths() {
  return (await loadCases()).map((item) => ({ params: { id: item.id }, props: { item } }));
}

export const GET: APIRoute = ({ props }) => new Response(JSON.stringify(props.item, null, 2), {
  headers: { "Content-Type": "application/json; charset=utf-8" },
});
