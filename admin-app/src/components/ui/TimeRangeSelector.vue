<template>
  <select
    :value="modelValue"
    @change="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
    class="form-select text-sm"
  >
    <option value="1d">Letzter Tag</option>
    <option value="7d">Letzte Woche</option>
    <option value="30d">Letzter Monat</option>
    <option value="90d">Letzte 3 Monate</option>
  </select>
</template>

<script setup lang="ts">
interface Props {
  modelValue: string
}

defineProps<Props>()

defineEmits<{
  'update:modelValue': [value: string]
  change: [value: string]
}>()
</script>