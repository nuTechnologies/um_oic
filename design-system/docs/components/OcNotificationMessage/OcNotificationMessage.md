---
title: OcNotificationMessage component
next: false
prev: false
---

# OcNotificationMessage component

## Description

The `OcNotificationMessage` component can be used to display notification messages to the user.
You usually want to wrap it inside the [OcNotifications](./OcNotifications) component which serves as a container that can be positioned in different places on the screen.

## Examples

### Default

The basic usage of the component needs a `title` property. An optional `message` can be provided to give more context to the user.

::: livecode

```html
<nu-notification-message title="Folder has been created successfully." />
<nu-notification-message title="Success" message="Folder has been created successfully." />
```

:::

### Statuses

There are different statuses available for the notification message: `passive`, `primary`, `success`, `warning` and `danger`.

::: livecode

```html
<nu-notification-message title="Folder has been created successfully." status="passive" />
<nu-notification-message title="Folder has been created successfully." status="primary" />
<nu-notification-message title="Folder has been created successfully." status="success" />
<nu-notification-message
  title="Folder has been created under a different location."
  status="warning"
/>
<nu-notification-message title="Folder could not be created." status="danger" />
```

:::

::: component-api
