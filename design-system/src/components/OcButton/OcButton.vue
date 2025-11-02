<template>
  <component
    :is="type"
    v-bind="additionalAttributes"
    :aria-label="ariaLabel"
    :class="[
      `nu-button-${kebabCase(colorRole)}`,
      `nu-button-${appearance}`,
      `nu-button-${kebabCase(colorRole)}-${appearance}`,
      {
        ...getTailwindGapClass(gapSize),
        ...getTailwindJustifyContentClass(justifyContent),
        // size
        'text-sm min-h-3': size === 'small',
        'text-base min-h-4': size === 'medium',
        'text-lg min-h-7': size === 'large',
        // hover
        'no-hover': noHover
      }
    ]"
    class="nu-button cursor-pointer disabled:opacity-60 disabled:cursor-default"
    v-on="handlers"
  >
    <nu-spinner v-if="showSpinner" size="small" class="spinner" />
    <!-- @slot Content of the button -->
    <slot />
  </component>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { RouteLocationRaw } from 'vue-router'
import { AppearanceType, JustifyContentType, SizeType } from '../../helpers'
import { kebabCase } from 'lodash-es'
import { getTailwindGapClass, getTailwindJustifyContentClass } from '../../helpers/tailwind'

export interface Props {
  /**
   * @docs The appearance of the button.
   * @default outline
   */
  appearance?: AppearanceType
  /**
   * @docs The aria label of the button. Needs to be present if the button doesn't have a visible label.
   */
  ariaLabel?: string
  /**
   * @docs Material design color role.
   * @default secondary
   */
  colorRole?:
    | 'primary'
    | 'primaryContainer'
    | 'primaryFixed'
    | 'secondary'
    | 'secondaryContainer'
    | 'secondaryFixed'
    | 'tertiary'
    | 'tertiaryContainer'
    | 'tertiaryFixed'
    | 'surface'
    | 'surfaceContainer'
    | 'chrome'
  /**
   * @docs Determines if the button is disabled.
   * @default false
   */
  disabled?: boolean
  /**
   * @docs The gap size between content elements of the button.
   * @default medium
   */
  gapSize?: SizeType | 'none'
  /**
   * @docs The href if the `type` is set to `a'.
   */
  href?: string
  /**
   * @docs The alignment of the button content.
   * @default center
   */
  justifyContent?: JustifyContentType
  /**
   * @docs Determines if a spinner should be shown inside the button.
   * @default false
   */
  showSpinner?: boolean
  /**
   * @docs The size of the button.
   * @default medium
   */
  size?: 'small' | 'medium' | 'large'
  /**
   * @docs The type of the button element. Only takes effect if the `type` is set to `button`.
   * @default button
   */
  submit?: 'null' | 'button' | 'submit' | 'reset'
  /**
   * @docs The target of the button if the `type` is set to `a`.
   */
  target?: '_blank' | '_self' | '_parent' | '_top'
  /**
   * @docs The route location if the `type` is set to `router-link`.
   */
  to?: RouteLocationRaw
  /**
   * @docs The type of the button element.
   * @default button
   */
  type?: 'button' | 'a' | 'router-link'
  /**
   * @docs Determines if the button should have no hover effect.
   * @default false
   */
  noHover?: boolean
}

export interface Emits {
  /**
   * @docs Emitted when the button has been clicked.
   */
  (e: 'click', event: MouseEvent): void
}

export interface Slots {
  /**
   * @docs Button content.
   */
  default?: () => unknown
}

const {
  appearance = 'outline',
  ariaLabel,
  colorRole = 'secondary',
  disabled = false,
  gapSize = 'medium',
  href,
  justifyContent = 'center',
  showSpinner = false,
  size = 'medium',
  submit = 'button',
  target,
  to,
  type = 'button',
  noHover = false
} = defineProps<Props>()

const emit = defineEmits<Emits>()
defineSlots<Slots>()

const additionalAttributes = computed(() => {
  return {
    ...(href && { href }),
    ...(target && { target }),
    ...(to && { to }),
    ...(type === 'button' && { type: submit }),
    ...(type === 'button' && { disabled })
  }
})

const handlers = computed(() => {
  return {
    ...(type === 'button' && { click: onClick })
  }
})

const onClick = (event: MouseEvent) => {
  emit('click', event)
}
</script>

<style>
@reference '@xwork-eu/design-system/tailwind';

@layer components {
  .nu-button:not(.nu-button-raw, .nu-button-raw-inverse) {
    @apply py-1.5 px-2.5;
  }
  .nu-button {
    @apply rounded-sm items-center inline-flex;
  }
  .nu-button-group {
    @apply inline-flex flex-row flex-wrap rounded-sm outline outline-role-secondary outline-offset-[-1px];
  }
  .nu-button-group .nu-button {
    @apply rounded-none first:rounded-l-sm last:rounded-r-sm outline-0;
  }
}
</style>
<style lang="scss">
@mixin nu-button-color-role($color, $on-color) {
  &-raw,
  &-raw-inverse {
    background-color: transparent;
    color: $color;
    .nu-icon > svg {
      fill: $color;
    }

    &:focus:not([disabled]):not(button),
    &:hover:not([disabled]):not(button) {
      background-color: transparent;
    }

    &:focus:not([disabled]):not(.active):not(.no-hover),
    &:hover:not([disabled]):not(.active):not(.no-hover) {
      background-color: var(--nu-role-surface-container);
      color: var(--nu-role-on-surface);
      .nu-icon > svg {
        fill: var(--nu-role-on-surface);
      }
    }
  }
  &-raw-inverse {
    color: $on-color;
    .nu-icon > svg {
      fill: $on-color;
    }
  }

  &-filled {
    background-color: $color;
    color: $on-color !important;
    .nu-icon > svg {
      fill: $on-color;
    }
  }

  &-outline {
    outline: 1px solid $color;
    outline-offset: -1px;
    background-color: transparent;
    color: $color;
    .nu-icon > svg {
      fill: $color;
    }
  }
}

.nu-button {
  @layer components {
    &-primary {
      @include nu-button-color-role(var(--nu-role-primary), var(--nu-role-on-primary));
    }
    &-primary-container {
      @include nu-button-color-role(
        var(--nu-role-primary-container),
        var(--nu-role-on-primary-container)
      );
    }
    &-primary-fixed {
      @include nu-button-color-role(var(--nu-role-primary-fixed), var(--nu-role-on-primary-fixed));
    }
    &-secondary {
      @include nu-button-color-role(var(--nu-role-secondary), var(--nu-role-on-secondary));
    }
    &-secondary-container {
      @include nu-button-color-role(
        var(--nu-role-secondary-container),
        var(--nu-role-on-secondary-container)
      );
    }
    &-secondary-fixed {
      @include nu-button-color-role(
        var(--nu-role-secondary-fixed),
        var(--nu-role-on-secondary-fixed)
      );
    }
    &-tertiary {
      @include nu-button-color-role(var(--nu-role-tertiary), var(--nu-role-on-tertiary));
    }
    &-tertiary-container {
      @include nu-button-color-role(
        var(--nu-role-tertiary-container),
        var(--nu-role-on-tertiary-container)
      );
    }
    &-tertiary-fixed {
      @include nu-button-color-role(
        var(--nu-role-tertiary-fixed),
        var(--nu-role-on-tertiary-fixed)
      );
    }
    &-surface {
      @include nu-button-color-role(var(--nu-role-surface), var(--nu-role-on-surface));
    }
    &-surface-container {
      @include nu-button-color-role(var(--nu-role-surface-container), var(--nu-role-on-surface));
    }
    &-chrome {
      @include nu-button-color-role(var(--nu-role-chrome), var(--nu-role-on-chrome));
    }

    &:hover:not(.no-hover, .nu-button-raw-inverse, .nu-button-raw, .active, .selected, [disabled]) {
      filter: brightness(85%);
    }

    &-outline:hover:not(.no-hover, [disabled]) {
      background-color: var(--nu-role-surface-container);
      filter: none !important;
    }
  }
}
.quick-action-button,
.raw-hover-surface {
  &:hover {
    // overwrite default hover with an inverted one for buttons on backgrounds that have the default hover color
    background-color: var(--nu-role-surface) !important;
  }
}
</style>
