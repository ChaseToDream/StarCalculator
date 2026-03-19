<script setup lang="ts">
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useCalculatorStore } from '../../stores/calculatorStore';

const store = useCalculatorStore();

const categories = [
  { id: 'length', label: '长度', icon: '📏' },
  { id: 'weight', label: '重量', icon: '⚖️' },
  { id: 'temperature', label: '温度', icon: '🌡️' },
  { id: 'number_base', label: '进制', icon: '🔢' },
];

const units = {
  length: [
    { value: 'meter', label: '米 (m)' },
    { value: 'kilometer', label: '千米 (km)' },
    { value: 'centimeter', label: '厘米 (cm)' },
    { value: 'millimeter', label: '毫米 (mm)' },
    { value: 'inch', label: '英寸 (in)' },
    { value: 'foot', label: '英尺 (ft)' },
    { value: 'yard', label: '码 (yd)' },
    { value: 'mile', label: '英里 (mi)' },
  ],
  weight: [
    { value: 'kilogram', label: '千克 (kg)' },
    { value: 'gram', label: '克 (g)' },
    { value: 'milligram', label: '毫克 (mg)' },
    { value: 'ton', label: '吨 (t)' },
    { value: 'pound', label: '磅 (lb)' },
    { value: 'ounce', label: '盎司 (oz)' },
  ],
  temperature: [
    { value: 'celsius', label: '摄氏度 (°C)' },
    { value: 'fahrenheit', label: '华氏度 (°F)' },
    { value: 'kelvin', label: '开尔文 (K)' },
  ],
  number_base: [
    { value: 'binary', label: '二进制 (2)' },
    { value: 'octal', label: '八进制 (8)' },
    { value: 'decimal', label: '十进制 (10)' },
    { value: 'hexadecimal', label: '十六进制 (16)' },
  ],
};

const currentCategory = ref('length');
const fromValue = ref('0');
const fromUnit = ref('meter');
const toValue = ref('0');
const toUnit = ref('kilometer');
const error = ref('');

const getUnits = () => units[currentCategory.value as keyof typeof units];

const convert = async () => {
  if (!fromValue.value || fromValue.value === '') {
    toValue.value = '';
    error.value = '';
    return;
  }

  try {
    error.value = '';
    let result: string | number;

    if (currentCategory.value === 'number_base') {
      result = await invoke('converter_number_base', {
        value: fromValue.value,
        from: fromUnit.value,
        to: toUnit.value,
      });
    } else {
      const numValue = parseFloat(fromValue.value);
      if (isNaN(numValue)) {
        error.value = '请输入有效的数字';
        return;
      }

      const command = `converter_${currentCategory.value}`;
      result = await invoke(command, {
        value: numValue,
        from: fromUnit.value,
        to: toUnit.value,
      });
    }

    toValue.value = typeof result === 'number' ? formatNumber(result) : result;

    store.addHistory({
      expression: `${fromValue.value} ${fromUnit.value} → ${toUnit.value}`,
      result: toValue.value,
      module: 'converter',
    });
  } catch (err) {
    error.value = err as string;
    toValue.value = '';
  }
};

const formatNumber = (num: number): string => {
  if (Math.abs(num) >= 1e10 || (Math.abs(num) < 1e-6 && num !== 0)) {
    return num.toExponential(6);
  }
  const rounded = Math.round(num * 1e10) / 1e10;
  return rounded.toString();
};

const switchCategory = (category: string) => {
  currentCategory.value = category;
  const unitList = getUnits();
  fromUnit.value = unitList[0].value;
  toUnit.value = unitList.length > 1 ? unitList[1].value : unitList[0].value;
  fromValue.value = '0';
  toValue.value = '0';
  error.value = '';
};

const swapUnits = () => {
  const temp = fromUnit.value;
  fromUnit.value = toUnit.value;
  toUnit.value = temp;
  const tempValue = fromValue.value;
  fromValue.value = toValue.value;
  toValue.value = tempValue;
  if (fromValue.value && fromValue.value !== '0') {
    convert();
  }
};

watch([fromValue, fromUnit, toUnit, currentCategory], () => {
  convert();
});
</script>

<template>
  <div class="converter-module">
    <div class="category-selector">
      <button
        v-for="cat in categories"
        :key="cat.id"
        :class="['category-btn', { active: currentCategory === cat.id }]"
        @click="switchCategory(cat.id)"
      >
        <span class="icon">{{ cat.icon }}</span>
        <span class="label">{{ cat.label }}</span>
      </button>
    </div>

    <div class="converter-card">
      <div class="input-section">
        <label class="input-label">从</label>
        <input
          v-model="fromValue"
          type="text"
          class="input-value"
          placeholder="输入数值"
        />
        <select v-model="fromUnit" class="unit-select">
          <option v-for="unit in getUnits()" :key="unit.value" :value="unit.value">
            {{ unit.label }}
          </option>
        </select>
      </div>

      <button class="swap-btn" @click="swapUnits" title="交换单位">
        ⇅
      </button>

      <div class="input-section">
        <label class="input-label">到</label>
        <input
          v-model="toValue"
          type="text"
          class="input-value result"
          readonly
          placeholder="结果"
        />
        <select v-model="toUnit" class="unit-select">
          <option v-for="unit in getUnits()" :key="unit.value" :value="unit.value">
            {{ unit.label }}
          </option>
        </select>
      </div>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
    </div>
  </div>
</template>

<style scoped>
.converter-module {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.category-selector {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.category-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 16px 8px;
  border: none;
  border-radius: 12px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
}

.category-btn:hover {
  background: var(--bg-hover);
}

.category-btn.active {
  background: var(--primary-color);
  color: white;
}

.category-btn .icon {
  font-size: 24px;
}

.converter-card {
  background: var(--bg-secondary);
  border-radius: 16px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  position: relative;
}

.input-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-label {
  font-size: 14px;
  color: var(--text-secondary);
  font-weight: 500;
}

.input-value {
  width: 100%;
  padding: 14px 16px;
  border: 2px solid var(--border-color);
  border-radius: 10px;
  font-size: 20px;
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: border-color 0.2s;
}

.input-value:focus {
  outline: none;
  border-color: var(--primary-color);
}

.input-value.result {
  background: var(--bg-hover);
  font-weight: 600;
}

.unit-select {
  width: 100%;
  padding: 12px 16px;
  border: 2px solid var(--border-color);
  border-radius: 10px;
  font-size: 16px;
  background: var(--bg-primary);
  color: var(--text-primary);
  cursor: pointer;
  transition: border-color 0.2s;
}

.unit-select:focus {
  outline: none;
  border-color: var(--primary-color);
}

.swap-btn {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 44px;
  height: 44px;
  border: none;
  border-radius: 50%;
  background: var(--primary-color);
  color: white;
  font-size: 20px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
}

.swap-btn:hover {
  transform: translate(-50%, -50%) scale(1.1);
  background: var(--primary-hover);
}

.swap-btn:active {
  transform: translate(-50%, -50%) scale(0.95);
}

.error-message {
  padding: 12px 16px;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid var(--danger-color);
  border-radius: 10px;
  color: var(--danger-color);
  font-size: 14px;
  text-align: center;
}
</style>
