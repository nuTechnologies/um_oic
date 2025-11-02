import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { config } from '@/config'

// Layout components
import AppLayout from '@/components/layout/AppLayout.vue'

// Views
import Dashboard from '@/views/Dashboard.vue'

// User management
import UserList from '@/views/users/UserList.vue'
import UserCreate from '@/views/users/UserCreate.vue'
import UserEdit from '@/views/users/UserEdit.vue'
import UserImport from '@/views/users/UserImport.vue'

// Organizations
import OrganizationList from '@/views/organizations/OrganizationList.vue'
import OrganizationDetail from '@/views/organizations/OrganizationDetail.vue'


// OAuth2 Clients
import ClientList from '@/views/clients/ClientList.vue'
import ClientCreate from '@/views/clients/ClientCreate.vue'
import ClientDetail from '@/views/clients/ClientDetail.vue'

// Claims
import ClaimsList from '@/views/claims/ClaimsList.vue'
import ClaimsRegistry from '@/views/claims/ClaimsRegistry.vue'

// Activity & Audit
import AuditLog from '@/views/audit/AuditLog.vue'
import ActiveSessions from '@/views/sessions/ActiveSessions.vue'
import Analytics from '@/views/analytics/Analytics.vue'

// System
import SystemStatus from '@/views/system/SystemStatus.vue'
import SystemConfig from '@/views/system/SystemConfig.vue'

// Profile
import Profile from '@/views/profile/Profile.vue'

// Error pages
import NotFound from '@/views/errors/NotFound.vue'
import Unauthorized from '@/views/errors/Unauthorized.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '',
      redirect: '/dashboard'
    },
    // Main application routes
    {
      path: '/dashboard',
      name: 'Dashboard',
      component: Dashboard,
      meta: {
        requiresAuth: true,
        breadcrumb: [{ name: 'Dashboard' }]
      }
    },

    // User Management
    {
      path: '/users',
      name: 'UserList',
      component: UserList,
      meta: {
        requiresAuth: true,
        breadcrumb: [{ name: 'Benutzer' }]
      }
    },
    {
      path: '/users/create',
      name: 'UserCreate',
      component: UserCreate,
      meta: {
        requiresAuth: true,
        breadcrumb: [
          { name: 'Benutzer', href: '/users' },
          { name: 'Erstellen' }
        ]
      }
    },
    {
      path: '/users/:id',
      name: 'UserEdit',
      component: UserEdit,
      props: true,
      meta: {
        requiresAuth: true,
        breadcrumb: [
          { name: 'Benutzer', href: '/users' },
          { name: 'Bearbeiten' }
        ]
      }
    },
    {
      path: '/users/import',
      name: 'UserImport',
      component: UserImport,
      meta: {
        requiresAuth: true,
        breadcrumb: [
          { name: 'Benutzer', href: '/users' },
          { name: 'Import' }
        ]
      }
    },

    // Organizations
    {
      path: '/organizations',
      name: 'OrganizationList',
      component: OrganizationList,
      meta: {
        requiresAuth: true,
        breadcrumb: [{ name: 'Organisationen' }]
      }
    },
    {
      path: '/organizations/:id',
      name: 'OrganizationDetail',
      component: OrganizationDetail,
      props: true,
      meta: {
        requiresAuth: true,
        breadcrumb: [
          { name: 'Organisationen', href: '/organizations' },
          { name: 'Details' }
        ]
      }
    },

    // OAuth2 Clients
    {
      path: '/clients',
      name: 'ClientList',
      component: ClientList,
      meta: {
        requiresAuth: true,
        breadcrumb: [{ name: 'OAuth2 Clients' }]
      }
    },
    {
      path: '/clients/create',
      name: 'ClientCreate',
      component: ClientCreate,
      meta: {
        requiresAuth: true,
        breadcrumb: [
          { name: 'OAuth2 Clients', href: '/clients' },
          { name: 'Erstellen' }
        ]
      }
    },
    {
      path: '/clients/:id',
      name: 'ClientDetail',
      component: ClientDetail,
      props: true,
      meta: {
        requiresAuth: true,
        breadcrumb: [
          { name: 'OAuth2 Clients', href: '/clients' },
          { name: 'Details' }
        ]
      }
    },

    // Claims Management
    {
      path: '/claims',
      name: 'ClaimsList',
      component: ClaimsList,
      meta: {
        requiresAuth: true,
        breadcrumb: [{ name: 'Claims' }]
      }
    },
    {
      path: '/claims/registry',
      name: 'ClaimsRegistry',
      component: ClaimsRegistry,
      meta: {
        requiresAuth: true,
        breadcrumb: [
          { name: 'Claims', href: '/claims' },
          { name: 'Registry' }
        ]
      }
    },

    // Activity & Audit
    {
      path: '/audit',
      name: 'AuditLog',
      component: AuditLog,
      meta: {
        requiresAuth: true,
        breadcrumb: [{ name: 'Audit-Log' }]
      }
    },
    {
      path: '/sessions',
      name: 'ActiveSessions',
      component: ActiveSessions,
      meta: {
        requiresAuth: true,
        breadcrumb: [{ name: 'Aktive Sessions' }]
      }
    },
    {
      path: '/analytics',
      name: 'Analytics',
      component: Analytics,
      meta: {
        requiresAuth: true,
        breadcrumb: [{ name: 'Login-Statistiken' }]
      }
    },

    // System Management
    {
      path: '/system/status',
      name: 'SystemStatus',
      component: SystemStatus,
      meta: {
        requiresAuth: true,
        breadcrumb: [
          { name: 'System', href: '/system' },
          { name: 'Status' }
        ]
      }
    },
    {
      path: '/system/config',
      name: 'SystemConfig',
      component: SystemConfig,
      meta: {
        requiresAuth: true,
        breadcrumb: [
          { name: 'System', href: '/system' },
          { name: 'Konfiguration' }
        ]
      }
    },

    // Profile
    {
      path: '/profile',
      name: 'Profile',
      component: Profile,
      meta: {
        requiresAuth: true,
        breadcrumb: [{ name: 'Profil' }]
      }
    },

    // Error pages
    {
      path: '/unauthorized',
      name: 'Unauthorized',
      component: Unauthorized,
      meta: { requiresAuth: false }
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'NotFound',
      component: NotFound,
      meta: { requiresAuth: false }
    }
  ]
})

// Navigation guards
router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore()

  // Check if route requires authentication
  if (to.meta.requiresAuth !== false) {
    // Don't interfere if auth is already being checked by the app initialization
    if (authStore.isLoading) {
      // Wait for app initialization to complete
      const checkInterval = setInterval(() => {
        if (!authStore.isLoading) {
          clearInterval(checkInterval)
          // Check auth status after initialization is done
          if (authStore.isAuthenticated || authStore.token) {
            next()
          } else {
            const currentUrl = `${window.location.origin}${to.fullPath}`
            const authUrl = `${config.auth.serviceUrl}/?redirect=${encodeURIComponent(currentUrl)}`
            window.location.href = authUrl
          }
        }
      }, 100)
      return
    }

    // If we're authenticated, proceed
    if (authStore.isAuthenticated) {
      next()
      return
    }

    // If we have a token but no user, let the app initialization handle it
    if (authStore.token && !authStore.user) {
      // Give app initialization a chance to validate the token
      setTimeout(() => {
        if (!authStore.isAuthenticated) {
          const currentUrl = `${window.location.origin}${to.fullPath}`
          const authUrl = `${config.auth.serviceUrl}/?redirect=${encodeURIComponent(currentUrl)}`
          window.location.href = authUrl
        }
      }, 1000)
      next()
      return
    }

    // No token and not authenticated - redirect to auth service
    if (!authStore.token) {
      const currentUrl = `${window.location.origin}${to.fullPath}`
      const authUrl = `${config.auth.serviceUrl}/?redirect=${encodeURIComponent(currentUrl)}`
      window.location.href = authUrl
      return
    }
  }

  next()
})

export default router