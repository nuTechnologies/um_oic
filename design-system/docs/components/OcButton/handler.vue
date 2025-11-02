<template>
  <nu-button @click="increaseCounter">Increase counter</nu-button>
  <nu-button class="ml-4" @click="resetCounter">Reset counter</nu-button>
  <div class="mt-4">Current count: {{ counter }}</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const counter = ref(0)

const increaseCounter = () => {
  counter.value++
}

const resetCounter = () => {
  counter.value = 0
}
</script>
