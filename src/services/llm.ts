import { invoke } from '@tauri-apps/api/core';
import type { CaptureContext, DetectedCLITools, DetectedTask, LLMConfig, Task } from '../types';

export async function detectAvailableCLITools(): Promise<DetectedCLITools> {
  return invoke('detect_available_cli_tools');
}

export async function analyzeContext(context: CaptureContext): Promise<DetectedTask[]> {
  return invoke('analyze_context', { context });
}

export async function analyzeAndCreateTasks(context: CaptureContext): Promise<Task[]> {
  return invoke('analyze_and_create_tasks', { context });
}

export async function healthCheckLLM(): Promise<boolean> {
  return invoke('health_check_llm');
}

export async function getLLMConfig(): Promise<LLMConfig> {
  return invoke('get_llm_config');
}

export async function updateLLMConfig(config: LLMConfig): Promise<void> {
  return invoke('update_llm_config', { config });
}
