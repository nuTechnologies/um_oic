---
title: OcProgressPie component
next: false
prev: false
---

# OcProgressPie component

## Description

The `OcProgressPie` component displays progress in a pie shape.

## Examples

### Default

The default usage of the component involves the `progress` and `max` properties.

::: livecode

```html
<nu-progress-pie :progress="4" :max="10" />
```

:::

### Label

A `label` can be displayed in the center of the pie.

::: livecode

```html
<nu-progress-pie :progress="4" :max="10" :show-label="true" />
```

:::

::: component-api
