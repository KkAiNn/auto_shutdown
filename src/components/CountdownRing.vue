<template>
  <div class="relative flex items-center justify-center">
    <!-- Outer glow ring -->
    <div
      v-if="isRunning"
      class="absolute inset-0 rounded-full animate-pulse-ring"
      :style="{
        background: `radial-gradient(circle, rgba(0, 212, 255, 0.08) 0%, transparent 70%)`,
      }"
    />

    <!-- SVG Ring - Background Layer -->
    <svg
      class="transform -rotate-90"
      :width="size"
      :height="size"
      viewBox="0 0 200 200"
    >
      <!-- Background track circle -->
      <circle
        cx="100"
        cy="100"
        :r="radius"
        fill="none"
        stroke="rgba(255, 255, 255, 0.08)"
        :stroke-width="strokeWidth"
      />

      <!-- Progress circle -->
      <circle
        cx="100"
        cy="100"
        :r="radius"
        fill="none"
        :stroke="isRunning ? 'url(#gradient)' : 'rgba(0, 212, 255, 0.4)'"
        :stroke-width="strokeWidth"
        :stroke-dasharray="circumference"
        :stroke-dashoffset="strokeDashoffset"
        stroke-linecap="round"
        class="transition-all duration-1000 ease-linear"
      />

      <!-- Gradient definition -->
      <defs>
        <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#00d4ff" />
          <stop offset="100%" stop-color="#0891b2" />
        </linearGradient>
      </defs>
    </svg>

    <!-- Time display - Centered with background for better readability -->
    <div class="absolute inset-0 flex flex-col items-center justify-center">
      <!-- Background circle to prevent overlap -->
      <div 
        class="absolute rounded-full bg-dark-900/90"
        :style="{
          width: `${size * 0.65}px`,
          height: `${size * 0.65}px`,
        }"
      />
      
      <!-- Time text -->
      <div class="relative z-10 flex flex-col items-center">
        <div
          class="text-3xl font-mono font-bold tracking-wider"
          :class="isRunning ? 'gradient-text' : 'text-gray-400'"
        >
          {{ formattedTime }}
        </div>
        <div 
          class="text-[10px] mt-1 font-medium tracking-wide uppercase px-2 py-0.5 rounded-full"
          :class="isRunning ? 'text-primary-400 bg-primary-500/10' : 'text-gray-500 bg-gray-500/10'"
        >
          {{ isRunning ? '关机倒计时' : '准备就绪' }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  formattedTime: string;
  progress: number;
  isRunning: boolean;
  size?: number;
  strokeWidth?: number;
}

const props = withDefaults(defineProps<Props>(), {
  size: 200,
  strokeWidth: 6,
});

const radius = computed(() => (props.size / 2) - (props.strokeWidth * 2));
const circumference = computed(() => 2 * Math.PI * radius.value);
const strokeDashoffset = computed(() => {
  const progressValue = props.isRunning ? props.progress : 100;
  return circumference.value - (progressValue / 100) * circumference.value;
});
</script>
