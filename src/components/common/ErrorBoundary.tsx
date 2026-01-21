import { Component, type ErrorInfo, type ReactNode } from 'react';
import { Card } from './Card';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Error caught by boundary:', error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }
      return (
        <Card>
          <div style={{ color: 'var(--accent-danger)', padding: '16px' }}>
            <h3 style={{ margin: '0 0 8px 0' }}>Something went wrong</h3>
            <pre style={{
              whiteSpace: 'pre-wrap',
              fontSize: '12px',
              backgroundColor: 'var(--bg-tertiary)',
              padding: '8px',
              borderRadius: '4px'
            }}>
              {this.state.error?.message}
            </pre>
          </div>
        </Card>
      );
    }

    return this.props.children;
  }
}
