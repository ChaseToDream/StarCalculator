<script setup lang="ts">
import { onMounted } from 'vue';
import { useCalculatorStore } from './stores/calculatorStore';
import ModuleSwitcher from './components/ModuleSwitcher/index.vue';
import HistoryPanel from './components/HistoryPanel/index.vue';
import MathModule from './modules/math/index.vue';
import ConverterModule from './modules/converter/index.vue';
import EngineeringModule from './modules/engineering/index.vue';

const store = useCalculatorStore();

onMounted(() => {
  store.initTheme();
});

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
      <div class="header-left">
        <span class="logo-icon">✨</span>
        <h1 class="app-title">星辰计算器</h1>
      </div>
      <div class="header-right">
        <button class="theme-btn" @click="store.toggleTheme" :title="store.theme === 'dark' ? '切换浅色模式' : '切换深色模式'">
          <span class="theme-icon">{{ store.theme === 'dark' ? '☀️' : '🌙' }}</span>
        </button>
      </div>
    </header>

    <main class="app-main">
      <div class="content-wrapper">
        <ModuleSwitcher />
        <component :is="getCurrentModule()" />
        <HistoryPanel />
      </div>
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
  padding: 16px 24px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  box-shadow: var(--shadow-sm);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo-icon {
  font-size: 28px;
  animation: float 3s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-3px); }
}

.app-title {
  font-size: 22px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.3px;
  background: linear-gradient(135deg, var(--primary-light), var(--primary-color));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.theme-btn {
  width: 44px;
  height: 44px;
  border: none;
  border-radius: var(--radius-lg);
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: var(--shadow-sm);
}

.theme-btn:hover {
  background: var(--bg-hover);
  transform: scale(1.05);
  box-shadow: var(--shadow-md);
}

.theme-btn:active {
  transform: scale(0.95);
}

.theme-icon {
  font-size: 20px;
  transition: transform 0.3s ease;
}

.theme-btn:hover .theme-icon {
  transform: rotate(15deg);
}

.app-main {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.content-wrapper {
  max-width: 500px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

@media (max-width: 600px) {
  .app-header {
    padding: 12px 16px;
  }
  
  .app-title {
    font-size: 18px;
  }
  
  .logo-icon {
    font-size: 24px;
  }
  
  .app-main {
    padding: 16px;
  }
}
</style>
