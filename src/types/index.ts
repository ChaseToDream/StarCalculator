export type ModuleType = 'math' | 'converter' | 'engineering';

export interface CalculatorState {
  currentModule: ModuleType;
  theme: 'light' | 'dark';
  history: HistoryItem[];
}

export interface HistoryItem {
  id: string;
  timestamp: number;
  expression: string;
  result: string;
  module: ModuleType;
}

export interface CalculatorResult {
  success: boolean;
  value?: string;
  error?: string;
}
