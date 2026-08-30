import type { APIRoute, GetStaticPaths } from "astro";
import { loadDemoContracts } from "../../lib/contracts";

export const getStaticPaths = (async () => (await loadDemoContracts()).map((contract) => ({ params: { id: contract.id }, props: { contract } }))) satisfies GetStaticPaths;

export const GET: APIRoute = ({ props }) => new Response(JSON.stringify(props.contract, null, 2), { headers: { "Content-Type": "application/json; charset=utf-8" } });
