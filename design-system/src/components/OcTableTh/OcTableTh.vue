<template>
  <nu-table-cell
    type="th"
    :align-h="alignH"
    :align-v="alignV"
    :width="width"
    :wrap="wrap"
    class="nu-th aria-[sort]:cursor-pointer"
    @click="$emit('click', $event)"
  >
    <slot />
  </nu-table-cell>
</template>

<script setup lang="ts">
import OcTableCell from '../OcTableCell/OcTableCell.vue'

export interface Props {
  alignH?: 'left' | 'center' | 'right'
  alignV?: 'top' | 'middle' | 'bottom'
  width?: 'auto' | 'shrink' | 'expand'
  wrap?: 'break' | 'nowrap' | 'truncate'
}

const { alignH = 'left', alignV = 'middle', width = 'auto', wrap } = defineProps<Props>()
defineEmits(['click'])
</script>
<style scoped>
@reference '@xwork-eu/design-system/tailwind';

@layer components {
  .nu-th {
    @apply font-medium;
  }
}
</style>
