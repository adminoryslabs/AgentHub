/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        surface: '#101419',
        'surface-lowest': '#080b10',
        'surface-low': '#0d1117',
        'surface-high': '#141920',
        'surface-active': '#1a2030',
        'surface-hover': '#202840',
        'surface-elevated': '#283050',
        primary: '#00e475',
        'primary-dim': '#00e47520',
        secondary: '#adc6ff',
        'secondary-dim': '#adc6ff20',
        tertiary: '#ffb95f',
        'tertiary-dim': '#ffb95f20',
        error: '#ef4444',
        'error-dim': '#ef444420',
        outline: '#424754',
        'outline-dim': '#42475426',
        'on-surface': '#c9cdd5',
        'on-surface-variant': '#8b919d',
      },
      borderRadius: {
        DEFAULT: '0.25rem',
        sm: '0.125rem',
      },
      fontFamily: {
        headline: ['Space Grotesk', 'sans-serif'],
        ui: ['Inter', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      fontSize: {
        'label-sm': ['0.6875rem', { lineHeight: '1rem', letterSpacing: '0.02em' }],
        'label-md': ['0.75rem', { lineHeight: '1rem', letterSpacing: '0.01em' }],
        'body-md': ['0.875rem', { lineHeight: '1.25rem' }],
        'headline-lg': ['1.5rem', { lineHeight: '2rem', fontWeight: '700' }],
        'headline-md': ['1.125rem', { lineHeight: '1.5rem', fontWeight: '600' }],
      },
      spacing: {
        'compact': '0.5rem',
        'tight': '0.75rem',
        'card': '0.75rem',
      },
    },
  },
  plugins: [],
}
