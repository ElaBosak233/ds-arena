import { afterEach, expect, it, vi } from "vitest";
import { alova } from "./alova";

vi.mock("sonner", () => ({ toast: { error: vi.fn() } }));

afterEach(() => vi.unstubAllGlobals());

it("rejects HTTP failures instead of treating them as submission results", async () => {
  const fetch = vi
    .fn()
    .mockResolvedValue(
      new Response(JSON.stringify({ msg: "captcha_error" }), { status: 400 })
    );
  vi.stubGlobal("fetch", fetch);
  await expect(alova.Post("/submissions", new FormData())).rejects.toThrow(
    "captcha_error"
  );
});

it("fetches fresh results for repeated GET requests", async () => {
  const fetch = vi
    .fn()
    .mockResolvedValueOnce(
      new Response(JSON.stringify({ data: { accuracy: null } }))
    )
    .mockResolvedValueOnce(
      new Response(JSON.stringify({ data: { accuracy: 0 } }))
    );
  vi.stubGlobal("fetch", fetch);
  await alova.Get("/submissions", { params: { id: "fresh-result" } });
  const result = await alova.Get<{ data: { accuracy: number } }>(
    "/submissions",
    {
      params: { id: "fresh-result" },
    }
  );
  expect(fetch).toHaveBeenCalledTimes(2);
  expect(result.data.accuracy).toBe(0);
});
