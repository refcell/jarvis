export interface Task {
  id: string;
  title: string;
  description: string;
  context: string;
  initial_priority: number;
  current_priority: number;
  status: TaskStatus;
  source_window: string | null;
  created_at: string;
  updated_at: string;
  snoozed_until: string | null;
}

export type TaskStatus = 'pending' | 'in_progress' | 'completed' | 'dismissed' | 'snoozed';

export interface DetectedTask {
  title: string;
  description: string;
  priority: number;
}

export interface CaptureContext {
  id: string;
  ocr_text: string;
  active_window_title: string | null;
  active_app_name: string | null;
  captured_at: string;
  screen_width: number;
  screen_height: number;
}

export type LLMProviderType =
  | 'anthropic'
  | 'openai'
  | 'ollama'
  | 'claude_cli'
  | 'cbcode_cli'
  | 'custom';

export interface LLMConfig {
  provider_type: LLMProviderType;
  model: string | null;
  endpoint: string | null;
  api_key_stored: boolean;
  enabled: boolean;
}

export interface DetectedCLITools {
  claude: boolean;
  cbcode: boolean;
  ollama: boolean;
}

export interface Settings {
  capture_interval_secs: number;
  watching_enabled: boolean;
  notifications_enabled: boolean;
  llm_config: LLMConfig;
  priority_decay_rate: number;
}

export interface WatchStatus {
  is_watching: boolean;
  last_capture_at: string | null;
  captures_since_start: number;
  tasks_detected_since_start: number;
}

export interface NavigationItem {
  id: string;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
  path: string;
}
