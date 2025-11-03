<template>
  <div id="app">
    <!-- Loading Screen -->
    <LoadingScreen v-if="isInitializing" />

    <!-- Main Application -->
    <AppLayout v-if="authStore.isAuthenticated">
      <router-view />
    </AppLayout>

    <!-- Redirect to auth service if not authenticated -->
    <div v-else-if="!isInitializing" class="flex items-center justify-center min-h-screen bg-gray-50 dark:bg-gray-900">
      <div class="text-center">
        <h2 class="text-2xl font-semibold text-gray-900 dark:text-white mb-2">Weiterleitung zur Anmeldung...</h2>
        <p class="text-gray-600 dark:text-gray-400">Sie werden automatisch zum Login weitergeleitet.</p>
      </div>
    </div>

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
    } else {
      // Not authenticated, redirect to auth service
      redirectToAuthService()
    }
  } catch (error) {
    console.error('Initialization error:', error)
    // On error, also redirect to auth service
    redirectToAuthService()
  } finally {
    isInitializing.value = false
  }
})

const redirectToAuthService = () => {
  const currentUrl = `${window.location.origin}${window.location.pathname}${window.location.search}${window.location.hash}`
  const authUrl = `https://localhost:8443/?redirect=${encodeURIComponent(currentUrl)}`

  // Add a small delay to show the redirect message
  setTimeout(() => {
    window.location.href = authUrl
  }, 1500)
}
</script>

<style scoped>
/* Global app styles */
#app {
  font-family: 'Inter', system-ui, -apple-system, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>