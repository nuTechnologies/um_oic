import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/services/api'

export interface Group {
  id: string
  name: string
  description?: string
  org: string
  members: string[] // User IDs
  created_at: string
  updated_at: string
}

export interface CreateGroupRequest {
  name: string
  description?: string
  org: string
  members?: string[]
}

export interface UpdateGroupRequest {
  name?: string
  description?: string
  members?: string[]
}

export const useGroupsStore = defineStore('groups', () => {
  const groups = ref<Group[]>([])
  const currentGroup = ref<Group | null>(null)
  const isLoading = ref(false)

  const groupsByOrg = computed(() => {
    const grouped: Record<string, Group[]> = {}
    groups.value.forEach(group => {
      if (!grouped[group.org]) {
        grouped[group.org] = []
      }
      grouped[group.org].push(group)
    })
    return grouped
  })

  const activeGroupsCount = computed(() => {
    return groups.value.length
  })

  const loadGroups = async (orgId?: string): Promise<void> => {
    isLoading.value = true
    try {
      const params = orgId ? { org: orgId } : {}
      const response = await api.get('/groups', { params })
      groups.value = response.data
    } catch (error) {
      console.error('Failed to load groups:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  const loadGroup = async (groupId: string): Promise<Group> => {
    try {
      const response = await api.get(`/groups/${groupId}`)
      currentGroup.value = response.data
      return response.data
    } catch (error) {
      console.error('Failed to load group:', error)
      throw error
    }
  }

  const createGroup = async (groupData: CreateGroupRequest): Promise<Group> => {
    try {
      const response = await api.post('/groups', groupData)
      const newGroup = response.data

      // Add to local groups array
      groups.value.push(newGroup)

      return newGroup
    } catch (error) {
      console.error('Failed to create group:', error)
      throw error
    }
  }

  const updateGroup = async (groupId: string, groupData: UpdateGroupRequest): Promise<Group> => {
    try {
      const response = await api.put(`/groups/${groupId}`, groupData)
      const updatedGroup = response.data

      // Update in local groups array
      const index = groups.value.findIndex(group => group.id === groupId)
      if (index !== -1) {
        groups.value[index] = updatedGroup
      }

      // Update current group if it's the same
      if (currentGroup.value?.id === groupId) {
        currentGroup.value = updatedGroup
      }

      return updatedGroup
    } catch (error) {
      console.error('Failed to update group:', error)
      throw error
    }
  }

  const deleteGroup = async (groupId: string): Promise<void> => {
    try {
      await api.delete(`/groups/${groupId}`)

      // Remove from local groups array
      groups.value = groups.value.filter(group => group.id !== groupId)

      // Clear current group if it's the same
      if (currentGroup.value?.id === groupId) {
        currentGroup.value = null
      }
    } catch (error) {
      console.error('Failed to delete group:', error)
      throw error
    }
  }

  const addMember = async (groupId: string, userId: string): Promise<void> => {
    try {
      await api.post(`/groups/${groupId}/members`, { user_id: userId })

      // Update local group
      const group = groups.value.find(g => g.id === groupId)
      if (group && !group.members.includes(userId)) {
        group.members.push(userId)
      }

      if (currentGroup.value?.id === groupId && !currentGroup.value.members.includes(userId)) {
        currentGroup.value.members.push(userId)
      }
    } catch (error) {
      console.error('Failed to add group member:', error)
      throw error
    }
  }

  const removeMember = async (groupId: string, userId: string): Promise<void> => {
    try {
      await api.delete(`/groups/${groupId}/members/${userId}`)

      // Update local group
      const group = groups.value.find(g => g.id === groupId)
      if (group) {
        group.members = group.members.filter(id => id !== userId)
      }

      if (currentGroup.value?.id === groupId) {
        currentGroup.value.members = currentGroup.value.members.filter(id => id !== userId)
      }
    } catch (error) {
      console.error('Failed to remove group member:', error)
      throw error
    }
  }

  return {
    groups,
    currentGroup,
    isLoading,
    groupsByOrg,
    activeGroupsCount,
    loadGroups,
    loadGroup,
    createGroup,
    updateGroup,
    deleteGroup,
    addMember,
    removeMember
  }
})