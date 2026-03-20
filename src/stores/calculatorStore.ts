import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { ModuleType, HistoryItem } from '../types';

export const useCalculatorStore = defineStore('calculator', () => {
  const currentModule = ref<ModuleType>('math');
  const theme = ref<'light' | 'dark'>('dark');
  const history = ref<HistoryItem[]>([]);

  const setModule = (module: ModuleType) => {
    currentModule.value = module;
  };

  const initTheme = () => {
    document.documentElement.classList.toggle('light', theme.value === 'light');
    document.documentElement.classList.toggle('dark', theme.value === 'dark');
  };

  const toggleTheme = () => {
    theme.value = theme.value === 'dark' ? 'light' : 'dark';
    document.documentElement.classList.toggle('light', theme.value === 'light');
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
    initTheme,
    toggleTheme,
    addHistory,
    clearHistory,
  };
});
