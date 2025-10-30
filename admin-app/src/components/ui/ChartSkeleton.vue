<template>
  <div class="animate-pulse">
    <div class="flex items-end justify-between space-x-2" :style="{ height: height + 'px' }">
      <div
        v-for="i in barCount"
        :key="i"
        class="bg-gray-300 dark:bg-gray-600 rounded-sm flex-1"
        :style="{ height: getRandomHeight() + 'px' }"
      ></div>
    </div>
    <div class="mt-2 flex justify-between">
      <div
        v-for="i in barCount"
        :key="'label-' + i"
        class="skeleton w-8 h-3"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  height?: number
  barCount?: number
}

const props = withDefaults(defineProps<Props>(), {
  height: 200,
  barCount: 7
})

const getRandomHeight = () => {
  // Generate random height between 40% and 90% of total height
  return Math.floor(props.height * (0.4 + Math.random() * 0.5))
}
</script>