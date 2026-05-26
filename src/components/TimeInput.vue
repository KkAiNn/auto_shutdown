<template>
  <div class="flex items-center gap-2">
    <div class="flex-1">
      <label v-if="showLabels" class="block text-xs text-gray-500 mb-1">{{ hoursLabel }}</label>
      <input
        :value="hours"
        @input="onHoursInput"
        type="number"
        :min="minHours"
        :max="maxHours"
        :placeholder="hoursPlaceholder"
        class="no-spinner w-full px-3 py-2 rounded-xl bg-dark-700/50 border border-white/10 text-white text-sm focus:outline-none focus:border-primary-500/50 transition-colors"
        :class="inputClass"
      />
    </div>
    <span class="text-gray-500 pt-5" :class="{ 'pt-0': !showLabels }">:</span>
    <div class="flex-1">
      <label v-if="showLabels" class="block text-xs text-gray-500 mb-1">{{ minutesLabel }}</label>
      <input
        :value="minutes"
        @input="onMinutesInput"
        type="number"
        :min="0"
        :max="59"
        :placeholder="minutesPlaceholder"
        class="no-spinner w-full px-3 py-2 rounded-xl bg-dark-700/50 border border-white/10 text-white text-sm focus:outline-none focus:border-primary-500/50 transition-colors"
        :class="inputClass"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  modelValue: number; // Total minutes
  showLabels?: boolean;
  hoursLabel?: string;
  minutesLabel?: string;
  hoursPlaceholder?: string;
  minutesPlaceholder?: string;
  minHours?: number;
  maxHours?: number;
  inputClass?: string;
}

const props = withDefaults(defineProps<Props>(), {
  showLabels: true,
  hoursLabel: '小时',
  minutesLabel: '分钟',
  hoursPlaceholder: '0',
  minutesPlaceholder: '0',
  minHours: 0,
  maxHours: 23,
  inputClass: '',
});

const emit = defineEmits<{
  'update:modelValue': [value: number];
}>();

// Calculate hours and minutes from total minutes
const hours = computed(() => {
  if (props.modelValue <= 0) return '';
  return Math.floor(props.modelValue / 60);
});

const minutes = computed(() => {
  if (props.modelValue <= 0) return '';
  return props.modelValue % 60;
});

// Handle hours input
const onHoursInput = (e: Event) => {
  const input = e.target as HTMLInputElement;
  let value = parseInt(input.value) || 0;
  value = Math.max(props.minHours, Math.min(props.maxHours, value));
  
  // Update input display
  input.value = String(value);
  
  // Calculate total minutes
  const currentMinutes = minutes.value || 0;
  const totalMinutes = value * 60 + currentMinutes;
  emit('update:modelValue', totalMinutes);
};

// Handle minutes input
const onMinutesInput = (e: Event) => {
  const input = e.target as HTMLInputElement;
  let value = parseInt(input.value) || 0;
  value = Math.max(0, Math.min(59, value));
  
  // Update input display
  input.value = String(value);
  
  // Calculate total minutes
  const currentHours = hours.value || 0;
  const totalMinutes = currentHours * 60 + value;
  emit('update:modelValue', totalMinutes);
};
</script>

<style scoped>
/* Hide number input spinners */
.no-spinner::-webkit-outer-spin-button,
.no-spinner::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.no-spinner {
  -moz-appearance: textfield;
}
</style>
