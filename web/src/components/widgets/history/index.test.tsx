import { act, cleanup, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { Submission } from "@/models/submission";
import { useSharedStore } from "@/storages/shared";
import { alova } from "@/utils/alova";
import { History } from "./index";

vi.mock("@/utils/alova", () => ({ alova: { Get: vi.fn() } }));

const pending: Submission = {
  id: "submission-1",
  filename: "input.csv",
  accuracy: null,
  note: null,
};

beforeEach(() => {
  vi.useFakeTimers();
  useSharedStore.setState({ history: [pending] });
});

afterEach(() => {
  cleanup();
  vi.useRealTimers();
});

describe("submission history", () => {
  it("keeps polling pending results and completes a zero score", async () => {
    vi.mocked(alova.Get)
      .mockResolvedValueOnce({ data: pending })
      .mockResolvedValueOnce({
        data: { ...pending, accuracy: 0, note: "complete" },
      });
    render(<History />);

    await act(() => vi.advanceTimersByTimeAsync(1000));
    expect(alova.Get).toHaveBeenCalledTimes(1);
    await act(() => vi.advanceTimersByTimeAsync(1000));
    expect(screen.getByText("0%")).toBeTruthy();
    await act(() => vi.advanceTimersByTimeAsync(3000));
    expect(alova.Get).toHaveBeenCalledTimes(2);
  });

  it("retries after a failed poll", async () => {
    vi.mocked(alova.Get)
      .mockRejectedValueOnce(new Error("offline"))
      .mockResolvedValueOnce({ data: { ...pending, accuracy: 0.5 } });
    render(<History />);

    await act(() => vi.advanceTimersByTimeAsync(2000));
    expect(screen.getByText("50%")).toBeTruthy();
  });

  it("stops polling when unmounted", async () => {
    const { unmount } = render(<History />);
    unmount();
    await act(() => vi.advanceTimersByTimeAsync(2000));
    expect(alova.Get).not.toHaveBeenCalled();
  });
});
