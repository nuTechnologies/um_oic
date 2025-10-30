<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <div class="mx-auto h-12 w-auto flex justify-center">
          <img class="h-12 w-auto" src="/logo.svg" alt="UM-OIC" />
        </div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900 dark:text-white">
          Bei Ihrem Konto anmelden
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600 dark:text-gray-400">
          UM-OIC Admin Panel
        </p>
      </div>

      <form class="mt-8 space-y-6" @submit.prevent="handleLogin">
        <div class="space-y-4">
          <div>
            <label for="email" class="sr-only">E-Mail-Adresse</label>
            <input
              id="email"
              name="email"
              type="email"
              autocomplete="email"
              required
              v-model="form.email"
              class="form-input"
              placeholder="E-Mail-Adresse"
              :disabled="isLoading"
            />
          </div>

          <div>
            <label for="password" class="sr-only">Passwort</label>
            <div class="relative">
              <input
                id="password"
                name="password"
                :type="showPassword ? 'text' : 'password'"
                autocomplete="current-password"
                required
                v-model="form.password"
                class="form-input pr-10"
                placeholder="Passwort"
                :disabled="isLoading"
                @keyup.enter="handleLogin"
              />
              <button
                type="button"
                class="absolute inset-y-0 right-0 pr-3 flex items-center"
                @click="showPassword = !showPassword"
              >
                <EyeIcon v-if="!showPassword" class="h-5 w-5 text-gray-400" />
                <EyeSlashIcon v-else class="h-5 w-5 text-gray-400" />
              </button>
            </div>
          </div>
        </div>

        <!-- Error message -->
        <div v-if="error" class="alert alert-error">
          <ExclamationTriangleIcon class="h-5 w-5" />
          <span>{{ error }}</span>
        </div>

        <div>
          <button
            type="submit"
            :disabled="isLoading || !form.email || !form.password"
            class="btn btn-primary w-full flex justify-center py-2 px-4 text-sm font-medium"
          >
            <div v-if="isLoading" class="spinner mr-2"></div>
            {{ isLoading ? 'Anmeldung läuft...' : 'Anmelden' }}
          </button>
        </div>

        <div class="flex items-center justify-between text-sm">
          <div class="text-gray-600 dark:text-gray-400">
            Demo-Zugangsdaten: admin@um-oic.test / admin123
          </div>
        </div>
      </form>

      <!-- System status -->
      <div class="mt-6 text-center">
        <div class="inline-flex items-center text-xs text-gray-500 dark:text-gray-400">
          <div
            class="w-2 h-2 rounded-full mr-2"
            :class="{
              'bg-green-500': systemStatus === 'healthy',
              'bg-yellow-500': systemStatus === 'degraded',
              'bg-red-500': systemStatus === 'unhealthy',
              'bg-gray-400': systemStatus === 'unknown'
            }"
          ></div>
          System Status: {{ systemStatusText }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { EyeIcon, EyeSlashIcon, ExclamationTriangleIcon } from '@heroicons/vue/24/outline'

import { useAuthStore } from '@/stores/auth'
import { useSystemStore } from '@/stores/system'
import type { LoginCredentials } from '@/stores/auth'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const systemStore = useSystemStore()

const form = ref<LoginCredentials>({
  email: '',
  password: ''
})

const showPassword = ref(false)
const error = ref('')
const isLoading = computed(() => authStore.isLoading)

const systemStatus = computed(() => systemStore.systemStatus?.status || 'unknown')
const systemStatusText = computed(() => {
  switch (systemStatus.value) {
    case 'healthy':
      return 'Online'
    case 'degraded':
      return 'Eingeschränkt'
    case 'unhealthy':
      return 'Störung'
    default:
      return 'Unbekannt'
  }
})

const handleLogin = async () => {
  if (!form.value.email || !form.value.password) {
    return
  }

  error.value = ''

  try {
    await authStore.login(form.value)

    // Redirect to intended page or dashboard
    const redirectTo = (route.query.redirect as string) || '/dashboard'
    router.push(redirectTo)
  } catch (err: any) {
    error.value = err.message || 'Anmeldung fehlgeschlagen'

    // Clear password on error
    form.value.password = ''
  }
}

onMounted(async () => {
  // Load system status for health indicator
  await systemStore.loadSystemStatus()
})
</script>