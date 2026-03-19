<script setup lang="ts">
import { useCalculatorStore } from './stores/calculatorStore';
import ModuleSwitcher from './components/ModuleSwitcher/index.vue';
import HistoryPanel from './components/HistoryPanel/index.vue';
import MathModule from './modules/math/index.vue';
import ConverterModule from './modules/converter/index.vue';
import EngineeringModule from './modules/engineering/index.vue';

const store = useCalculatorStore();

const getCurrentModule = () => {
  switch (store.currentModule) {
    case 'math':
      return MathModule;
    case 'converter':
      return ConverterModule;
    case 'engineering':
      return EngineeringModule;
    default:
      return MathModule;
  }
};
</script>

<template>
  <div class="app-container">
    <header class="app-header">
      <h1 class="app-title">
        <span class="icon">✨</span>
        星辰计算器
      </h1>
      <button class="theme-btn" @click="store.toggleTheme" :title="store.theme === 'light' ? '切换深色模式' : '切换浅色模式'">
        {{ store.theme === 'light' ? '🌙' : '☀️' }}
      </button>
    </header>

    <main class="app-main">
      <ModuleSwitcher />
      <component :is="getCurrentModule()" />
      <HistoryPanel />
    </main>
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.app-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

.app-title .icon {
  font-size: 24px;
}

.theme-btn {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 10px;
  background: var(--bg-primary);
  font-size: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.theme-btn:hover {
  background: var(--bg-hover);
  transform: scale(1.05);
}

.app-main {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}
</style>
