import { ref } from 'vue';

type DialogType = 'warning' | 'error' | 'success' | 'info' | 'danger';

interface ConfirmOptions {
  title: string;
  message: string;
  type?: DialogType;
  confirmText?: string;
  cancelText?: string;
  showCancel?: boolean;
}

interface ConfirmState extends ConfirmOptions {
  visible: boolean;
  resolve: ((value: boolean) => void) | null;
}

const state = ref<ConfirmState>({
  visible: false,
  title: '',
  message: '',
  type: 'info',
  confirmText: '确定',
  cancelText: '取消',
  showCancel: true,
  resolve: null,
});

export function useConfirm() {
  const confirm = (options: ConfirmOptions): Promise<boolean> => {
    return new Promise((resolve) => {
      state.value = {
        ...options,
        visible: true,
        resolve,
      };
    });
  };

  const alert = (options: Omit<ConfirmOptions, 'showCancel'>): Promise<boolean> => {
    return confirm({
      ...options,
      showCancel: false,
    });
  };

  const onConfirm = () => {
    if (state.value.resolve) {
      state.value.resolve(true);
    }
    state.value.visible = false;
  };

  const onCancel = () => {
    if (state.value.resolve) {
      state.value.resolve(false);
    }
    state.value.visible = false;
  };

  return {
    state,
    confirm,
    alert,
    onConfirm,
    onCancel,
  };
}
