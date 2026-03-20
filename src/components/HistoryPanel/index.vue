<script setup lang="ts">
import { useCalculatorStore } from '../../stores/calculatorStore';

const store = useCalculatorStore();
</script>

<template>
  <div class="history-panel" v-if="store.history.length > 0">
    <div class="history-header">
      <h3>
        <span class="header-icon">📜</span>
        计算历史
      </h3>
      <button class="clear-btn" @click="store.clearHistory">
        <span class="clear-icon">🗑️</span>
        清除
      </button>
    </div>
    <div class="history-list">
      <div v-for="item in store.history" :key="item.id" class="history-item">
        <div class="history-expression">{{ item.expression }}</div>
        <div class="history-result">= {{ item.result }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.history-panel {
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  padding: 20px;
  box-shadow: var(--shadow-md);
  border: 1px solid var(--border-color);
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.history-header h3 {
  font-size: 15px;
  color: var(--text-primary);
  margin: 0;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-icon {
  font-size: 18px;
}

.clear-btn {
  padding: 8px 14px;
  border: none;
  border-radius: var(--radius-md);
  background: linear-gradient(135deg, var(--danger-color), #dc2626);
  color: white;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 6px;
}

.clear-btn:hover {
  opacity: 0.9;
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

.clear-btn:active {
  transform: translateY(0);
}

.clear-icon {
  font-size: 14px;
}

.history-list {
  max-height: 180px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.history-item {
  background: var(--bg-tertiary);
  padding: 12px 16px;
  border-radius: var(--radius-md);
  border-left: 3px solid var(--primary-color);
  transition: all 0.2s ease;
}

.history-item:hover {
  background: var(--bg-hover);
  transform: translateX(4px);
}

.history-expression {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 4px;
  font-weight: 500;
}

.history-result {
  font-size: 16px;
  font-weight: 700;
  color: var(--primary-light);
}

.history-list::-webkit-scrollbar {
  width: 6px;
}

.history-list::-webkit-scrollbar-track {
  background: var(--bg-secondary);
  border-radius: 3px;
}

.history-list::-webkit-scrollbar-thumb {
  background: var(--bg-hover);
  border-radius: 3px;
}

.history-list::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}
</style>
