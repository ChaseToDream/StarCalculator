<script setup lang="ts">
import { useCalculatorStore } from '../../stores/calculatorStore';
import type { ModuleType } from '../../types';

const store = useCalculatorStore();

const modules: { type: ModuleType; label: string; icon: string }[] = [
  { type: 'math', label: '数学计算', icon: '∑' },
  { type: 'converter', label: '数据换算', icon: '⇄' },
  { type: 'engineering', label: '工程计算', icon: '⚙' },
];
</script>

<template>
  <div class="module-switcher">
    <button
      v-for="module in modules"
      :key="module.type"
      :class="['module-btn', { active: store.currentModule === module.type }]"
      @click="store.setModule(module.type)"
    >
      <span class="icon">{{ module.icon }}</span>
      <span class="label">{{ module.label }}</span>
    </button>
  </div>
</template>

<style scoped>
.module-switcher {
  display: flex;
  gap: 8px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 12px;
  margin-bottom: 16px;
}

.module-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 12px 8px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
}

.module-btn:hover {
  background: var(--bg-hover);
}

.module-btn.active {
  background: var(--primary-color);
  color: white;
}

.module-btn .icon {
  font-size: 24px;
  line-height: 1;
}

.module-btn .label {
  font-size: 12px;
}
</style>
