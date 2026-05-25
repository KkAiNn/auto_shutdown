<template>
  <div class="flex flex-col gap-3">
    <label class="text-xs font-medium text-gray-500 uppercase tracking-wider">
      快速预设
    </label>
    <div class="grid grid-cols-4 gap-2">
      <button
        v-for="preset in presets"
        :key="preset.minutes"
        @click="selectPreset(preset.minutes)"
        :disabled="isRunning"
        class="relative px-3 py-2.5 rounded-xl text-sm font-medium transition-all duration-200 border"
        :class="selectedMinutes === preset.minutes && !isRunning
          ? 'bg-primary-500/20 border-primary-500/50 text-primary-400 shadow-glow'
          : 'bg-dark-800/50 border-white/5 text-gray-400 hover:bg-dark-700 hover:text-white hover:border-white/10'
        "
        :style="isRunning ? 'opacity: 0.5; cursor: not-allowed;' : ''"
      >
        {{ preset.label }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Preset {
  minutes: number;
  label: string;
}

interface Props {
  selectedMinutes: number;
  isRunning: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  select: [minutes: number];
}>();

const presets: Preset[] = [
  { minutes: 15, label: '15分' },
  { minutes: 30, label: '30分' },
  { minutes: 45, label: '45分' },
  { minutes: 60, label: '1小时' },
  { minutes: 90, label: '1.5小时' },
  { minutes: 120, label: '2小时' },
  { minutes: 180, label: '3小时' },
  { minutes: 240, label: '4小时' },
];

const selectPreset = (minutes: number) => {
  if (!props.isRunning) {
    emit('select', minutes);
  }
};
</script>
