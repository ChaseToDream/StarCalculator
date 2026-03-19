import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { ModuleType, HistoryItem } from '../types';

export const useCalculatorStore = defineStore('calculator', () => {
  const currentModule = ref<ModuleType>('math');
  const theme = ref<'light' | 'dark'>('light');
  const history = ref<HistoryItem[]>([]);

  const setModule = (module: ModuleType) => {
    currentModule.value = module;
  };

  const toggleTheme = () => {
    theme.value = theme.value === 'light' ? 'dark' : 'light';
    document.documentElement.classList.toggle('dark', theme.value === 'dark');
  };

  const addHistory = (item: Omit<HistoryItem, 'id' | 'timestamp'>) => {
    history.value.unshift({
      ...item,
      id: Date.now().toString(),
      timestamp: Date.now(),
    });
    if (history.value.length > 100) {
      history.value.pop();
    }
  };

  const clearHistory = () => {
    history.value = [];
  };

  return {
    currentModule,
    theme,
    history,
    setModule,
    toggleTheme,
    addHistory,
    clearHistory,
  };
});
