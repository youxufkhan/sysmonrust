// Percentage thresholds for metric color coding
export const THRESHOLDS = {
  GREEN_MAX: 60,  // 0-60% = green
  YELLOW_MAX: 85, // 60-85% = yellow
  RED_MIN: 85,    // 85-100% = red
} as const;

// Color hex codes
export const COLORS = {
  GREEN: '#22c55e',
  YELLOW: '#eab308',
  RED: '#ef4444',
} as const;

/**
 * Returns a color hex code based on percentage thresholds.
 * - 0-60%: Green (#22c55e)
 * - 60-85%: Yellow (#eab308)
 * - 85-100%: Red (#ef4444)
 */
export function getThresholdColor(percent: number): string {
  if (percent <= THRESHOLDS.GREEN_MAX) {
    return COLORS.GREEN;
  }
  if (percent <= THRESHOLDS.YELLOW_MAX) {
    return COLORS.YELLOW;
  }
  return COLORS.RED;
}
