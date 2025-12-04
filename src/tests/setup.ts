import "@testing-library/jest-dom";
import { afterEach } from "vitest";
import { cleanup } from "@testing-library/react";

// Cleanup after each test
afterEach(() => {
  cleanup();
});

// Mock Tauri API
(globalThis as any).__TAURI__ = {
  invoke: async () => ({}),
  convertFileSrc: (src: string) => src,
};

// Mock navigator.clipboard
Object.assign(navigator, {
  clipboard: {
    writeText: async () => {
      return Promise.resolve();
    },
    readText: async () => {
      return Promise.resolve("");
    },
  },
});

// Mock window.matchMedia
Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: (query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: () => {},
    removeListener: () => {},
    addEventListener: () => {},
    removeEventListener: () => {},
    dispatchEvent: () => {},
  }),
});
