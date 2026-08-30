export type InlineToken = { kind: "text" | "code"; value: string };

export function tokenizeInlineCode(text: string): InlineToken[] {
  return text.split(/(`[^`]+`)/g).filter(Boolean).map((part) =>
    part.startsWith("`") && part.endsWith("`")
      ? { kind: "code", value: part.slice(1, -1) }
      : { kind: "text", value: part },
  );
}
