export const theme = {
  colors: {
    bg: {
      primary: '#0a0a0a',
      secondary: '#141414',
      tertiary: '#1c1c1c',
      elevated: '#242424',
      hover: '#2a2a2a',
      active: '#333333',
    },
    text: {
      primary: '#ffffff',
      secondary: '#a0a0a0',
      tertiary: '#666666',
      muted: '#444444',
    },
    accent: {
      primary: '#3b82f6',
      primaryHover: '#2563eb',
      success: '#22c55e',
      warning: '#f59e0b',
      danger: '#ef4444',
      info: '#06b6d4',
    },
    priority: {
      high: '#ef4444',
      medium: '#f59e0b',
      low: '#22c55e',
    },
    border: {
      primary: '#2a2a2a',
      secondary: '#333333',
      focus: '#3b82f6',
    },
  },
  spacing: {
    xs: '4px',
    sm: '8px',
    md: '16px',
    lg: '24px',
    xl: '32px',
    '2xl': '48px',
  },
  radius: {
    sm: '4px',
    md: '8px',
    lg: '12px',
    xl: '16px',
    full: '9999px',
  },
  fontSize: {
    xs: '0.75rem',
    sm: '0.875rem',
    md: '1rem',
    lg: '1.125rem',
    xl: '1.25rem',
    '2xl': '1.5rem',
    '3xl': '2rem',
  },
  fontFamily: "'Commit Mono', ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
  transition: {
    fast: '150ms ease',
    normal: '250ms ease',
    slow: '350ms ease',
  },
  layout: {
    sidebarWidth: '240px',
    headerHeight: '60px',
  },
} as const;

export type Theme = typeof theme;

export const getPriorityColor = (priority: number): string => {
  if (priority >= 0.7) return theme.colors.priority.high;
  if (priority >= 0.4) return theme.colors.priority.medium;
  return theme.colors.priority.low;
};

export const getPriorityLabel = (priority: number): string => {
  if (priority >= 0.7) return 'High';
  if (priority >= 0.4) return 'Medium';
  return 'Low';
};
