<template>
  <div>
    <div class="mb-6">Hidden announcer (please inspect element)</div>
    <nu-button @click="announce">Announce</nu-button>
    <nu-hidden-announcer level="polite" :announcement="announcement" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const announcement = ref<string>()

const announce = () => {
  announcement.value = 'Announcement complete'
}
</script>
