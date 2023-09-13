---
sidebar_position: 1
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdClient"></a>

The cocmdClient class is the main class of the web core library. An object of the
cocmdClient class can be created using `await cocmdClient.init({ ... })`.
Typically, an object of `cocmdClient` is named `meeting`.

- [cocmdClient](#module_cocmdClient)
  - _instance_
    - [.participants](#module_cocmdClient+participants)
    - [.self](#module_cocmdClient+self)
    - [.meta](#module_cocmdClient+meta)
    - [.plugins](#module_cocmdClient+plugins)
    - [.chat](#module_cocmdClient+chat)
    - [.polls](#module_cocmdClient+polls)
    - [.remote](#module_cocmdClient+remote)
    - [.connectedMeetings](#module_cocmdClient+connectedMeetings)
    - [.**internals**](#module_cocmdClient+__internals__)
    - [.joinRoom()](#module_cocmdClient+joinRoom)
    - [.leaveRoom()](#module_cocmdClient+leaveRoom)
  - _static_
    - [.init(options)](#module_cocmdClient.init)

<a name="module_cocmdClient+participants"></a>

### meeting.participants

The `participants` object consists of 4 maps of participants, `waitlisted`,
`joined`, `active`, `pinned`. The maps are indexed by `peerId`s, and the values
are the corresponding participant objects.

**Kind**: instance property of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+self"></a>

### meeting.self

The `self` object can be used to manipulate audio and video settings, and other
configurations for the local participant. This exposes methods to enable and
disable media tracks, share the user's screen, etc.

**Kind**: instance property of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+meta"></a>

### meeting.meta

The `room` object stores information about the current meeting, such as chat
messages, polls, room name, etc.

**Kind**: instance property of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+plugins"></a>

### meeting.plugins

The `plugins` object stores information about the plugins available in the
current meeting. It exposes methods to activate and deactivate them.

**Kind**: instance property of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+chat"></a>

### meeting.chat

The chat object stores the chat messages that were sent in the meeting. This
includes text messages, images, and files.

**Kind**: instance property of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+polls"></a>

### meeting.polls

The polls object stores the polls that were initiated in the meeting. It exposes
methods to create and vote on polls.

**Kind**: instance property of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+remote"></a>

### meeting.remote

The remote object stores the remote control requests for the meeting. It exposes
methods to request, accept and end the remote control.

**Kind**: instance property of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+connectedMeetings"></a>

### meeting.connectedMeetings

The connectedMeetings object stores the connected meetings states. It exposes
methods to create/read/update/delete methods for connected meetings.

**Kind**: instance property of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+__internals__"></a>

### meeting.\_\_internals\_\_

The **internals** object exposes the internal tools & utilities such as features
and logger so that client can utilise the same to build their own feature based
UI. cocmdLogger (**internals**.logger) can be used to send logs to cocmd's servers
to inform cocmd of issues, if any, proactively.

**Kind**: instance property of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+joinRoom"></a>

### meeting.joinRoom()

The `joinRoom()` method can be used to join the meeting. A `roomJoined` event is
emitted on `self` when the room is joined successfully.

**Kind**: instance method of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient+leaveRoom"></a>

### meeting.leaveRoom()

The `leaveRoom()` method can be used to leave a meeting.

**Kind**: instance method of [<code>cocmdClient</code>](#module_cocmdClient)  
<a name="module_cocmdClient.init"></a>

### meeting.init(options)

The `init` method can be used to instantiate the cocmdClient class. This returns
an instance of cocmdClient, which can be used to perform actions on the meeting.

**Kind**: static method of [<code>cocmdClient</code>](#module_cocmdClient)

| Param             | Description                                                                   |
| ----------------- | ----------------------------------------------------------------------------- |
| options           | The options object.                                                           |
| options.roomName  | The name of the room. (If you are using V2 Api's you don't need to send this) |
| options.authToken | The authorization token received using the API.                               |
| options.apiBase   | The base URL of the API.                                                      |
| options.defaults  | The default audio and video settings.                                         |
