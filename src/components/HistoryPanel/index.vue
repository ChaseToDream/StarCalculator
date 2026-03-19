<script setup lang="ts">
import { useCalculatorStore } from '../../stores/calculatorStore';

const store = useCalculatorStore();
</script>

<template>
  <div class="history-panel" v-if="store.history.length > 0">
    <div class="history-header">
      <h3>📜 计算历史</h3>
      <button class="clear-btn" @click="store.clearHistory">清除</button>
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
  margin-top: 16px;
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 16px;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.history-header h3 {
  font-size: 16px;
  color: var(--text-primary);
  margin: 0;
}

.clear-btn {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  background: var(--danger-color);
  color: white;
  font-size: 12px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.clear-btn:hover {
  opacity: 0.8;
}

.history-list {
  max-height: 200px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.history-item {
  background: var(--bg-primary);
  padding: 10px 12px;
  border-radius: 8px;
  border-left: 3px solid var(--primary-color);
}

.history-expression {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.history-result {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.history-list::-webkit-scrollbar {
  width: 6px;
}

.history-list::-webkit-scrollbar-track {
  background: var(--bg-secondary);
}

.history-list::-webkit-scrollbar-thumb {
  background: var(--bg-hover);
  border-radius: 3px;
}

.history-list::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}
</style>
