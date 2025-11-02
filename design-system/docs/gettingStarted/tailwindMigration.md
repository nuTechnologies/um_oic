---
title: Tailwind migration
next: false
---

# Tailwind migration

Starting with `v4` of xwork Web and the design-system, the custom utility classes and variables have been removed in favor of [Tailwind](https://tailwindcss.com/). We made this decision because great CSS frameworks like Tailwind are already out there and we feel like there is no need to reinvent the wheel. We want as little custom code as possible to be able to develop fast and with high quality while providing a well documented developer experience. The latter is given via [Tailwind's extensive documentation](https://tailwindcss.com/docs).

The following guide shows how to migrate your application or extension to `v4` of xwork Web and its design-system.

> [!NOTE]
> When using Tailwind classes within a Web extension, it's recommended to use the Tailwind default config provided by the extension-sdk via `@xwork-eu/extension-sdk/tailwind.css`. This however requires you to prefix all Tailwind classes with `ext:` to avoid style conflicts.

## Utility classes

### Spacing

| nu utility class | Tailwind utility class |
| ---------------- | ---------------------- |
| `nu-m-xs`        | `m-1`                  |
| `nu-m-s`         | `m-2`                  |
| `nu-m-m`         | `m-4`                  |
| `nu-m-l`         | `m-6`                  |
| `nu-m-xl`        | `m-12`                 |
| `nu-m-xxl`       | `m-24`                 |
| `nu-m`           | `m-4`                  |
| `nu-m-rm`        | `m-0`                  |

The same goes for the `padding` utility classes (replace `m` with `p`).

### Text size

| nu utility class | Tailwind utility class |
| ---------------- | ---------------------- |
| `nu-text-xsmall` | `text-xs`              |
| `nu-text-small`  | `text-sm`              |
| `nu-text-medium` | `text-base`            |
| `nu-text-large`  | `text-lg`              |
| `nu-text-xlarge` | `text-xl`              |

### Font weight

| nu utility class   | Tailwind utility class |
| ------------------ | ---------------------- |
| `nu-font-semibold` | `font-semibold`        |
| `nu-text-bold`     | `font-semibold`        |

### Text alignment

| nu utility class | Tailwind utility class |
| ---------------- | ---------------------- |
| `nu-text-center` | `text-center`          |
| `nu-text-left`   | `text-left`            |
| `nu-text-right`  | `text-right`           |

### Word breaks and truncation

| nu utility class   | Tailwind utility class |
| ------------------ | ---------------------- |
| `nu-text-truncate` | `truncate`             |
| `nu-text-nowrap`   | `whitespace-nowrap`    |

### Line height

The `line-height` gets determined by the given `text-` class. However, it can also be adjusted separately via the `leading-` classes (see https://tailwindcss.com/docs/line-height#setting-independently).

### Text decoration

For `text-decoration` we didn't have a utility class. When a decoration is needed on hover, just use `hover:underline`. Disabling the underline is usually not needed since this is already done via the Tailwind preflight.

### Colors

The theme color roles now exist as Tailwind variables and can be used like so: `bg-role-primary`, `text-role-on-primary`. This also supports variations, e.g. `hover:text-role-on-primary` or `bg-role-primary/50`.

These nu-helper classes are now redundant:

| nu utility class | Tailwind utility class         |
| ---------------- | ------------------------------ |
| `nu-text-muted`  | `text-role-on-surface-variant` |
| `nu-text-error`  | `text-role-on-error`           |

### Borders

| nu utility class | Tailwind utility class |
| ---------------- | ---------------------- |
| `nu-border`      | `border`               |
| `nu-rounded`     | `rounded-sm`           |

For more variants, please refer to https://tailwindcss.com/docs/border-width.

### Width

| nu utility class  | Tailwind utility class |
| ----------------- | ---------------------- |
| `nu-width-1-1`    | `w-full`               |
| `nu-width-1`      | `w-full`               |
| `nu-width-large`  | `w-lg`                 |
| `nu-width-medium` | `w-sm`                 |
| `nu-width-small`  | `w-xs`                 |
| `nu-width-expand` | `flex-1`               |
| `nu-width-auto`   | `w-auto`               |

### Height

| nu utility class     | Tailwind utility class |
| -------------------- | ---------------------- |
| `nu-height-1-1`      | `h-full`               |
| `nu-height-viewport` | `h-screen`             |
| `nu-height-small`    | `h-[150px]`            |
| `nu-height-medium`   | `h-[300px]`            |
| `nu-height-large`    | `h-[450px]`            |

### Display

| nu utility class          | Tailwind utility class |
| ------------------------- | ---------------------- |
| `nu-display-block`        | `block`                |
| `nu-display-inline-block` | `inline-block`         |

### Flex

| nu utility class         | Tailwind utility class |
| ------------------------ | ---------------------- |
| `nu-flex`                | `flex`                 |
| `nu-flex-inline`         | `inline-flex`          |
| `nu-flex-left`           | `justify-start`        |
| `nu-flex-center`         | `justify-center`       |
| `nu-flex-right`          | `justify-end`          |
| `nu-flex-between`        | `justify-between`      |
| `nu-flex-around`         | `justify-around`       |
| `nu-flex-top`            | `items-start`          |
| `nu-flex-middle`         | `items-center`         |
| `nu-flex-bottom`         | `items-end`            |
| `nu-flex-stretch`        | `items-stretch`        |
| `nu-flex-row`            | `flex-row`             |
| `nu-flex-row-reverse`    | `flex-row-reverse`     |
| `nu-flex-column`         | `flex-col`             |
| `nu-flex-column-reverse` | `flex-col-reverse`     |
| `nu-flex-nowrap`         | `flex-nowrap`          |
| `nu-flex-wrap`           | `flex-wrap`            |
| `nu-flex-wrap-reverse`   | `flex-wrap-reverse`    |
| `nu-flex-1`              | `flex-1`               |

### Overflow

| nu utility class     | Tailwind utility class                |
| -------------------- | ------------------------------------- |
| `nu-overflow-hidden` | `overflow-hidden`                     |
| `nu-overflow-auto`   | `overflow-auto`                       |
| `nu-text-overflow`   | `max-w-full` and/or `overflow-hidden` |

### Position

| nu utility class            | Tailwind utility class                                            |
| --------------------------- | ----------------------------------------------------------------- |
| `nu-position-relative`      | `relative`                                                        |
| `nu-position-fixed`         | `fixed`                                                           |
| `nu-position-absolute`      | `absolute`                                                        |
| `nu-position-center`        | `absolute top-[50%] left-[50%] transform-[translate(-50%, -50%)]` |
| `nu-position-center-right`  | `absolute top-[50%] right-0 transform-[translateY(-50%)]`         |
| `nu-position-bottom-center` | `absolute left-[50%] bottom-0 transform-[translateX(-50%)]`       |
| `nu-position-cover`         | `absolute inset-0`                                                |

### Visibility

| nu utility class  | Tailwind utility class |
| ----------------- | ---------------------- |
| `nu-invisible-sr` | `sr-only`              |
| `nu-invisible`    | `invisible`            |
| `nu-hidden`       | `hidden`               |

### Cursor

| nu utility class    | Tailwind utility class |
| ------------------- | ---------------------- |
| `nu-cursor-pointer` | `cursor-pointer`       |

### Box-shadow

| nu utility class       | Tailwind utility class |
| ---------------------- | ---------------------- |
| `nu-box-shadow-medium` | `shadow-md/20`         |

## Custom CSS

The philosophy of Tailwind is to use utility classes as much as possible. However, in rare occasions where you need to write custom CSS, it's recommended to put your styles into Tailwind's layer system.

### Example

```html
<style>
  @reference '@xwork-eu/design-system/dist/tailwind.css';

  @layer components {
    .element {
      @apply text-small;
    }
  }
</style>
```

The `reference` is needed so Tailwind classes like `text-small` are recognized when working with `@apply`. Still, you can also use layers without that:

```html
<style>
  @layer components {
    .element {
      font-size: 12px;
    }
  }
</style>
```

Please refer to https://tailwindcss.com/docs/adding-custom-styles#using-custom-css for more information on layers and how to write custom CSS with Tailwind.

## OcCard

`OcCard` is now a dedicated component instead of a class.

```ts
<div class="nu-card"> // [!code --]
<nu-card> // [!code ++]
  <div class="nu-card-header"> // [!code --]
    <template #header>  // [!code ++]
    <h2>Card title</h2>
  </div> // [!code --]
  </template> // [!code ++]
  <div class="nu-card-body"> // [!code --]
    <p>Some body content</p>
  </div> // [!code --]
  <div class="nu-card-footer"> // [!code --]
  <template #footer>  // [!code ++]
    <p>Some footer</p>
  </div> // [!code --]
  </template> // [!code ++]
</div> // [!code --]
</nu-card> // [!code ++]
```

Please refer to the [OcCard docs](../components/OcCard/OcCard.md) for more details on how to use this component.

## OcGrid

The `OcGrid` component has been removed. Please use the [Tailwind grid layout](https://tailwindcss.com/docs/grid-template-columns) instead.

## Mixins

The following mixins have been removed:

- `nu-form-check-size`
- `nu-icon-size`
- `nu-spinner-size`

## Media queries (breakpoints)

The custom breakpoint variables have been removed:

- `$nu-breakpoint-xsmall-max`
- `$nu-breakpoint-small-default`
- `$nu-breakpoint-small-max`
- `$nu-breakpoint-medium-default`
- `$nu-breakpoint-medium-max`
- `$nu-breakpoint-large-default`
- `$nu-breakpoint-large-max`
- `$nu-breakpoint-xlarge`

Please use the corresponding [Tailwind utilities](https://tailwindcss.com/docs/responsive-design) instead. The old breakpoints have been mapped to those.

```ts
<div class="element" /> // [!code --]
<div class="hidden sm:block" /> // [!code ++]

<style> // [!code --]
  .element { // [!code --]
    display: block; // [!code --]
  } // [!code --]
  @media (max-width: $nu-breakpoint-small-default) { // [!code --]
    .element { // [!code --]
      display: none; // [!code --]
    } // [!code --]
  } // [!code --]
</style> // [!code --]
```

## Theming options

The theming options for breakpoints, spacing, fontSizes and sizes have been removed. They just added unnecessary complexity and are not needed with Tailwind.
