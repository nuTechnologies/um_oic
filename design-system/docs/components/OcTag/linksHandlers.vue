<template>
  <nu-tag type="button" class="mr-2" @click="onClick">I am a button</nu-tag>
  <nu-tag type="router-link" :to="{ name: 'somePage' }">I am a link</nu-tag>
  <p>Button clicked: {{ buttonClicked }}</p>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const buttonClicked = ref(false)

const onClick = () => {
  buttonClicked.value = true
}
</script>
