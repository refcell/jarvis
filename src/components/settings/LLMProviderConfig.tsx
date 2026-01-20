import { useState, useEffect } from 'react';
import { Check, AlertCircle, Loader2 } from 'lucide-react';
import { Card, Button, Input, Badge } from '../common';
import { useLLMProvider } from '../../hooks';
import type { LLMProviderType } from '../../types';
import styles from './LLMProviderConfig.module.css';

const providerOptions: { value: LLMProviderType; label: string; requiresKey: boolean }[] = [
  { value: 'claude_cli', label: 'Claude CLI', requiresKey: false },
  { value: 'cbcode_cli', label: 'cbcode CLI', requiresKey: false },
  { value: 'anthropic', label: 'Anthropic API', requiresKey: true },
  { value: 'openai', label: 'OpenAI API', requiresKey: true },
  { value: 'ollama', label: 'Ollama (Local)', requiresKey: false },
];

export function LLMProviderConfig() {
  const {
    config,
    detectedTools,
    isLoadingConfig,
    isCheckingHealth,
    isHealthy,
    healthCheck,
    updateConfig,
    storeApiKey,
    hasApiKey,
  } = useLLMProvider();

  const [apiKey, setApiKey] = useState('');
  const [selectedProvider, setSelectedProvider] = useState<LLMProviderType>('claude_cli');
  const [endpoint, setEndpoint] = useState('http://localhost:11434');

  useEffect(() => {
    if (config) {
      setSelectedProvider(config.provider_type);
      if (config.endpoint) setEndpoint(config.endpoint);
    }
  }, [config]);

  const currentProviderOption = providerOptions.find((p) => p.value === selectedProvider);
  const requiresApiKey = currentProviderOption?.requiresKey ?? false;

  const handleSaveProvider = () => {
    updateConfig({
      provider_type: selectedProvider,
      model: config?.model ?? null,
      endpoint: selectedProvider === 'ollama' ? endpoint : null,
      api_key_stored: hasApiKey,
      enabled: true,
    });
  };

  const handleSaveApiKey = () => {
    if (apiKey) {
      storeApiKey({ provider: selectedProvider, apiKey });
      setApiKey('');
    }
  };

  if (isLoadingConfig) {
    return (
      <Card>
        <div className={styles.loading}>
          <Loader2 className={styles.spinner} />
          Loading configuration...
        </div>
      </Card>
    );
  }

  return (
    <Card>
      <h3 className={styles.title}>LLM Provider</h3>
      <p className={styles.description}>
        Choose how Jarvis analyzes screen content to detect tasks.
      </p>

      <div className={styles.detectedTools}>
        <span className={styles.label}>Detected CLI Tools:</span>
        <div className={styles.tools}>
          <Badge variant={detectedTools?.claude ? 'success' : 'default'} size="sm">
            claude {detectedTools?.claude ? '✓' : '✗'}
          </Badge>
          <Badge variant={detectedTools?.cbcode ? 'success' : 'default'} size="sm">
            cbcode {detectedTools?.cbcode ? '✓' : '✗'}
          </Badge>
          <Badge variant={detectedTools?.ollama ? 'success' : 'default'} size="sm">
            ollama {detectedTools?.ollama ? '✓' : '✗'}
          </Badge>
        </div>
      </div>

      <div className={styles.providerSelect}>
        <label className={styles.label}>Provider</label>
        <select
          value={selectedProvider}
          onChange={(e) => setSelectedProvider(e.target.value as LLMProviderType)}
          className={styles.select}
        >
          {providerOptions.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </div>

      {selectedProvider === 'ollama' && (
        <Input
          label="Ollama Endpoint"
          value={endpoint}
          onChange={(e) => setEndpoint(e.target.value)}
          placeholder="http://localhost:11434"
        />
      )}

      {requiresApiKey && (
        <div className={styles.apiKeySection}>
          <Input
            label="API Key"
            type="password"
            value={apiKey}
            onChange={(e) => setApiKey(e.target.value)}
            placeholder={hasApiKey ? '••••••••' : 'Enter your API key'}
          />
          {hasApiKey && (
            <Badge variant="success" size="sm">
              <Check size={12} /> Key stored securely
            </Badge>
          )}
          <Button size="sm" onClick={handleSaveApiKey} disabled={!apiKey}>
            Save API Key
          </Button>
        </div>
      )}

      <div className={styles.actions}>
        <Button onClick={handleSaveProvider}>Save Provider</Button>
        <Button
          variant="secondary"
          onClick={() => healthCheck()}
          loading={isCheckingHealth}
          icon={
            isHealthy === true ? (
              <Check size={16} />
            ) : isHealthy === false ? (
              <AlertCircle size={16} />
            ) : undefined
          }
        >
          {isHealthy === true ? 'Connected' : isHealthy === false ? 'Failed' : 'Test Connection'}
        </Button>
      </div>
    </Card>
  );
}
