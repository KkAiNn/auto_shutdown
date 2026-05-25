<template>
  <div class="flex flex-col gap-3">
    <label class="text-xs font-medium text-gray-500 uppercase tracking-wider">
      自定义时间
    </label>
    <div class="flex items-center gap-3">
      <div class="flex-1 relative">
        <input
          v-model="hoursInput"
          type="text"
          inputmode="numeric"
          pattern="[0-9]*"
          :disabled="isRunning"
          placeholder="00"
          class="input-field text-center font-mono text-lg"
          @input="onHoursInput"
          @blur="validateHours"
        />
        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-gray-500">时</span>
      </div>
      <span class="text-xl text-gray-600 font-light">:</span>
      <div class="flex-1 relative">
        <input
          v-model="minutesInput"
          type="text"
          inputmode="numeric"
          pattern="[0-9]*"
          :disabled="isRunning"
          placeholder="00"
          class="input-field text-center font-mono text-lg"
          @input="onMinutesInput"
          @blur="validateMinutes"
        />
        <span class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-gray-500">分</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

interface Props {
  isRunning: boolean;
  modelValue?: number;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  change: [totalMinutes: number];
  'update:modelValue': [totalMinutes: number];
}>();

const hoursInput = ref('');
const minutesInput = ref('');
const hours = ref(0);
const minutes = ref(0);

const onHoursInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  // 只允许数字
  const value = target.value.replace(/[^0-9]/g, '');
  hoursInput.value = value;
  hours.value = parseInt(value) || 0;
  validateHours();
};

const onMinutesInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  // 只允许数字
  const value = target.value.replace(/[^0-9]/g, '');
  minutesInput.value = value;
  minutes.value = parseInt(value) || 0;
  validateMinutes();
};

const validateHours = () => {
  if (hours.value < 0) hours.value = 0;
  if (hours.value > 23) hours.value = 23;
  hoursInput.value = hours.value.toString();
  emitChange();
};

const validateMinutes = () => {
  if (minutes.value < 0) minutes.value = 0;
  if (minutes.value > 59) minutes.value = 59;
  minutesInput.value = minutes.value.toString();
  emitChange();
};

const emitChange = () => {
  emit('change', hours.value * 60 + minutes.value);
};

// Reset when not running
watch(() => props.isRunning, (newVal) => {
  if (!newVal) {
    hoursInput.value = '';
    minutesInput.value = '';
    hours.value = 0;
    minutes.value = 0;
  }
});

// Watch for external value changes
watch(() => props.modelValue, (newVal) => {
  if (newVal !== undefined && newVal >= 0) {
    hours.value = Math.floor(newVal / 60);
    minutes.value = newVal % 60;
    hoursInput.value = hours.value > 0 ? hours.value.toString() : '';
    minutesInput.value = minutes.value > 0 ? minutes.value.toString() : '';
  }
}, { immediate: true });
</script>
