<template>
  <div id="app">
    <!-- Loading Screen -->
    <LoadingScreen v-if="isInitializing" />

    <!-- Main Application -->
    <AppLayout v-if="authStore.isAuthenticated">
      <router-view />
    </AppLayout>

    <!-- Global Notifications -->
    <NotificationContainer />

    <!-- Global Modals -->
    <GlobalModals />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useSystemStore } from '@/stores/system'
import AppLayout from '@/components/layout/AppLayout.vue'
import LoadingScreen from '@/components/ui/LoadingScreen.vue'
import NotificationContainer from '@/components/ui/NotificationContainer.vue'
import GlobalModals from '@/components/ui/GlobalModals.vue'

const authStore = useAuthStore()
const systemStore = useSystemStore()
const isInitializing = ref(true)

onMounted(async () => {
  try {
    // Check for token in URL parameters (from auth service redirect)
    const urlParams = new URLSearchParams(window.location.search)
    const token = urlParams.get('token')

    if (token) {
      // Save token and remove from URL
      authStore.setToken(token)
      // Clean URL without reloading
      const url = new URL(window.location.href)
      url.searchParams.delete('token')
      window.history.replaceState({}, document.title, url.pathname + url.hash)
    }

    // Initialize authentication
    await authStore.checkAuth()

    // Load system status if authenticated
    if (authStore.isAuthenticated) {
      await systemStore.loadSystemStatus()
    }
  } catch (error) {
    console.error('Initialization error:', error)
  } finally {
    isInitializing.value = false
  }
})
</script>

<style scoped>
/* Global app styles */
#app {
  font-family: 'Inter', system-ui, -apple-system, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>