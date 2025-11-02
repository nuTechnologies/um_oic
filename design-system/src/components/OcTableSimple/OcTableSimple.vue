<template>
  <table :class="tableClasses">
    <slot />
  </table>
</template>

<script setup lang="ts">
import { computed } from 'vue'

export interface Props {
  /**
   * @docs Determines if the table rows should have a hover effect.
   * @default false
   */
  hover?: boolean
}

export interface Slots {
  /**
   * @docs Table content.
   */
  default?: () => unknown
}

const { hover = false } = defineProps<Props>()
defineSlots<Slots>()

const tableClasses = computed(() => {
  const result = ['nu-table-simple']
  if (hover) {
    result.push('nu-table-simple-hover')
  }
  return result
})
</script>
<style scoped>
@reference '@xwork-eu/design-system/tailwind';

@layer components {
  .nu-table-simple {
    @apply w-full;
  }
  .nu-table-simple-hover tr {
    @apply transition-colors duration-200 ease-in-out;
  }
  .nu-table-simple-hover tr:hover {
    @apply bg-role-secondary-container;
  }
  .nu-table-simple tr + tr {
    @apply border-t;
  }
}
</style>
