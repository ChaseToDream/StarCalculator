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
  gap: 10px;
  padding: 14px;
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-md);
  border: 1px solid var(--border-color);
}

.module-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 14px 10px;
  border: none;
  border-radius: var(--radius-lg);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  font-size: 13px;
  font-weight: 500;
}

.module-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  transform: translateY(-2px);
}

.module-btn.active {
  background: linear-gradient(135deg, var(--primary-color), var(--primary-dark));
  color: white;
  box-shadow: var(--shadow-md);
}

.module-btn .icon {
  font-size: 26px;
  line-height: 1;
  font-weight: 600;
}

.module-btn .label {
  font-size: 12px;
  font-weight: 600;
}
</style>
