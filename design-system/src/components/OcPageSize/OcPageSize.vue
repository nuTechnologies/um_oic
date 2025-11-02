<template>
  <div class="nu-page-size flex items-center gap-1">
    <label
      class="nu-page-size-label"
      :for="selectId"
      data-testid="nu-page-size-label"
      :aria-hidden="true"
      v-text="label"
    />
    <nu-select
      :input-id="selectId"
      class="nu-page-size-select min-w-25 [&_.vs\_\_dropdown-menu]:!min-w-25"
      data-testid="nu-page-size-select"
      :model-value="selected"
      :label="label"
      :label-hidden="true"
      :options="options"
      :clearable="false"
      :searchable="false"
      @update:model-value="emitChange"
    />
  </div>
</template>

<script setup lang="ts">
import { uniqueId } from '../../helpers'
import OcSelect from '../OcSelect/OcSelect.vue'

export interface Props {
  /**
   * @docs The label of the select.
   */
  label: string
  /**
   * @docs The available options of the select.
   */
  options: unknown[]
  /**
   * @docs The selected value.
   */
  selected: string | number
  /**
   * @docs The element ID of the select.
   */
  selectId?: string
}

export interface Emits {
  /**
   * @docs Emitted when the value of the select has changed.
   */
  (event: 'change', value: string | boolean): void
}

const { label, options, selected, selectId = uniqueId('nu-page-size-') } = defineProps<Props>()

const emit = defineEmits<Emits>()

const emitChange = (value: string | boolean) => {
  emit('change', value)
}
</script>
