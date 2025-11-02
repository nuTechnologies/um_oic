---
title: OcNotifications component
next: false
prev: false
---

# OcNotifications component

## Description

The `OcNotifications` component serves as a container for the [OcNotificationMessage](./OcNotificationMessage) component. It can be positioned in different places on the screen.

## Examples

### Default

The basic position defaults to `default`, meaning the notification gets displayed in place. Other possible positions are: `top-right`, `top-left` and `top-center`.

::: livecode

```html
<nu-notifications>
  <nu-notification-message title="Folder has been created successfully." />
</nu-notifications>
```

:::

::: component-api
