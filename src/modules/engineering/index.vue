<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useCalculatorStore } from '../../stores/calculatorStore';

const store = useCalculatorStore();

const categories = [
  { id: 'area', label: '面积计算', icon: '📐' },
  { id: 'volume', label: '体积计算', icon: '📦' },
  { id: 'mechanics', label: '力学计算', icon: '⚡' },
  { id: 'electric', label: '电路计算', icon: '🔌' },
];

const currentCategory = ref('area');
const result = ref('');
const error = ref('');
const inputs = ref<Record<string, string>>({});

const calculators = {
  area: [
    {
      id: 'rectangle',
      label: '矩形面积',
      formula: '面积 = 宽 × 高',
      fields: [
        { name: 'width', label: '宽', placeholder: '输入宽度' },
        { name: 'height', label: '高', placeholder: '输入高度' },
      ],
      command: 'engineering_rectangle_area',
    },
    {
      id: 'square',
      label: '正方形面积',
      formula: '面积 = 边长²',
      fields: [{ name: 'side', label: '边长', placeholder: '输入边长' }],
      command: 'engineering_square_area',
    },
    {
      id: 'circle',
      label: '圆形面积',
      formula: '面积 = π × 半径²',
      fields: [{ name: 'radius', label: '半径', placeholder: '输入半径' }],
      command: 'engineering_circle_area',
    },
    {
      id: 'triangle',
      label: '三角形面积',
      formula: '面积 = ½ × 底 × 高',
      fields: [
        { name: 'base', label: '底', placeholder: '输入底边长度' },
        { name: 'height', label: '高', placeholder: '输入高' },
      ],
      command: 'engineering_triangle_area',
    },
    {
      id: 'trapezoid',
      label: '梯形面积',
      formula: '面积 = ½ × (上底 + 下底) × 高',
      fields: [
        { name: 'base1', label: '上底', placeholder: '输入上底长度' },
        { name: 'base2', label: '下底', placeholder: '输入下底长度' },
        { name: 'height', label: '高', placeholder: '输入高' },
      ],
      command: 'engineering_trapezoid_area',
    },
  ],
  volume: [
    {
      id: 'cube',
      label: '立方体体积',
      formula: '体积 = 边长³',
      fields: [{ name: 'side', label: '边长', placeholder: '输入边长' }],
      command: 'engineering_cube_volume',
    },
    {
      id: 'rectangular_prism',
      label: '长方体体积',
      formula: '体积 = 长 × 宽 × 高',
      fields: [
        { name: 'length', label: '长', placeholder: '输入长度' },
        { name: 'width', label: '宽', placeholder: '输入宽度' },
        { name: 'height', label: '高', placeholder: '输入高度' },
      ],
      command: 'engineering_rectangular_prism_volume',
    },
    {
      id: 'sphere',
      label: '球体体积',
      formula: '体积 = (4/3) × π × 半径³',
      fields: [{ name: 'radius', label: '半径', placeholder: '输入半径' }],
      command: 'engineering_sphere_volume',
    },
    {
      id: 'cylinder',
      label: '圆柱体体积',
      formula: '体积 = π × 半径² × 高',
      fields: [
        { name: 'radius', label: '半径', placeholder: '输入半径' },
        { name: 'height', label: '高', placeholder: '输入高' },
      ],
      command: 'engineering_cylinder_volume',
    },
    {
      id: 'cone',
      label: '圆锥体体积',
      formula: '体积 = (1/3) × π × 半径² × 高',
      fields: [
        { name: 'radius', label: '半径', placeholder: '输入半径' },
        { name: 'height', label: '高', placeholder: '输入高' },
      ],
      command: 'engineering_cone_volume',
    },
  ],
  mechanics: [
    {
      id: 'force',
      label: '力的计算',
      formula: '力 = 质量 × 加速度 (F = ma)',
      fields: [
        { name: 'mass', label: '质量 (kg)', placeholder: '输入质量' },
        { name: 'acceleration', label: '加速度 (m/s²)', placeholder: '输入加速度' },
      ],
      command: 'engineering_force',
      resultUnit: 'N',
    },
    {
      id: 'pressure',
      label: '压力计算',
      formula: '压力 = 力 / 面积 (P = F/A)',
      fields: [
        { name: 'force_val', label: '力 (N)', placeholder: '输入力' },
        { name: 'area', label: '面积 (m²)', placeholder: '输入面积' },
      ],
      command: 'engineering_pressure',
      resultUnit: 'Pa',
    },
    {
      id: 'work',
      label: '功的计算',
      formula: '功 = 力 × 距离 (W = Fs)',
      fields: [
        { name: 'force_val', label: '力 (N)', placeholder: '输入力' },
        { name: 'distance', label: '距离 (m)', placeholder: '输入距离' },
      ],
      command: 'engineering_work',
      resultUnit: 'J',
    },
    {
      id: 'power',
      label: '功率计算',
      formula: '功率 = 功 / 时间 (P = W/t)',
      fields: [
        { name: 'work_val', label: '功 (J)', placeholder: '输入功' },
        { name: 'time', label: '时间 (s)', placeholder: '输入时间' },
      ],
      command: 'engineering_power',
      resultUnit: 'W',
    },
  ],
  electric: [
    {
      id: 'ohms_voltage',
      label: '电压计算 (欧姆定律)',
      formula: '电压 = 电流 × 电阻 (V = IR)',
      fields: [
        { name: 'current', label: '电流 (A)', placeholder: '输入电流' },
        { name: 'resistance', label: '电阻 (Ω)', placeholder: '输入电阻' },
      ],
      command: 'engineering_ohms_law_voltage',
      resultUnit: 'V',
    },
    {
      id: 'ohms_current',
      label: '电流计算 (欧姆定律)',
      formula: '电流 = 电压 / 电阻 (I = V/R)',
      fields: [
        { name: 'voltage', label: '电压 (V)', placeholder: '输入电压' },
        { name: 'resistance', label: '电阻 (Ω)', placeholder: '输入电阻' },
      ],
      command: 'engineering_ohms_law_current',
      resultUnit: 'A',
    },
    {
      id: 'electrical_power',
      label: '电功率计算',
      formula: '功率 = 电压 × 电流 (P = VI)',
      fields: [
        { name: 'voltage', label: '电压 (V)', placeholder: '输入电压' },
        { name: 'current', label: '电流 (A)', placeholder: '输入电流' },
      ],
      command: 'engineering_electrical_power',
      resultUnit: 'W',
    },
  ],
};

const currentCalculator = ref(calculators.area[0]);

const switchCategory = (category: string) => {
  currentCategory.value = category;
  currentCalculator.value = calculators[category as keyof typeof calculators][0];
  inputs.value = {};
  result.value = '';
  error.value = '';
};

const switchCalculator = (calc: any) => {
  currentCalculator.value = calc;
  inputs.value = {};
  result.value = '';
  error.value = '';
};

const calculate = async () => {
  try {
    error.value = '';
    const args: Record<string, number> = {};

    for (const field of currentCalculator.value.fields) {
      const value = parseFloat(inputs.value[field.name] || '');
      if (isNaN(value)) {
        error.value = `请输入有效的${field.label}`;
        return;
      }
      args[field.name] = value;
    }

    const res: number = await invoke(currentCalculator.value.command, args);
    const unit = (currentCalculator.value as any).resultUnit || '';
    result.value = formatResult(res) + (unit ? ` ${unit}` : '');

    store.addHistory({
      expression: currentCalculator.value.label,
      result: result.value,
      module: 'engineering',
    });
  } catch (err) {
    error.value = err as string;
    result.value = '';
  }
};

const formatResult = (num: number): string => {
  if (Math.abs(num) >= 1e10 || (Math.abs(num) < 1e-6 && num !== 0)) {
    return num.toExponential(6);
  }
  const rounded = Math.round(num * 1e10) / 1e10;
  return rounded.toString();
};
</script>

<template>
  <div class="engineering-module">
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

    <div class="calculator-list">
      <button
        v-for="calc in calculators[currentCategory as keyof typeof calculators]"
        :key="calc.id"
        :class="['calculator-item', { active: currentCalculator.id === calc.id }]"
        @click="switchCalculator(calc)"
      >
        {{ calc.label }}
      </button>
    </div>

    <div class="calculator-form">
      <div class="formula">
        <span class="formula-icon">📝</span>
        {{ currentCalculator.formula }}
      </div>

      <div class="input-group">
        <div
          v-for="field in currentCalculator.fields"
          :key="field.name"
          class="input-field"
        >
          <label>{{ field.label }}</label>
          <input
            v-model="inputs[field.name]"
            type="text"
            :placeholder="field.placeholder"
            @input="calculate()"
          />
        </div>
      </div>

      <div v-if="result" class="result-display">
        <div class="result-label">计算结果</div>
        <div class="result-value">{{ result }}</div>
      </div>

      <div v-if="error" class="error-message">
        {{ error }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.engineering-module {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.category-selector {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
}

.category-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 16px 12px;
  border: none;
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  font-size: 13px;
  box-shadow: var(--shadow-sm);
  border: 1px solid transparent;
}

.category-btn:hover {
  background: var(--bg-tertiary);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.category-btn.active {
  background: linear-gradient(135deg, var(--primary-color), var(--primary-dark));
  color: white;
  border-color: var(--primary-light);
  box-shadow: var(--shadow-md);
}

.category-btn .icon {
  font-size: 26px;
  line-height: 1;
}

.calculator-list {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.calculator-item {
  padding: 10px 16px;
  border: 2px solid var(--border-color);
  border-radius: 24px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 14px;
  font-weight: 500;
}

.calculator-item:hover {
  border-color: var(--primary-color);
  background: var(--bg-tertiary);
  transform: translateY(-1px);
}

.calculator-item.active {
  background: linear-gradient(135deg, var(--primary-color), var(--primary-dark));
  border-color: var(--primary-color);
  color: white;
  box-shadow: var(--shadow-md);
}

.calculator-form {
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  padding: 24px;
  box-shadow: var(--shadow-md);
  border: 1px solid var(--border-color);
}

.formula {
  text-align: center;
  padding: 14px 18px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: 20px;
  font-family: 'Consolas', 'Monaco', monospace;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.formula-icon {
  font-size: 18px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 14px;
  margin-bottom: 20px;
}

.input-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.input-field label {
  font-size: 14px;
  color: var(--text-secondary);
  font-weight: 600;
}

.input-field input {
  width: 100%;
  padding: 14px 18px;
  border: 2px solid var(--border-color);
  border-radius: var(--radius-md);
  font-size: 16px;
  font-weight: 500;
  background: var(--bg-tertiary);
  color: var(--text-primary);
  transition: all 0.3s ease;
}

.input-field input:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.2);
}

.result-display {
  text-align: center;
  padding: 24px;
  background: linear-gradient(135deg, var(--primary-color), var(--primary-dark));
  border-radius: var(--radius-lg);
  color: white;
  box-shadow: var(--shadow-lg);
}

.result-label {
  font-size: 14px;
  opacity: 0.9;
  margin-bottom: 10px;
  text-transform: uppercase;
  letter-spacing: 1px;
  font-weight: 600;
}

.result-value {
  font-size: 32px;
  font-weight: 700;
  word-break: break-all;
  letter-spacing: -0.5px;
}

.error-message {
  margin-top: 16px;
  padding: 14px 18px;
  background: linear-gradient(135deg, rgba(239, 68, 68, 0.15), rgba(239, 68, 68, 0.1));
  border: 1px solid var(--danger-color);
  border-radius: var(--radius-md);
  color: var(--danger-color);
  font-size: 14px;
  font-weight: 500;
  text-align: center;
  box-shadow: var(--shadow-sm);
}
</style>
