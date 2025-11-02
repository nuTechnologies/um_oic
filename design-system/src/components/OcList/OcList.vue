<template>
  <ul class="nu-list" :class="{ 'nu-list-raw': raw }">
    <slot />
  </ul>
</template>

<script setup lang="ts">
export interface Props {
  /**
   * @docs Render the list without any list style type.
   * @default false
   */
  raw?: boolean
}

export interface Slots {
  /**
   * @docs Content of the list, usually a bunch of HTML `li` elements.
   */
  default?: () => unknown
}

const { raw = false } = defineProps<Props>()

defineSlots<Slots>()
</script>
<style>
@reference '@xwork-eu/design-system/tailwind';

@layer components {
  ul.nu-list,
  ul.nu-list.nu-timeline {
    @apply m-0 p-0;
  }
  ul.nu-list.nu-timeline {
    @apply relative before:absolute before:inset-0;
  }
  ul.nu-list-divider > :nth-child(n + 2) {
    @apply mt-2 pt-2 border-t;
  }
  ul.nu-list.nu-timeline li {
    @apply py-2 pl-8 pr-7 flex flex-col before:rounded-[50%] w-full relative;
  }
  ul.nu-list.nu-timeline li:before {
    @apply absolute;
    left: -4px;
    top: 50%;
  }
  ul.nu-list.nu-timeline::before,
  ul.nu-list.nu-timeline li::before {
    @apply bg-role-outline-variant;
  }
  ul.nu-list.nu-timeline::before {
    width: 1.5px;
    content: '';
  }
  ul.nu-list.nu-timeline li::before {
    @apply size-2.5;
    transform: translateY(-50%);
    content: '';
  }
  ul.nu-list-raw a:hover {
    @apply text-inherit;
  }
  .nu-list li::before {
    @apply z-1;
  }
}
</style>
