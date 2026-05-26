<template>
  <Transition name="fade">
    <div
      v-if="visible"
      class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-[100]"
      @click.self="onCancel"
    >
      <div
        class="bg-dark-800 border border-white/10 rounded-2xl p-6 m-4 max-w-sm w-full shadow-2xl animate-slide-up"
      >
        <!-- Icon -->
        <div class="flex justify-center mb-4">
          <div
            class="w-14 h-14 rounded-full flex items-center justify-center"
            :class="iconBgClass"
          >
            <component
              :is="iconComponent"
              class="w-7 h-7"
              :class="iconColorClass"
            />
          </div>
        </div>

        <!-- Title -->
        <h3 class="text-lg font-semibold text-center mb-2">{{ title }}</h3>

        <!-- Message -->
        <p class="text-gray-400 text-sm text-center mb-6">{{ message }}</p>

        <!-- Buttons -->
        <div class="flex gap-3">
          <button
            v-if="showCancel"
            @click="onCancel"
            class="btn-secondary flex-1 py-2.5"
          >
            {{ cancelText }}
          </button>
          <button
            @click="onConfirm"
            class="flex-1 py-2.5 rounded-xl font-medium transition-all duration-200"
            :class="confirmButtonClass"
          >
            {{ confirmText }}
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { FunctionalComponent, computed } from "vue";
import {
  AlertTriangleIcon,
  AlertCircleIcon,
  CheckCircleIcon,
  HelpCircleIcon,
  Trash2Icon,
} from "lucide-vue-next";

type DialogType = "warning" | "error" | "success" | "info" | "danger";

interface Props {
  visible: boolean;
  title: string;
  message: string;
  type?: DialogType;
  confirmText?: string;
  cancelText?: string;
  showCancel?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  type: "info",
  confirmText: "确定",
  cancelText: "取消",
  showCancel: true,
});

const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();

const iconComponent = computed<FunctionalComponent>(() => {
  const iconMap: Record<DialogType, FunctionalComponent> = {
    warning: AlertTriangleIcon,
    error: AlertCircleIcon,
    success: CheckCircleIcon,
    info: HelpCircleIcon,
    danger: Trash2Icon,
  };
  return iconMap[props.type];
});

const iconBgClass = computed(() => {
  const bgMap: Record<DialogType, string> = {
    warning: "bg-yellow-500/20",
    error: "bg-red-500/20",
    success: "bg-green-500/20",
    info: "bg-blue-500/20",
    danger: "bg-red-500/20",
  };
  return bgMap[props.type];
});

const iconColorClass = computed(() => {
  const colorMap: Record<DialogType, string> = {
    warning: "text-yellow-400",
    error: "text-red-400",
    success: "text-green-400",
    info: "text-blue-400",
    danger: "text-red-400",
  };
  return colorMap[props.type];
});

const confirmButtonClass = computed(() => {
  const classMap: Record<DialogType, string> = {
    warning: "bg-yellow-500 hover:bg-yellow-600 text-white",
    error: "bg-red-500 hover:bg-red-600 text-white",
    success: "bg-green-500 hover:bg-green-600 text-white",
    info: "btn-primary",
    danger: "bg-red-500 hover:bg-red-600 text-white",
  };
  return classMap[props.type];
});

const onConfirm = () => {
  emit("confirm");
};

const onCancel = () => {
  emit("cancel");
};
</script>
