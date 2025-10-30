<template>
  <div id="app" class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <!-- Loading Screen -->
    <LoadingScreen v-if="isInitializing" />

    <!-- Login View -->
    <LoginView v-else-if="!authStore.isAuthenticated" />

    <!-- Main Application -->
    <AppLayout v-else>
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
import LoginView from '@/views/auth/LoginView.vue'
import LoadingScreen from '@/components/ui/LoadingScreen.vue'
import NotificationContainer from '@/components/ui/NotificationContainer.vue'
import GlobalModals from '@/components/ui/GlobalModals.vue'

const authStore = useAuthStore()
const systemStore = useSystemStore()
const isInitializing = ref(true)

onMounted(async () => {
  try {
    // Initialize authentication
    await authStore.initializeAuth()

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