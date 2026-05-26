<template>
  <div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50" @click.self="$emit('close')">
    <div class="bg-dark-800 border border-white/10 rounded-2xl m-4 max-w-lg w-full shadow-2xl animate-slide-up flex flex-col max-h-[80vh]">
      <!-- Header -->
      <div class="flex items-center justify-between p-4 border-b border-white/10">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-yellow-500/20 flex items-center justify-center">
            <ClockIcon class="w-4 h-4 text-yellow-400" />
          </div>
          <div>
            <h3 class="text-lg font-semibold">管理快速预设</h3>
            <p class="text-xs text-gray-500">自定义您的快捷时间选项</p>
          </div>
        </div>
        <button @click="$emit('close')" class="p-2 rounded-lg hover:bg-white/10 transition-colors">
          <XIcon class="w-5 h-5 text-gray-400" />
        </button>
      </div>

      <!-- Add New Preset -->
      <div class="p-4 border-b border-white/10">
        <div class="flex items-end gap-2">
          <div class="w-16">
            <label class="block text-xs text-gray-500 mb-1">小时</label>
            <input
              v-model.number="newHours"
              type="number"
              min="0"
              placeholder="0"
              class="no-spinner w-full px-3 py-2 rounded-xl bg-dark-700/50 border border-white/10 text-white text-sm focus:outline-none focus:border-primary-500/50 transition-colors"
            />
          </div>
          <div class="text-gray-500 pb-2 text-lg font-bold">:</div>
          <div class="w-16">
            <label class="block text-xs text-gray-500 mb-1">分钟</label>
            <input
              ref="minsInputRef"
              :value="newMins"
              @input="onMinsInput"
              type="number"
              placeholder="0"
              class="no-spinner w-full px-3 py-2 rounded-xl bg-dark-700/50 border border-white/10 text-white text-sm focus:outline-none focus:border-primary-500/50 transition-colors"
            />
          </div>
          <div class="flex-1">
            <label class="block text-xs text-gray-500 mb-1">显示标签</label>
            <input
              v-model="newLabel"
              type="text"
              :placeholder="autoLabel"
              class="w-full px-3 py-2 rounded-xl bg-dark-700/50 border border-white/10 text-white text-sm focus:outline-none focus:border-primary-500/50 transition-colors"
            />
          </div>
          <button
            @click="addNewPreset"
            :disabled="!canAdd"
            class="px-4 py-2 rounded-xl text-sm font-medium bg-primary-500 hover:bg-primary-600 text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            添加
          </button>
        </div>
      </div>

      <!-- Preset List -->
      <div class="flex-1 overflow-y-auto p-4 space-y-2">
        <div v-if="presets.length === 0" class="text-center py-8 text-gray-500">
          <ClockIcon class="w-12 h-12 mx-auto mb-3 opacity-30" />
          <p>暂无预设，添加一个吧</p>
        </div>
        <div
          v-for="preset in presets"
          :key="preset.id"
          class="flex items-center gap-3 p-3 rounded-xl bg-dark-700/50 border border-white/5 hover:bg-dark-700 transition-colors group"
        >
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <span class="text-sm font-medium text-white">{{ preset.label }}</span>
              <span class="text-xs text-gray-500">({{ preset.minutes }}分钟)</span>
            </div>
          </div>
          <button
            @click="deletePreset(preset.id)"
            class="p-2 rounded-lg text-gray-500 hover:text-red-400 hover:bg-red-500/10 transition-colors opacity-0 group-hover:opacity-100"
          >
            <TrashIcon class="w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- Footer -->
      <div class="flex items-center justify-between p-4 border-t border-white/10">
        <button
          @click="resetPresets"
          class="px-4 py-2 rounded-xl text-sm font-medium text-gray-400 hover:text-white hover:bg-white/10 transition-colors"
        >
          恢复默认
        </button>
        <button
          @click="$emit('close')"
          class="btn-primary px-6 py-2"
        >
          完成
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { XIcon, ClockIcon, TrashIcon } from 'lucide-vue-next';
import { usePresets } from '../composables/usePresets';

defineEmits<{
  close: [];
}>();

const { presets, addPreset, deletePreset, resetToDefaults } = usePresets();

const newHours = ref<number | null>(null);
const newMins = ref<number | null>(null);
const newLabel = ref('');
const minsInputRef = ref<HTMLInputElement | null>(null);

const onMinsInput = (e: Event) => {
  const input = e.target as HTMLInputElement;
  let v = parseInt(input.value) || 0;
  v = Math.max(0, Math.min(59, v));
  newMins.value = v;
  if (minsInputRef.value) {
    minsInputRef.value.value = String(v);
  }
};

const totalMinutes = computed(() => {
  const hours = newHours.value ?? 0;
  const mins = newMins.value ?? 0;
  return hours * 60 + mins;
});

const autoLabel = computed(() => {
  let totalMins = totalMinutes.value;
  if (totalMins <= 0) return '';

  const minsPerDay = 24 * 60;
  const minsPerHour = 60;

  let label = '';

  if (totalMins >= minsPerDay) {
    const days = Math.floor(totalMins / minsPerDay);
    totalMins = totalMins % minsPerDay;
    label += `${days}天`;
  }

  if (totalMins >= minsPerHour) {
    const hours = Math.floor(totalMins / minsPerHour);
    totalMins = totalMins % minsPerHour;
    label += `${hours}小时`;
  }

  if (totalMins > 0) {
    label += `${totalMins}分钟`;
  }

  return label;
});

const canAdd = computed(() => {
  return totalMinutes.value > 0;
});

const addNewPreset = () => {
  if (canAdd.value) {
    const label = newLabel.value.trim() || autoLabel.value;
    addPreset(totalMinutes.value, label);
    newHours.value = null;
    newMins.value = null;
    newLabel.value = '';
  }
};

const resetPresets = () => {
  resetToDefaults();
};
</script>

<style scoped>
.no-spinner::-webkit-outer-spin-button,
.no-spinner::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
.no-spinner {
  -moz-appearance: textfield;
}
</style>
