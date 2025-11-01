// API Types for UM-OIC Admin Interface

export interface ApiResponse<T> {
  data: T
  success: boolean
  message?: string
}

export interface PaginatedResponse<T> {
  data: T[]
  pagination: {
    page: number
    limit: number
    total: number
    pages: number
  }
}

export interface ErrorResponse {
  error: string
  message: string
  details?: Record<string, any>
}

// User Management Types
export interface User {
  id: string
  email: string
  first_name: string
  last_name: string
  status: 'active' | 'inactive' | 'suspended'
  verified: boolean
  authenticated?: string
  admin: string[]
  org: string
  claims: Record<string, any>
  created_at: string
  updated_at: string
}

export interface CreateUserRequest {
  email: string
  password: string
  first_name: string
  last_name: string
  org: string
  admin?: string[]
  claims?: Record<string, any>
}

export interface UpdateUserRequest {
  first_name?: string
  last_name?: string
  status?: 'active' | 'inactive' | 'suspended'
  org?: string
  admin?: string[]
  claims?: Record<string, any>
}

export interface UserSearchParams {
  search?: string
  status?: string
  org?: string
  role?: string
  limit?: number
  page?: number
}

// Organization Types
export interface Organization {
  name: string
  user_count: number
  active_users: number
  created_at: string
}

// Group Management Types
export interface Group {
  id: string
  name: string
  description: string
  metadata: Record<string, any>
  created_at: string
}

export interface CreateGroupRequest {
  id: string
  name: string
  description: string
  metadata?: Record<string, any>
}

export interface UpdateGroupRequest {
  name?: string
  description?: string
  metadata?: Record<string, any>
}

// OAuth2 Client Types
export interface Client {
  client_id: string
  name: string
  client_type: 'public' | 'confidential'
  redirect_uris: string[]
  allowed_scopes: string[]
  require_pkce: boolean
  grant_types: string[]
  created_at: string
}

export interface CreateClientRequest {
  client_id: string
  name: string
  client_type: 'public' | 'confidential'
  redirect_uris: string[]
  allowed_scopes: string[]
  require_pkce?: boolean
  grant_types?: string[]
}

export interface UpdateClientRequest {
  name?: string
  redirect_uris?: string[]
  allowed_scopes?: string[]
  require_pkce?: boolean
  grant_types?: string[]
}

export interface ClientSecret {
  client_secret: string
  expires_at?: string
}

// Claims Registry Types
export interface ClaimsRegistry {
  claims: Record<string, ClaimDefinition>
}

export interface ClaimDefinition {
  type: string
  items?: any
  description: string
  default_allowed: boolean
  required?: boolean
  sensitive?: boolean
  admin_only?: boolean
}

// Audit Types
export interface AuditEvent {
  id: string
  user_id?: string
  org?: string
  event_type: string
  ip_address?: string
  user_agent?: string
  metadata: Record<string, any>
  created_at: string
}

export interface AuditSearchParams {
  user_id?: string
  org?: string
  event_type?: string
  from?: string
  to?: string
  limit?: number
  page?: number
}

// System Types
export interface SystemStatus {
  status: 'healthy' | 'degraded' | 'unhealthy'
  auth_data_stale: boolean
  last_auth_reload?: string
  last_data_update: string
  users_count: number
  clients_count: number
  organizations_count: number
  version: string
  uptime: number
}

export interface SystemConfig {
  instance: {
    name: string
    logo_url?: string
    primary_color?: string
    issuer: string
  }
  security: {
    password_min_length: number
    access_token_ttl: number
    refresh_token_ttl: number
    require_mfa: boolean
  }
  features: {
    allow_registration: boolean
    allow_password_reset: boolean
  }
}

// Statistics Types
export interface UserStatistics {
  total_users: number
  active_users: number
  verified_users: number
  admin_users: number
  users_by_org: Record<string, number>
  users_by_status: Record<string, number>
  recent_registrations: Array<{
    date: string
    count: number
  }>
}

export interface OrganizationStatistics {
  total_organizations: number
  organizations: Array<{
    name: string
    user_count: number
    active_users: number
    admin_count: number
  }>
}

export interface ActivityStatistics {
  login_events: Array<{
    date: string
    count: number
    unique_users: number
  }>
  popular_times: Array<{
    hour: number
    count: number
  }>
  failed_logins: number
  password_resets: number
}

// Bulk Operations
export interface BulkUserOperation {
  operation: 'create' | 'update' | 'delete' | 'activate' | 'deactivate'
  users: string[] | CreateUserRequest[]
  data?: Partial<UpdateUserRequest>
}

export interface BulkOperationResult {
  success_count: number
  error_count: number
  errors: Array<{
    user_id?: string
    email?: string
    error: string
  }>
}

// User Import
export interface ImportPreview {
  total_rows: number
  valid_rows: number
  invalid_rows: number
  preview_data: Array<{
    row: number
    data: CreateUserRequest
    errors?: string[]
  }>
}

export interface ImportResult {
  imported_count: number
  skipped_count: number
  error_count: number
  errors: Array<{
    row: number
    email: string
    error: string
  }>
}

// Session Management
export interface UserSession {
  session_id: string
  user_id: string
  ip_address: string
  user_agent: string
  created_at: string
  last_activity: string
  expires_at: string
}

// Advanced Search
export interface AdvancedUserSearch {
  email?: string
  name?: string
  organization?: string[]
  status?: string[]
  roles?: string[]
  claims?: Record<string, any>
  created_after?: string
  created_before?: string
  last_login_after?: string
  last_login_before?: string
  has_mfa?: boolean
  is_verified?: boolean
  is_admin?: boolean
}

export interface AdvancedAuditSearch {
  user_ids?: string[]
  organizations?: string[]
  event_types?: string[]
  ip_addresses?: string[]
  date_range: {
    from: string
    to: string
  }
  metadata_filters?: Record<string, any>
}

// Password Reset
export interface PasswordResetRequest {
  user_id: string
  send_email?: boolean
  temporary_password?: boolean
}

export interface PasswordResetResult {
  reset_token?: string
  temporary_password?: string
  email_sent: boolean
  expires_at: string
}

// MFA Management
export interface MfaStatus {
  enabled: boolean
  secret?: string
  backup_codes?: string[]
  last_used?: string
}

export interface MfaSetupRequest {
  user_id: string
  enable: boolean
  force_reset?: boolean
}