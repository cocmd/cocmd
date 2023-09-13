---
sidebar_position: 14
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdRemote"></a>

This module provides the ability to control a remotely shared screen using mouse
and keyboard. Methods are available to request, accept and end control of shared
screen.

- [cocmdRemote](#module_cocmdRemote)
  - [.requestControl(peerId)](#module_cocmdRemote+requestControl) ⇒
  - [.acceptControl(requestId)](#module_cocmdRemote+acceptControl)
  - [.endControl()](#module_cocmdRemote+endControl)

<a name="module_cocmdRemote+requestControl"></a>

### meeting.remote.requestControl(peerId) ⇒

Sends a request to a peer for remote control.

**Kind**: instance method of [<code>cocmdRemote</code>](#module_cocmdRemote)  
**Returns**: request identifier for the control request.

| Param  | Description                                               |
| ------ | --------------------------------------------------------- |
| peerId | The peer that needs to receive the remote control request |

<a name="module_cocmdRemote+acceptControl"></a>

### meeting.remote.acceptControl(requestId)

Accepts a remote control request from a peer.

**Kind**: instance method of [<code>cocmdRemote</code>](#module_cocmdRemote)

| Param     | Description                                      |
| --------- | ------------------------------------------------ |
| requestId | The request identifier that needs to be accepted |

<a name="module_cocmdRemote+endControl"></a>

### meeting.remote.endControl()

End an active remote control.

**Kind**: instance method of [<code>cocmdRemote</code>](#module_cocmdRemote)
