import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

// Layout components
import AppLayout from '@/components/layout/AppLayout.vue'

// Views
import Login from '@/views/auth/Login.vue'
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
    // Authentication
    {
      path: '/login',
      name: 'Login',
      component: Login,
      meta: {
        requiresAuth: false,
        layout: 'auth'
      }
    },

    // Main application routes
    {
      path: '/',
      component: AppLayout,
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          redirect: '/dashboard'
        },
        {
          path: 'dashboard',
          name: 'Dashboard',
          component: Dashboard,
          meta: {
            breadcrumb: [{ name: 'Dashboard' }]
          }
        },

        // User Management
        {
          path: 'users',
          children: [
            {
              path: '',
              name: 'UserList',
              component: UserList,
              meta: {
                breadcrumb: [{ name: 'Benutzer' }]
              }
            },
            {
              path: 'create',
              name: 'UserCreate',
              component: UserCreate,
              meta: {
                breadcrumb: [
                  { name: 'Benutzer', href: '/users' },
                  { name: 'Erstellen' }
                ]
              }
            },
            {
              path: ':id',
              name: 'UserEdit',
              component: UserEdit,
              props: true,
              meta: {
                breadcrumb: [
                  { name: 'Benutzer', href: '/users' },
                  { name: 'Bearbeiten' }
                ]
              }
            },
            {
              path: 'import',
              name: 'UserImport',
              component: UserImport,
              meta: {
                breadcrumb: [
                  { name: 'Benutzer', href: '/users' },
                  { name: 'Import' }
                ]
              }
            }
          ]
        },

        // Organizations
        {
          path: 'organizations',
          children: [
            {
              path: '',
              name: 'OrganizationList',
              component: OrganizationList,
              meta: {
                breadcrumb: [{ name: 'Organisationen' }]
              }
            },
            {
              path: ':id',
              name: 'OrganizationDetail',
              component: OrganizationDetail,
              props: true,
              meta: {
                breadcrumb: [
                  { name: 'Organisationen', href: '/organizations' },
                  { name: 'Details' }
                ]
              }
            }
          ]
        },


        // OAuth2 Clients
        {
          path: 'clients',
          children: [
            {
              path: '',
              name: 'ClientList',
              component: ClientList,
              meta: {
                breadcrumb: [{ name: 'OAuth2 Clients' }]
              }
            },
            {
              path: 'create',
              name: 'ClientCreate',
              component: ClientCreate,
              meta: {
                breadcrumb: [
                  { name: 'OAuth2 Clients', href: '/clients' },
                  { name: 'Erstellen' }
                ]
              }
            },
            {
              path: ':id',
              name: 'ClientDetail',
              component: ClientDetail,
              props: true,
              meta: {
                breadcrumb: [
                  { name: 'OAuth2 Clients', href: '/clients' },
                  { name: 'Details' }
                ]
              }
            }
          ]
        },

        // Claims Management
        {
          path: 'claims',
          children: [
            {
              path: '',
              name: 'ClaimsList',
              component: ClaimsList,
              meta: {
                breadcrumb: [{ name: 'Claims' }]
              }
            },
            {
              path: 'registry',
              name: 'ClaimsRegistry',
              component: ClaimsRegistry,
              meta: {
                breadcrumb: [
                  { name: 'Claims', href: '/claims' },
                  { name: 'Registry' }
                ]
              }
            }
          ]
        },

        // Activity & Audit
        {
          path: 'audit',
          name: 'AuditLog',
          component: AuditLog,
          meta: {
            breadcrumb: [{ name: 'Audit-Log' }]
          }
        },
        {
          path: 'sessions',
          name: 'ActiveSessions',
          component: ActiveSessions,
          meta: {
            breadcrumb: [{ name: 'Aktive Sessions' }]
          }
        },
        {
          path: 'analytics',
          name: 'Analytics',
          component: Analytics,
          meta: {
            breadcrumb: [{ name: 'Login-Statistiken' }]
          }
        },

        // System Management
        {
          path: 'system',
          children: [
            {
              path: 'status',
              name: 'SystemStatus',
              component: SystemStatus,
              meta: {
                breadcrumb: [
                  { name: 'System', href: '/system' },
                  { name: 'Status' }
                ]
              }
            },
            {
              path: 'config',
              name: 'SystemConfig',
              component: SystemConfig,
              meta: {
                breadcrumb: [
                  { name: 'System', href: '/system' },
                  { name: 'Konfiguration' }
                ]
              }
            }
          ]
        },

        // Profile
        {
          path: 'profile',
          name: 'Profile',
          component: Profile,
          meta: {
            breadcrumb: [{ name: 'Profil' }]
          }
        }
      ]
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
    // Try to restore auth state if not already loaded
    if (!authStore.isAuthenticated && !authStore.isLoading) {
      await authStore.checkAuth()
    }

    // Redirect to login if not authenticated
    if (!authStore.isAuthenticated) {
      next({
        name: 'Login',
        query: { redirect: to.fullPath }
      })
      return
    }
  }

  // Redirect authenticated users away from login page
  if (to.name === 'Login' && authStore.isAuthenticated) {
    next({ name: 'Dashboard' })
    return
  }

  next()
})

export default router