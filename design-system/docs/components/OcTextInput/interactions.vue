<template>
  <nu-button class="my-4" @click="focus">Focus input below</nu-button>
  <nu-text-input ref="inputForFocusRef" label="Focus field" />
  <nu-button class="my-4" @click="focusAndSelect">Focus and select input below</nu-button>
  <nu-text-input
    ref="inputForFocusSelectRef"
    label="Select field"
    model-value="Text to be selected"
  />
  <nu-text-input v-model="inputValueForClearing" label="Clear input" :clear-button-enabled="true" />
  <nu-text-input
    v-model="inputValueWithDefault"
    label="Input with default"
    :clear-button-enabled="true"
    default-value="Some default"
  />

  <p>Value: {{ inputValueWithDefault !== null ? inputValueWithDefault : 'null' }}</p>
</template>

<script setup lang="ts">
import { ref, unref, useTemplateRef } from 'vue'

const inputValueForClearing = ref<string>('clear me')
const inputValueWithDefault = ref<string | null>(null)

const inputForFocusRef = useTemplateRef('inputForFocusRef')
const focus = () => {
  unref(inputForFocusRef)?.focus()
}
const inputForFocusSelectRef = useTemplateRef('inputForFocusSelectRef')
const focusAndSelect = () => {
  unref(inputForFocusSelectRef)?.focus()
}
</script>
