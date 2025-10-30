<template>
  <div>
    <!-- Welcome Section -->
    <div class="mb-8">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
        Willkommen zurück, {{ authStore.user?.first_name }}
      </h1>
      <p class="mt-1 text-sm text-gray-600 dark:text-gray-400">
        Hier ist eine Übersicht über Ihr UM-OIC System
      </p>
    </div>

    <!-- Quick Actions -->
    <div class="mb-6 flex flex-wrap gap-3">
      <router-link
        to="/users/create"
        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
      >
        <PlusIcon class="w-4 h-4 mr-2" />
        Benutzer erstellen
      </router-link>

      <router-link
        to="/clients/create"
        class="inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
      >
        <KeyIcon class="w-4 h-4 mr-2" />
        OAuth2 Client
      </router-link>

      <button
        @click="systemStore.reloadAuthService()"
        :disabled="systemStore.isReloading"
        class="inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-200 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors disabled:opacity-50"
      >
        <ArrowPathIcon
          class="w-4 h-4 mr-2"
          :class="{ 'animate-spin': systemStore.isReloading }"
        />
        Auth neu laden
      </button>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4 mb-8">
      <StatCard
        title="Gesamt Benutzer"
        :value="stats.totalUsers"
        :change="stats.userGrowth"
        change-type="increase"
        icon="UsersIcon"
        color="blue"
        :loading="statsLoading"
      />

      <StatCard
        title="Aktive Sessions"
        :value="stats.activeSessions"
        :change="stats.sessionChange"
        change-type="increase"
        icon="ComputerDesktopIcon"
        color="green"
        :loading="statsLoading"
      />

      <StatCard
        title="Organisationen"
        :value="stats.organizations"
        :change="stats.orgGrowth"
        change-type="increase"
        icon="BuildingOfficeIcon"
        color="purple"
        :loading="statsLoading"
      />

      <StatCard
        title="OAuth2 Clients"
        :value="stats.oauthClients"
        :change="stats.clientChange"
        change-type="increase"
        icon="KeyIcon"
        color="orange"
        :loading="statsLoading"
      />
    </div>

    <!-- System Health -->
    <div class="mb-8">
      <SystemHealthBanner
        :status="systemStore.systemStatus"
        @reload="systemStore.reloadAuthService()"
      />
    </div>

    <!-- Charts Section -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
      <!-- User Activity Chart -->
      <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">
            Benutzer-Aktivität (7 Tage)
          </h3>
          <ChartRefreshButton @refresh="refreshActivityData" />
        </div>

        <Suspense>
          <UserActivityChart
            :data="activityData"
            :loading="activityLoading"
            height="300"
          />
          <template #fallback>
            <ChartSkeleton height="300" />
          </template>
        </Suspense>
      </div>

      <!-- Login Distribution -->
      <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">
            Login-Verteilung nach Organisation
          </h3>
          <TimeRangeSelector
            v-model="loginTimeRange"
            @change="refreshLoginData"
          />
        </div>

        <Suspense>
          <OrgLoginChart
            :data="loginDistribution"
            :loading="loginLoading"
            height="300"
          />
          <template #fallback>
            <ChartSkeleton height="300" />
          </template>
        </Suspense>
      </div>
    </div>

    <!-- Recent Activity & Alerts -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Recent Activity -->
      <div class="lg:col-span-2 bg-white dark:bg-gray-800 shadow rounded-lg">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <div class="flex items-center justify-between">
            <h3 class="text-lg font-medium text-gray-900 dark:text-white">
              Letzte Aktivitäten
            </h3>
            <router-link
              to="/audit"
              class="text-sm text-blue-600 hover:text-blue-500 dark:text-blue-400"
            >
              Alle anzeigen →
            </router-link>
          </div>
        </div>

        <RecentActivityList
          :activities="recentActivities"
          :loading="activitiesLoading"
          :max-items="10"
        />
      </div>

      <!-- System Alerts & Info -->
      <div class="space-y-6">
        <!-- Alerts -->
        <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">
            System-Hinweise
          </h3>

          <SystemAlerts :alerts="systemAlerts" />
        </div>

        <!-- Quick Stats -->
        <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6">
          <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">
            Schnellübersicht
          </h3>

          <QuickStatsList :stats="quickStats" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import {
  PlusIcon,
  KeyIcon,
  ArrowPathIcon
} from '@heroicons/vue/24/outline'

// Stores
import { useAuthStore } from '@/stores/auth'
import { useUsersStore } from '@/stores/users'
import { useSystemStore } from '@/stores/system'
import { useStatsStore } from '@/stores/stats'

// Components
import StatCard from '@/components/ui/StatCard.vue'
import SystemHealthBanner from '@/components/system/SystemHealthBanner.vue'
import RecentActivityList from '@/components/activity/RecentActivityList.vue'
import SystemAlerts from '@/components/system/SystemAlerts.vue'
import QuickStatsList from '@/components/ui/QuickStatsList.vue'
import ChartRefreshButton from '@/components/ui/ChartRefreshButton.vue'
import TimeRangeSelector from '@/components/ui/TimeRangeSelector.vue'
import ChartSkeleton from '@/components/ui/ChartSkeleton.vue'

// Lazy loaded chart components
import { defineAsyncComponent } from 'vue'
const UserActivityChart = defineAsyncComponent(
  () => import('@/components/charts/UserActivityChart.vue')
)
const OrgLoginChart = defineAsyncComponent(
  () => import('@/components/charts/OrgLoginChart.vue')
)

// Store instances
const authStore = useAuthStore()
const usersStore = useUsersStore()
const systemStore = useSystemStore()
const statsStore = useStatsStore()

// Reactive data
const statsLoading = ref(true)
const activityLoading = ref(true)
const loginLoading = ref(true)
const activitiesLoading = ref(true)

const loginTimeRange = ref('7d')

// Computed properties
const stats = computed(() => ({
  totalUsers: statsStore.userStats?.total_users || 0,
  userGrowth: statsStore.userStats?.growth_percentage || 0,
  activeSessions: statsStore.sessionStats?.active_sessions || 0,
  sessionChange: statsStore.sessionStats?.change_percentage || 0,
  organizations: statsStore.orgStats?.total_organizations || 0,
  orgGrowth: statsStore.orgStats?.growth_percentage || 0,
  oauthClients: statsStore.clientStats?.total_clients || 0,
  clientChange: statsStore.clientStats?.change_percentage || 0
}))

const activityData = computed(() => statsStore.activityData)
const loginDistribution = computed(() => statsStore.loginDistribution)
const recentActivities = computed(() => statsStore.recentActivities)

const systemAlerts = computed(() => [
  ...(systemStore.systemStatus?.auth_data_stale ? [{
    type: 'warning' as const,
    title: 'Auth-Service Daten veraltet',
    message: 'Die Auth-Service Daten sind nicht mehr aktuell.',
    action: 'Neu laden',
    actionFn: () => systemStore.reloadAuthService()
  }] : []),
  ...(systemStore.systemStatus?.status === 'unhealthy' ? [{
    type: 'error' as const,
    title: 'System-Fehler',
    message: 'Ein oder mehrere Services sind nicht verfügbar.',
    action: 'Status prüfen',
    actionFn: () => router.push('/system/status')
  }] : [])
])

const quickStats = computed(() => [
  {
    label: 'Letzte Anmeldung',
    value: statsStore.lastLoginTime || 'Nie',
    icon: 'ClockIcon'
  },
  {
    label: 'Fehlgeschlagene Logins',
    value: statsStore.failedLoginsToday || 0,
    icon: 'ExclamationTriangleIcon',
    color: statsStore.failedLoginsToday > 10 ? 'red' : 'gray'
  },
  {
    label: 'Neue Benutzer (Woche)',
    value: statsStore.newUsersThisWeek || 0,
    icon: 'UserPlusIcon',
    color: 'green'
  }
])

// Methods
const loadDashboardData = async () => {
  try {
    statsLoading.value = true
    activityLoading.value = true
    loginLoading.value = true
    activitiesLoading.value = true

    // Load all dashboard data in parallel
    await Promise.all([
      statsStore.loadUserStats(),
      statsStore.loadSessionStats(),
      statsStore.loadOrgStats(),
      statsStore.loadClientStats(),
      refreshActivityData(),
      refreshLoginData(),
      loadRecentActivities()
    ])
  } catch (error) {
    console.error('Error loading dashboard data:', error)
  } finally {
    statsLoading.value = false
  }
}

const refreshActivityData = async () => {
  try {
    activityLoading.value = true
    await statsStore.loadActivityData('7d')
  } finally {
    activityLoading.value = false
  }
}

const refreshLoginData = async () => {
  try {
    loginLoading.value = true
    await statsStore.loadLoginDistribution(loginTimeRange.value)
  } finally {
    loginLoading.value = false
  }
}

const loadRecentActivities = async () => {
  try {
    activitiesLoading.value = true
    await statsStore.loadRecentActivities(10)
  } finally {
    activitiesLoading.value = false
  }
}

// Lifecycle
onMounted(() => {
  loadDashboardData()
})
</script>