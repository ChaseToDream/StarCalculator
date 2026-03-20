<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useCalculatorStore } from '../../stores/calculatorStore';

const store = useCalculatorStore();

const expression = ref('');
const result = ref('');
const currentValue = ref('0');
const previousValue = ref('');
const operator = ref('');
const waitingForOperand = ref(false);
const justCalculated = ref(false);

const updateDisplay = () => {
  expression.value = previousValue.value + (operator.value ? ` ${operator.value} ` : '') + currentValue.value;
};

const inputNumber = (num: string) => {
  if (waitingForOperand.value || justCalculated.value) {
    currentValue.value = num;
    waitingForOperand.value = false;
    justCalculated.value = false;
  } else {
    currentValue.value = currentValue.value === '0' ? num : currentValue.value + num;
  }
  updateDisplay();
};

const inputDecimal = () => {
  if (waitingForOperand.value || justCalculated.value) {
    currentValue.value = '0.';
    waitingForOperand.value = false;
    justCalculated.value = false;
  } else if (!currentValue.value.includes('.')) {
    currentValue.value += '.';
  }
  updateDisplay();
};

const clear = () => {
  currentValue.value = '0';
  previousValue.value = '';
  operator.value = '';
  expression.value = '';
  result.value = '';
  waitingForOperand.value = false;
  justCalculated.value = false;
};

const backspace = () => {
  if (waitingForOperand.value || justCalculated.value) return;
  if (currentValue.value.length > 1) {
    currentValue.value = currentValue.value.slice(0, -1);
  } else {
    currentValue.value = '0';
  }
  updateDisplay();
};

const performOperation = async (op: string) => {
  if (operator.value && !waitingForOperand.value && !justCalculated.value) {
    await calculate();
  }
  previousValue.value = currentValue.value;
  operator.value = op;
  waitingForOperand.value = true;
  updateDisplay();
};

const calculate = async () => {
  if (!operator.value || !previousValue.value) return;

  try {
    const a = parseFloat(previousValue.value);
    const b = parseFloat(currentValue.value);
    let res: number;

    switch (operator.value) {
      case '+':
        res = await invoke('math_add', { a, b });
        break;
      case '-':
        res = await invoke('math_subtract', { a, b });
        break;
      case '×':
      case '*':
        res = await invoke('math_multiply', { a, b });
        break;
      case '÷':
      case '/':
        res = await invoke('math_divide', { a, b });
        break;
      default:
        return;
    }

    currentValue.value = formatResult(res);
    result.value = currentValue.value;
    previousValue.value = '';
    operator.value = '';
    waitingForOperand.value = false;
    justCalculated.value = true;

    store.addHistory({
      expression: expression.value,
      result: result.value,
      module: 'math',
    });

    expression.value = result.value;
  } catch (error) {
    result.value = error as string;
  }
};

const formatResult = (num: number): string => {
  if (Number.isInteger(num)) {
    return num.toString();
  }
  const formatted = num.toPrecision(12);
  return parseFloat(formatted).toString();
};

const buttons = [
  ['C', '←', '%', '÷'],
  ['7', '8', '9', '×'],
  ['4', '5', '6', '-'],
  ['1', '2', '3', '+'],
  ['0', '.', '=', '='],
];

const handleButton = async (btn: string) => {
  switch (btn) {
    case 'C':
      clear();
      break;
    case '←':
      backspace();
      break;
    case '.':
      inputDecimal();
      break;
    case '=':
      await calculate();
      break;
    case '+':
    case '-':
    case '×':
    case '÷':
    case '%':
      await performOperation(btn);
      break;
    default:
      if (/^[0-9]$/.test(btn)) {
        inputNumber(btn);
      }
  }
};
</script>

<template>
  <div class="math-module">
    <div class="display-container">
      <div class="display">
        <div class="expression">{{ expression || '0' }}</div>
        <div class="result">{{ result || '0' }}</div>
      </div>
    </div>

    <div class="keypad-container">
      <div class="keypad">
        <div v-for="(row, rowIndex) in buttons" :key="rowIndex" class="keypad-row">
          <button
            v-for="btn in row"
            :key="btn"
            :class="['key', {
              'key-function': ['C', '←', '%'].includes(btn),
              'key-operator': ['+', '-', '×', '÷'].includes(btn),
              'key-equals': btn === '='
            }]"
            @click="handleButton(btn)"
          >
            {{ btn }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.math-module {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.display-container {
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  padding: 24px;
  box-shadow: var(--shadow-md);
  border: 1px solid var(--border-color);
}

.display {
  text-align: right;
}

.expression {
  font-size: 16px;
  color: var(--text-secondary);
  min-height: 26px;
  word-break: break-all;
  margin-bottom: 8px;
  font-weight: 500;
}

.result {
  font-size: 36px;
  font-weight: 700;
  color: var(--primary-light);
  min-height: 48px;
  word-break: break-all;
  letter-spacing: -0.5px;
}

.keypad-container {
  background: var(--bg-secondary);
  border-radius: var(--radius-xl);
  padding: 20px;
  box-shadow: var(--shadow-md);
  border: 1px solid var(--border-color);
}

.keypad {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.keypad-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.key {
  height: 64px;
  border: none;
  border-radius: var(--radius-lg);
  font-size: 22px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  background: var(--bg-tertiary);
  color: var(--text-primary);
  box-shadow: var(--shadow-sm);
  position: relative;
  overflow: hidden;
}

.key::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(180deg, rgba(255,255,255,0.1) 0%, rgba(255,255,255,0) 100%);
  pointer-events: none;
}

.key:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.key:active {
  transform: translateY(0);
  box-shadow: var(--shadow-sm);
}

.key-function {
  background: var(--bg-hover);
  color: var(--primary-light);
  font-size: 18px;
}

.key-operator {
  background: linear-gradient(135deg, var(--primary-color), var(--primary-dark));
  color: white;
  font-size: 24px;
}

.key-operator:hover {
  background: linear-gradient(135deg, var(--primary-hover), var(--primary-color));
}

.key-equals {
  grid-column: span 2;
  background: linear-gradient(135deg, var(--success-color), #16a34a);
  color: white;
  font-size: 28px;
}

.key-equals:hover {
  background: linear-gradient(135deg, #16a34a, #15803d);
}
</style>
