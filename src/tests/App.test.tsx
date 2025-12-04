import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";

// Mock all Tauri modules
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}));

vi.mock("@tauri-apps/api/window", () => ({
  Window: vi.fn(() => ({
    setSize: vi.fn(),
    hide: vi.fn(),
    show: vi.fn(),
    setFocus: vi.fn(),
  })),
  LogicalSize: vi.fn((width, height) => ({ width, height })),
}));

vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: vi.fn(() => ({
    onDragDropEvent: vi.fn(() => Promise.resolve(() => {})),
  })),
}));

import App from "../App";

describe("App Component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("Initial Render", () => {
    it("should render the application title", () => {
      render(<App />);
      expect(screen.getByText(/# Checksum Check/i)).toBeInTheDocument();
    });

    it("should render the file selection area", () => {
      render(<App />);
      expect(screen.getByText(/Drop your file here/i)).toBeInTheDocument();
      expect(screen.getByText(/or click to browse/i)).toBeInTheDocument();
    });

    it("should render supporting text", () => {
      render(<App />);
      expect(
        screen.getByText(/Select or drop a file to calculate its checksums/i),
      ).toBeInTheDocument();
      expect(
        screen.getByText(/Supports MD5, SHA1, SHA256, and SHA512/i),
      ).toBeInTheDocument();
    });

    it("should render privacy notice", () => {
      render(<App />);
      expect(
        screen.getByText(
          /All calculations are performed locally on your device/i,
        ),
      ).toBeInTheDocument();
    });

    it("should render theme toggle button", () => {
      render(<App />);
      const buttons = screen.getAllByRole("button");
      expect(buttons.length).toBeGreaterThan(0);
    });

    it("should not show file information initially", () => {
      render(<App />);
      expect(screen.queryByText(/File Information/i)).not.toBeInTheDocument();
    });

    it("should not show checksums section initially", () => {
      render(<App />);
      expect(screen.queryByText(/^Checksums$/i)).not.toBeInTheDocument();
    });

    it("should set dark theme by default", () => {
      render(<App />);
      expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
    });
  });

  describe("UI Elements", () => {
    it("should have clickable drop zone", () => {
      render(<App />);
      const dropZone = screen.getByText(/Drop your file here/i).closest("div");
      const parentDiv = dropZone?.parentElement;
      expect(parentDiv).toHaveClass("cursor-pointer");
    });

    it("should have proper styling for drop zone", () => {
      render(<App />);
      const dropZone = screen.getByText(/Drop your file here/i).closest("div");
      const parentDiv = dropZone?.parentElement;
      expect(parentDiv).toHaveClass("border-2", "border-dashed");
    });

    it("should display file upload icon", () => {
      render(<App />);
      const svg = screen
        .getByText(/Drop your file here/i)
        .closest("div")
        ?.querySelector("svg");
      expect(svg).toBeInTheDocument();
    });
  });

  describe("Accessibility", () => {
    it("should have proper button roles", () => {
      render(<App />);
      const buttons = screen.getAllByRole("button");
      expect(buttons.length).toBeGreaterThan(0);
    });

    it("should have descriptive text for users", () => {
      render(<App />);
      expect(screen.getByText(/Drop your file here/i)).toBeInTheDocument();
      expect(screen.getByText(/or click to browse/i)).toBeInTheDocument();
    });
  });
});
