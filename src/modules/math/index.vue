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
    <div class="display">
      <div class="expression">{{ expression || '0' }}</div>
      <div class="result">{{ result }}</div>
    </div>

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
</template>

<style scoped>
.math-module {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.display {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 16px;
  text-align: right;
}

.expression {
  font-size: 18px;
  color: var(--text-secondary);
  min-height: 28px;
  word-break: break-all;
}

.result {
  font-size: 32px;
  font-weight: bold;
  color: var(--text-primary);
  min-height: 42px;
  word-break: break-all;
}

.keypad {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.keypad-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.key {
  height: 60px;
  border: none;
  border-radius: 12px;
  font-size: 20px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.key:hover {
  transform: scale(1.03);
  background: var(--bg-hover);
}

.key:active {
  transform: scale(0.97);
}

.key-function {
  background: var(--bg-hover);
  color: var(--primary-color);
}

.key-operator {
  background: var(--primary-color);
  color: white;
}

.key-operator:hover {
  background: var(--primary-hover);
}

.key-equals {
  grid-column: span 2;
  background: var(--success-color);
  color: white;
}

.key-equals:hover {
  background: #16a34a;
}
</style>
