<template>
  <div class="flex flex-col gap-3">
    <label class="text-xs font-medium text-gray-500 uppercase tracking-wider">
      快速预设
    </label>
    <div class="grid grid-cols-4 gap-2">
      <button
        v-for="preset in displayPresets"
        :key="preset.id"
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
import { computed } from 'vue';
import { usePresets } from '../composables/usePresets';

interface Props {
  selectedMinutes: number;
  isRunning: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  select: [minutes: number];
}>();

const { presets } = usePresets();

const displayPresets = computed(() => {
  return presets.value.slice(0, 8).map((p) => ({
    id: p.id,
    minutes: p.minutes,
    label: p.label,
  }));
});

const selectPreset = (minutes: number) => {
  if (!props.isRunning) {
    emit('select', minutes);
  }
};
</script>
