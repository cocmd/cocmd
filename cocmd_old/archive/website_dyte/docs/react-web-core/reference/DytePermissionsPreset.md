---
sidebar_position: 6
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdPermissionsPreset"></a>

The cocmdPermissionsPreset class represents the meeting permissions for the
current participant

- [cocmdPermissionsPreset](#module_cocmdPermissionsPreset)
  - [.acceptWaitingRequests](#module_cocmdPermissionsPreset+acceptWaitingRequests)
  - [.requestProduceVideo](#module_cocmdPermissionsPreset+requestProduceVideo)
  - [.requestProduceAudio](#module_cocmdPermissionsPreset+requestProduceAudio)
  - [.requestProduceScreenshare](#module_cocmdPermissionsPreset+requestProduceScreenshare)
  - [.canAllowParticipantAudio](#module_cocmdPermissionsPreset+canAllowParticipantAudio)
  - [.canAllowParticipantScreensharing](#module_cocmdPermissionsPreset+canAllowParticipantScreensharing)
  - [.canAllowParticipantVideo](#module_cocmdPermissionsPreset+canAllowParticipantVideo)
  - [.kickParticipant](#module_cocmdPermissionsPreset+kickParticipant)
  - [.pinParticipant](#module_cocmdPermissionsPreset+pinParticipant)
  - [.canRecord](#module_cocmdPermissionsPreset+canRecord)
  - ~~[.waitingRoomType](#module_cocmdPermissionsPreset+waitingRoomType)~~
  - [.waitingRoomBehaviour](#module_cocmdPermissionsPreset+waitingRoomBehaviour)
  - [.plugins](#module_cocmdPermissionsPreset+plugins)
  - [.polls](#module_cocmdPermissionsPreset+polls)
  - ~~[.produceVideo](#module_cocmdPermissionsPreset+produceVideo)~~
  - ~~[.requestProduce](#module_cocmdPermissionsPreset+requestProduce)~~
  - [.canProduceVideo](#module_cocmdPermissionsPreset+canProduceVideo)
  - ~~[.produceScreenshare](#module_cocmdPermissionsPreset+produceScreenshare)~~
  - [.canProduceScreenshare](#module_cocmdPermissionsPreset+canProduceScreenshare)
  - ~~[.produceAudio](#module_cocmdPermissionsPreset+produceAudio)~~
  - [.canProduceAudio](#module_cocmdPermissionsPreset+canProduceAudio)
  - [.chatPublic](#module_cocmdPermissionsPreset+chatPublic)
  - [.chatPrivate](#module_cocmdPermissionsPreset+chatPrivate)
  - [.hiddenParticipant](#module_cocmdPermissionsPreset+hiddenParticipant)
  - [.showParticipantList](#module_cocmdPermissionsPreset+showParticipantList)
  - ~~[.canChangeParticipantRole](#module_cocmdPermissionsPreset+canChangeParticipantRole)~~
  - [.canChangeParticipantPermissions](#module_cocmdPermissionsPreset+canChangeParticipantPermissions)
  - ~~[.canChangeTheme](#module_cocmdPermissionsPreset+canChangeTheme)~~
  - ~~[.canPresent](#module_cocmdPermissionsPreset+canPresent)~~
  - ~~[.maxScreenShareCount](#module_cocmdPermissionsPreset+maxScreenShareCount)~~

<a name="module_cocmdPermissionsPreset+acceptWaitingRequests"></a>

### meeting.self.permissions.acceptWaitingRequests

The `acceptWaitingRequests` returns boolean value. If `true`, participant can
accept the request of waiting participant.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+requestProduceVideo"></a>

### meeting.self.permissions.requestProduceVideo

The `requestProduceVideo` returns boolean value. If `true`, participant can send
request to participants about producing video.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+requestProduceAudio"></a>

### meeting.self.permissions.requestProduceAudio

The `requestProduceAudio` returns boolean value. If `true`, participant can send
request to participants about producing audio.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+requestProduceScreenshare"></a>

### meeting.self.permissions.requestProduceScreenshare

The `requestProduceScreenshare` returns boolean value. If `true`, participant
can send request to participants about sharing screen.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canAllowParticipantAudio"></a>

### meeting.self.permissions.canAllowParticipantAudio

The `canAllowParticipantAudio` returns boolean value. If `true`, participant can
enable other participants` audio.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canAllowParticipantScreensharing"></a>

### meeting.self.permissions.canAllowParticipantScreensharing

The `canAllowParticipantScreensharing` returns boolean value. If `true`,
participant can enable other participants` screen share.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canAllowParticipantVideo"></a>

### meeting.self.permissions.canAllowParticipantVideo

The `canAllowParticipantVideo` returns boolean value. If `true`, participant can
enable other participants` video.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+kickParticipant"></a>

### meeting.self.permissions.kickParticipant

The `kickParticipant` returns boolean value. If `true`, participant can remove
other participants from the meeting.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+pinParticipant"></a>

### meeting.self.permissions.pinParticipant

The `pinParticipant` returns boolean value. If `true`, participant can pin a
participant in the meeting.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canRecord"></a>

### meeting.self.permissions.canRecord

The `canRecord` returns boolean value. If `true`, participant can record the
meeting.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+waitingRoomType"></a>

### ~~meeting.self.permissions.waitingRoomType~~

**_Deprecated_**

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+waitingRoomBehaviour"></a>

### meeting.self.permissions.waitingRoomBehaviour

The `waitingRoomType` returns string value. type of waiting room behavior
possible values are `SKIP`, `ON_PRIVILEGED_USER_ENTRY`, `SKIP_ON_ACCEPT`

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+plugins"></a>

### meeting.self.permissions.plugins

The `plugins` tells if the participant can act on plugins there are 2
permissions with boolean values, `canStart` and `canClose`.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+polls"></a>

### meeting.self.permissions.polls

The `polls` tells if the participant can use polls. There are 3 permissions with
boolean values, `canCreate`, `canVote`, `canViewResults`

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+produceVideo"></a>

### ~~meeting.self.permissions.produceVideo~~

**_Deprecated_**

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+requestProduce"></a>

### ~~meeting.self.permissions.requestProduce~~

**_Deprecated_**

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canProduceVideo"></a>

### meeting.self.permissions.canProduceVideo

The `canProduceVideo` shows permissions for enabling video. There possible
values are `ALLOWED`, `NOT_ALLOWED`, `CAN_REQUEST`

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+produceScreenshare"></a>

### ~~meeting.self.permissions.produceScreenshare~~

**_Deprecated_**

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canProduceScreenshare"></a>

### meeting.self.permissions.canProduceScreenshare

The `canProduceScreenshare` shows permissions for sharing screen. There possible
values are `ALLOWED`, `NOT_ALLOWED`, `CAN_REQUEST`

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+produceAudio"></a>

### ~~meeting.self.permissions.produceAudio~~

**_Deprecated_**

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canProduceAudio"></a>

### meeting.self.permissions.canProduceAudio

The `canProduceAudio` shows permissions for enabling audio. There possible
values are `ALLOWED`, `NOT_ALLOWED`, `CAN_REQUEST`

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+chatPublic"></a>

### meeting.self.permissions.chatPublic

The `chatPublic` shows permissions for public chat there are 4 permissions
`canSend` - if true, the participant can send chat `text` - if true, the
participant can send text `files` - if true, the participant can send files

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+chatPrivate"></a>

### meeting.self.permissions.chatPrivate

The `chatPrivate` shows permissions for public chat there are 4 permissions
`canSend` - if true, the participant can send private chat `text` - if true, the
participant can send text as private chat `files` - if true, the participant can
send files as private chat `canReceive` - (optional) if true, the participant
can receive private chat

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+hiddenParticipant"></a>

### meeting.self.permissions.hiddenParticipant

The `hiddenParticipant` returns boolean value. If `true`, participant is hidden.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+showParticipantList"></a>

### meeting.self.permissions.showParticipantList

The `showParticipantList` returns boolean value. If `true`, participant list can
be shown to the participant.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canChangeParticipantRole"></a>

### ~~meeting.self.permissions.canChangeParticipantRole~~

**_Deprecated_**

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canChangeParticipantPermissions"></a>

### meeting.self.permissions.canChangeParticipantPermissions

The `canChangeParticipantPermissions` returns boolean value. If `true`, allow
changing the participants' permissions.

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canChangeTheme"></a>

### ~~meeting.self.permissions.canChangeTheme~~

**_Deprecated_**

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+canPresent"></a>

### ~~meeting.self.permissions.canPresent~~

**_Deprecated_**

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)  
<a name="module_cocmdPermissionsPreset+maxScreenShareCount"></a>

### ~~meeting.self.permissions.maxScreenShareCount~~

**_Deprecated_**

**Kind**: instance property of
[<code>cocmdPermissionsPreset</code>](#module_cocmdPermissionsPreset)
