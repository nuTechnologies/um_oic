<template>
  <nu-button @click="showModal">Show modal</nu-button>
  <nu-modal
    v-if="modalActive"
    title="Modal title"
    message="Some message."
    @confirm="confirm"
    @cancel="hideModal"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'

const modalActive = ref(false)

const showModal = () => {
  modalActive.value = true
}

const hideModal = () => {
  modalActive.value = false
}

const confirm = () => {
  // handle confirm...

  modalActive.value = false
}
</script>
