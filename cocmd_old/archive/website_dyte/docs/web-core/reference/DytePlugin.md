---
sidebar_position: 12
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdPlugin"></a>

The cocmdPlugin module represents a single plugin in the meeting. A plugin can be
obtained from one of the plugin arrays in `meeting.plugins`. For example,

```ts
const plugin1 = meeting.plugins.active.get(pluginId);
const plugin2 = meeting.plugins.all.get(pluginId);
```

- [cocmdPlugin](#module_cocmdPlugin)
  - [.sendIframeEvent(message)](#module_cocmdPlugin+sendIframeEvent)
  - [.sendData(payload)](#module_cocmdPlugin+sendData)
  - [.removePluginView(viewId)](#module_cocmdPlugin+removePluginView)
  - [.addPluginView(iframe, viewId)](#module_cocmdPlugin+addPluginView)
  - [.activateForSelf()](#module_cocmdPlugin+activateForSelf)
  - [.deactivateForSelf()](#module_cocmdPlugin+deactivateForSelf)
  - ~~[.enable()](#module_cocmdPlugin+enable)~~
  - ~~[.disable()](#module_cocmdPlugin+disable)~~
  - [.activate()](#module_cocmdPlugin+activate)
  - [.deactivate()](#module_cocmdPlugin+deactivate)

<a name="module_cocmdPlugin+sendIframeEvent"></a>

### plugin.sendIframeEvent(message)

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)

| Param   | Description                              |
| ------- | ---------------------------------------- |
| message | Socket message forwarded to this plugin. |

<a name="module_cocmdPlugin+sendData"></a>

### plugin.sendData(payload)

This method is used to send arbitrary data to the plugin.

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)

| Param             | Description                                                            |
| ----------------- | ---------------------------------------------------------------------- |
| payload           | The payload that you want to send inside the plugin.                   |
| payload.eventName | Name of the event. This is used to listen for the event in plugin SDK. |
| payload.data      | Data you wish to emit. It can assume any data type.                    |

<a name="module_cocmdPlugin+removePluginView"></a>

### plugin.removePluginView(viewId)

This method is used for cleaning up event listeners attached to an iframe. It
must be used before the iframe is removed from the DOM.

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)

| Param  | Default              | Description                                                        |
| ------ | -------------------- | ------------------------------------------------------------------ |
| viewId | <code>default</code> | ID of the view corresponding to this iframe. Default is 'default'. |

<a name="module_cocmdPlugin+addPluginView"></a>

### plugin.addPluginView(iframe, viewId)

This method adds the communication layer between the plugin inside the iframe and
the core application (meeting object) in the main window.

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)

| Param  | Default              | Description                                                        |
| ------ | -------------------- | ------------------------------------------------------------------ |
| iframe |                      | Iframe element to display this plugin.                             |
| viewId | <code>default</code> | ID of the view corresponding to this iframe. Default is 'default'. |

<a name="module_cocmdPlugin+activateForSelf"></a>

### plugin.activateForSelf()

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)  
<a name="module_cocmdPlugin+deactivateForSelf"></a>

### plugin.deactivateForSelf()

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)  
<a name="module_cocmdPlugin+enable"></a>

### ~~plugin.enable()~~

**_Deprecated_**

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)  
<a name="module_cocmdPlugin+disable"></a>

### ~~plugin.disable()~~

**_Deprecated_**

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)  
<a name="module_cocmdPlugin+activate"></a>

### plugin.activate()

Activate this plugin for all participants.

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)  
<a name="module_cocmdPlugin+deactivate"></a>

### plugin.deactivate()

Deactivate this plugin for all participants.

**Kind**: instance method of [<code>cocmdPlugin</code>](#module_cocmdPlugin)
