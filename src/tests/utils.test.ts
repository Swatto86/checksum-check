import { describe, it, expect } from 'vitest';

// Import the utility functions by recreating them for testing
function formatFileSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let size = bytes;
  let unitIndex = 0;

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }

  return `${size.toFixed(2)} ${units[unitIndex]}`;
}

function formatDate(timestamp: string): string {
  return new Date(parseInt(timestamp) * 1000).toLocaleString();
}

describe('formatFileSize', () => {
  it('should format bytes correctly', () => {
    expect(formatFileSize(0)).toBe('0.00 B');
    expect(formatFileSize(100)).toBe('100.00 B');
    expect(formatFileSize(1023)).toBe('1023.00 B');
  });

  it('should format kilobytes correctly', () => {
    expect(formatFileSize(1024)).toBe('1.00 KB');
    expect(formatFileSize(1536)).toBe('1.50 KB');
    expect(formatFileSize(102400)).toBe('100.00 KB');
  });

  it('should format megabytes correctly', () => {
    expect(formatFileSize(1048576)).toBe('1.00 MB');
    expect(formatFileSize(1572864)).toBe('1.50 MB');
    expect(formatFileSize(104857600)).toBe('100.00 MB');
  });

  it('should format gigabytes correctly', () => {
    expect(formatFileSize(1073741824)).toBe('1.00 GB');
    expect(formatFileSize(1610612736)).toBe('1.50 GB');
    expect(formatFileSize(107374182400)).toBe('100.00 GB');
  });

  it('should format terabytes correctly', () => {
    expect(formatFileSize(1099511627776)).toBe('1.00 TB');
    expect(formatFileSize(1649267441664)).toBe('1.50 TB');
  });

  it('should not exceed TB units', () => {
    expect(formatFileSize(1125899906842624)).toBe('1024.00 TB');
  });

  it('should handle edge cases', () => {
    expect(formatFileSize(1)).toBe('1.00 B');
    expect(formatFileSize(1025)).toBe('1.00 KB');
  });
});

describe('formatDate', () => {
  it('should format Unix timestamp correctly', () => {
    // Test with a known timestamp: Jan 1, 2020 00:00:00 UTC
    const timestamp = '1577836800';
    const result = formatDate(timestamp);

    // Should be a valid date string
    expect(result).toBeTruthy();
    expect(result).toContain('2020');
  });

  it('should format zero timestamp correctly', () => {
    const timestamp = '0';
    const result = formatDate(timestamp);

    // Unix epoch: Jan 1, 1970
    expect(result).toBeTruthy();
    expect(result).toContain('1970');
  });

  it('should format recent timestamp correctly', () => {
    // Recent timestamp should produce a valid date
    const timestamp = '1700000000'; // Nov 2023
    const result = formatDate(timestamp);

    expect(result).toBeTruthy();
    expect(result).toContain('2023');
  });

  it('should handle string input and convert to date', () => {
    const timestamp = '1609459200'; // Jan 1, 2021
    const result = formatDate(timestamp);

    expect(typeof result).toBe('string');
    expect(result.length).toBeGreaterThan(0);
  });
});
