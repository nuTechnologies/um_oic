<template>
  <nu-button @click="showModal">Show modal</nu-button>
  <nu-modal v-if="modalActive" title="Modal title" :hide-actions="true">
    <template #content>
      <span>Your changes were not saved. Do you want to save them?</span>
      <div class="flex justify-end items-center mt-4">
        <div class="nu-modal-body-actions-grid">
          <nu-button
            class="nu-modal-body-actions-cancel ml-2"
            appearance="outline"
            @click="hideModal"
            >Cancel
          </nu-button>
          <nu-button
            class="nu-modal-body-actions-secondary ml-2"
            appearance="outline"
            @click="hideModal"
          >
            Don't Save
          </nu-button>
          <nu-button class="nu-modal-body-actions-confirm ml-2" appearance="filled" @click="confirm"
            >Save
          </nu-button>
        </div>
      </div>
    </template>
  </nu-modal>
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
