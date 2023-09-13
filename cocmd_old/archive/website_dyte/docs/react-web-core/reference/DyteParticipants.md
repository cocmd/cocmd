---
sidebar_position: 3
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdParticipants"></a>

This module represents all the participants in the meeting (except the local
user). It consists of 4 maps:

- `joined`: A map of all participants that have joined the meeting.
- `waitlisted`: A map of all participants that have been added to the waitlist.
- `active`: A map of active participants who should be displayed in the meeting
  grid.
- `pinned`: A map of pinned participants.

* [cocmdParticipants](#module_cocmdParticipants)
  - [module.exports](#exp_module_cocmdParticipants--module.exports) ⏏
    - [new module.exports(roomNodeClient, self)](#new_module_cocmdParticipants--module.exports_new)
    - [.waitlisted](#module_cocmdParticipants--module.exports+waitlisted)
    - [.joined](#module_cocmdParticipants--module.exports+joined)
    - [.active](#module_cocmdParticipants--module.exports+active)
    - [.pinned](#module_cocmdParticipants--module.exports+pinned)
    - [.viewMode](#module_cocmdParticipants--module.exports+viewMode)
    - [.currentPage](#module_cocmdParticipants--module.exports+currentPage)
    - [.lastActiveSpeaker](#module_cocmdParticipants--module.exports+lastActiveSpeaker)
    - [.selectedPeers](#module_cocmdParticipants--module.exports+selectedPeers)
    - [.count](#module_cocmdParticipants--module.exports+count)
    - [.maxActiveParticipantsCount](#module_cocmdParticipants--module.exports+maxActiveParticipantsCount)
    - [.pageCount](#module_cocmdParticipants--module.exports+pageCount)
    - [.acceptWaitingRoomRequest(id)](#module_cocmdParticipants--module.exports+acceptWaitingRoomRequest)
    - [.rejectWaitingRoomRequest(id)](#module_cocmdParticipants--module.exports+rejectWaitingRoomRequest)
    - [.setViewMode(viewMode)](#module_cocmdParticipants--module.exports+setViewMode)
    - [.setPage(page)](#module_cocmdParticipants--module.exports+setPage)
    - [.disableAllAudio(allowUnMute)](#module_cocmdParticipants--module.exports+disableAllAudio)
    - [.disableAllVideo()](#module_cocmdParticipants--module.exports+disableAllVideo)
    - ~~[.disableAudio(participantId)](#module_cocmdParticipants--module.exports+disableAudio)~~
    - ~~[.disableVideo(participantId)](#module_cocmdParticipants--module.exports+disableVideo)~~
    - ~~[.kick(participantId)](#module_cocmdParticipants--module.exports+kick)~~
    - [.kickAll()](#module_cocmdParticipants--module.exports+kickAll)
    - [.broadcastMessage()](#module_cocmdParticipants--module.exports+broadcastMessage)
    - [.acceptAllRequestToJoinStageRequests()](#module_cocmdParticipants--module.exports+acceptAllRequestToJoinStageRequests)

<a name="exp_module_cocmdParticipants--module.exports"></a>

### module.exports ⏏

**Kind**: Exported class  
<a name="new_module_cocmdParticipants--module.exports_new"></a>

#### new module.exports(roomNodeClient, self)

This constructs a new cocmdParticipant object and maintains the maps of
active/joined/waitlisted/pinned/selectedPeers maps.

| Param          | Description      |
| -------------- | ---------------- |
| roomNodeClient | : RoomNodeClient |
| self           | : cocmdSelf       |

<a name="module_cocmdParticipants--module.exports+waitlisted"></a>

#### module.exports.waitlisted

Returns a list of participants waiting to join the meeting.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+joined"></a>

#### module.exports.joined

Returns a list of all participants in the meeting.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+active"></a>

#### module.exports.active

Returns a list of participants whose streams are currently consumed.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+pinned"></a>

#### module.exports.pinned

Returns a list of participants who have been pinned.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+viewMode"></a>

#### module.exports.viewMode

Indicates whether the meeting is in 'ACTIVE_GRID' mode or 'PAGINATED' mode.

In 'ACTIVE_GRID' mode, participants are populated in the participants.active map
dynamically. The participants present in the map will keep changing when other
participants unmute their audio or turn on their videos.

In 'PAGINATED' mode, participants are populated in the participants.active map
just once, and the participants in the map will only change if the page number
is changed by the user using setPage(page).

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+currentPage"></a>

#### module.exports.currentPage

This indicates the current page that has been set by the user in PAGINATED mode.
If the meeting is in ACTIVE_GRID mode, this value will be 0.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+lastActiveSpeaker"></a>

#### module.exports.lastActiveSpeaker

This stores the `participantId` of the last participant who spoke in the
meeting.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+selectedPeers"></a>

#### module.exports.selectedPeers

Keeps a list of all participants who have been present in the selected peers
list.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+count"></a>

#### module.exports.count

Returns the number of participants who are joined in the meeting.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+maxActiveParticipantsCount"></a>

#### module.exports.maxActiveParticipantsCount

Returns the maximum number of participants that can be present in the active
map.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+pageCount"></a>

#### module.exports.pageCount

Returns the number of pages that are available in the meeting in PAGINATED mode.
If the meeting is in ACTIVE_GRID mode, this value will be 0.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+acceptWaitingRoomRequest"></a>

#### module.exports.acceptWaitingRoomRequest(id)

Accepts requests from waitlisted participants if user has appropriate
permissions.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param | Description                                  |
| ----- | -------------------------------------------- |
| id    | participantId of the waitlisted participant. |

<a name="module_cocmdParticipants--module.exports+rejectWaitingRoomRequest"></a>

#### module.exports.rejectWaitingRoomRequest(id)

Rejects requests from waitlisted participants if user has appropriate
permissions.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param | Description                                  |
| ----- | -------------------------------------------- |
| id    | participantId of the waitlisted participant. |

<a name="module_cocmdParticipants--module.exports+setViewMode"></a>

#### module.exports.setViewMode(viewMode)

Sets the view mode of the meeting to either ACTIVE_GRID or PAGINATED.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param    | Description                                          |
| -------- | ---------------------------------------------------- |
| viewMode | The mode in which the active map should be populated |

<a name="module_cocmdParticipants--module.exports+setPage"></a>

#### module.exports.setPage(page)

Populates the active map with participants present in the page number indicated
by the parameter `page` in PAGINATED mode. Does not do anything in ACTIVE_GRID
mode.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param | Description                |
| ----- | -------------------------- |
| page  | The page number to be set. |

<a name="module_cocmdParticipants--module.exports+disableAllAudio"></a>

#### module.exports.disableAllAudio(allowUnMute)

Disables audio for all participants in the meeting.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param       | Description                                        |
| ----------- | -------------------------------------------------- |
| allowUnMute | Allow participants to unmute after they are muted. |

<a name="module_cocmdParticipants--module.exports+disableAllVideo"></a>

#### module.exports.disableAllVideo()

Disables video for all participants in the meeting.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+disableAudio"></a>

#### ~~module.exports.disableAudio(participantId)~~

**_Deprecated_**

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param         | Description                    |
| ------------- | ------------------------------ |
| participantId | ID of participant to be muted. |

<a name="module_cocmdParticipants--module.exports+disableVideo"></a>

#### ~~module.exports.disableVideo(participantId)~~

**_Deprecated_**

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param         | Description                    |
| ------------- | ------------------------------ |
| participantId | ID of participant to be muted. |

<a name="module_cocmdParticipants--module.exports+kick"></a>

#### ~~module.exports.kick(participantId)~~

**_Deprecated_**

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param         | Description                     |
| ------------- | ------------------------------- |
| participantId | ID of participant to be kicked. |

<a name="module_cocmdParticipants--module.exports+kickAll"></a>

#### module.exports.kickAll()

Kicks all participants from the meeting.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+broadcastMessage"></a>

#### module.exports.broadcastMessage()

Broadcasts the message to all participants including `self`.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+acceptAllRequestToJoinStageRequests"></a>

#### module.exports.acceptAllRequestToJoinStageRequests()

Accepts all join stage requests

## **Kind**: instance method of [<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

sidebar_position: ((sidebar_position)) web_core_version: ((web_core_version))

---

<!-- Auto Generated Below -->

<a name="module_cocmdParticipants"></a>

This module represents all the participants in the meeting (except the local
user). It consists of 4 maps:

- `joined`: A map of all participants that have joined the meeting.
- `waitlisted`: A map of all participants that have been added to the waitlist.
- `active`: A map of active participants who should be displayed in the meeting
  grid.
- `pinned`: A map of pinned participants.

* [cocmdParticipants](#module_cocmdParticipants)
  - [module.exports](#exp_module_cocmdParticipants--module.exports) ⏏
    - [new module.exports(roomNodeClient, self)](#new_module_cocmdParticipants--module.exports_new)
    - [.waitlisted](#module_cocmdParticipants--module.exports+waitlisted)
    - [.joined](#module_cocmdParticipants--module.exports+joined)
    - [.active](#module_cocmdParticipants--module.exports+active)
    - [.pinned](#module_cocmdParticipants--module.exports+pinned)
    - [.viewMode](#module_cocmdParticipants--module.exports+viewMode)
    - [.currentPage](#module_cocmdParticipants--module.exports+currentPage)
    - [.lastActiveSpeaker](#module_cocmdParticipants--module.exports+lastActiveSpeaker)
    - [.selectedPeers](#module_cocmdParticipants--module.exports+selectedPeers)
    - [.count](#module_cocmdParticipants--module.exports+count)
    - [.maxActiveParticipantsCount](#module_cocmdParticipants--module.exports+maxActiveParticipantsCount)
    - [.pageCount](#module_cocmdParticipants--module.exports+pageCount)
    - [.acceptWaitingRoomRequest(id)](#module_cocmdParticipants--module.exports+acceptWaitingRoomRequest)
    - [.rejectWaitingRoomRequest(id)](#module_cocmdParticipants--module.exports+rejectWaitingRoomRequest)
    - [.setViewMode(viewMode)](#module_cocmdParticipants--module.exports+setViewMode)
    - [.setPage(page)](#module_cocmdParticipants--module.exports+setPage)
    - [.disableAllAudio(allowUnMute)](#module_cocmdParticipants--module.exports+disableAllAudio)
    - [.disableAllVideo()](#module_cocmdParticipants--module.exports+disableAllVideo)
    - ~~[.disableAudio(participantId)](#module_cocmdParticipants--module.exports+disableAudio)~~
    - ~~[.disableVideo(participantId)](#module_cocmdParticipants--module.exports+disableVideo)~~
    - ~~[.kick(participantId)](#module_cocmdParticipants--module.exports+kick)~~
    - [.kickAll()](#module_cocmdParticipants--module.exports+kickAll)
    - [.broadcastMessage()](#module_cocmdParticipants--module.exports+broadcastMessage)
    - [.acceptAllRequestToJoinStageRequests()](#module_cocmdParticipants--module.exports+acceptAllRequestToJoinStageRequests)

<a name="exp_module_cocmdParticipants--module.exports"></a>

### module.exports ⏏

**Kind**: Exported class  
<a name="new_module_cocmdParticipants--module.exports_new"></a>

#### new module.exports(roomNodeClient, self)

This constructs a new cocmdParticipant object and maintains the maps of
active/joined/waitlisted/pinned/selectedPeers maps.

| Param          | Description      |
| -------------- | ---------------- |
| roomNodeClient | : RoomNodeClient |
| self           | : cocmdSelf       |

<a name="module_cocmdParticipants--module.exports+waitlisted"></a>

#### module.exports.waitlisted

Returns a list of participants waiting to join the meeting.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+joined"></a>

#### module.exports.joined

Returns a list of all participants in the meeting.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+active"></a>

#### module.exports.active

Returns a list of participants whose streams are currently consumed.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+pinned"></a>

#### module.exports.pinned

Returns a list of participants who have been pinned.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+viewMode"></a>

#### module.exports.viewMode

Indicates whether the meeting is in 'ACTIVE_GRID' mode or 'PAGINATED' mode.

In 'ACTIVE_GRID' mode, participants are populated in the participants.active map
dynamically. The participants present in the map will keep changing when other
participants unmute their audio or turn on their videos.

In 'PAGINATED' mode, participants are populated in the participants.active map
just once, and the participants in the map will only change if the page number
is changed by the user using setPage(page).

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+currentPage"></a>

#### module.exports.currentPage

This indicates the current page that has been set by the user in PAGINATED mode.
If the meeting is in ACTIVE_GRID mode, this value will be 0.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+lastActiveSpeaker"></a>

#### module.exports.lastActiveSpeaker

This stores the `participantId` of the last participant who spoke in the
meeting.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+selectedPeers"></a>

#### module.exports.selectedPeers

Keeps a list of all participants who have been present in the selected peers
list.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+count"></a>

#### module.exports.count

Returns the number of participants who are joined in the meeting.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+maxActiveParticipantsCount"></a>

#### module.exports.maxActiveParticipantsCount

Returns the maximum number of participants that can be present in the active
map.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+pageCount"></a>

#### module.exports.pageCount

Returns the number of pages that are available in the meeting in PAGINATED mode.
If the meeting is in ACTIVE_GRID mode, this value will be 0.

**Kind**: instance property of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+acceptWaitingRoomRequest"></a>

#### module.exports.acceptWaitingRoomRequest(id)

Accepts requests from waitlisted participants if user has appropriate
permissions.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param | Description                                  |
| ----- | -------------------------------------------- |
| id    | participantId of the waitlisted participant. |

<a name="module_cocmdParticipants--module.exports+rejectWaitingRoomRequest"></a>

#### module.exports.rejectWaitingRoomRequest(id)

Rejects requests from waitlisted participants if user has appropriate
permissions.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param | Description                                  |
| ----- | -------------------------------------------- |
| id    | participantId of the waitlisted participant. |

<a name="module_cocmdParticipants--module.exports+setViewMode"></a>

#### module.exports.setViewMode(viewMode)

Sets the view mode of the meeting to either ACTIVE_GRID or PAGINATED.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param    | Description                                          |
| -------- | ---------------------------------------------------- |
| viewMode | The mode in which the active map should be populated |

<a name="module_cocmdParticipants--module.exports+setPage"></a>

#### module.exports.setPage(page)

Populates the active map with participants present in the page number indicated
by the parameter `page` in PAGINATED mode. Does not do anything in ACTIVE_GRID
mode.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param | Description                |
| ----- | -------------------------- |
| page  | The page number to be set. |

<a name="module_cocmdParticipants--module.exports+disableAllAudio"></a>

#### module.exports.disableAllAudio(allowUnMute)

Disables audio for all participants in the meeting.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param       | Description                                        |
| ----------- | -------------------------------------------------- |
| allowUnMute | Allow participants to unmute after they are muted. |

<a name="module_cocmdParticipants--module.exports+disableAllVideo"></a>

#### module.exports.disableAllVideo()

Disables video for all participants in the meeting.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+disableAudio"></a>

#### ~~module.exports.disableAudio(participantId)~~

**_Deprecated_**

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param         | Description                    |
| ------------- | ------------------------------ |
| participantId | ID of participant to be muted. |

<a name="module_cocmdParticipants--module.exports+disableVideo"></a>

#### ~~module.exports.disableVideo(participantId)~~

**_Deprecated_**

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param         | Description                    |
| ------------- | ------------------------------ |
| participantId | ID of participant to be muted. |

<a name="module_cocmdParticipants--module.exports+kick"></a>

#### ~~module.exports.kick(participantId)~~

**_Deprecated_**

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)

| Param         | Description                     |
| ------------- | ------------------------------- |
| participantId | ID of participant to be kicked. |

<a name="module_cocmdParticipants--module.exports+kickAll"></a>

#### module.exports.kickAll()

Kicks all participants from the meeting.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+broadcastMessage"></a>

#### module.exports.broadcastMessage()

Broadcasts the message to all participants including `self`.

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)  
<a name="module_cocmdParticipants--module.exports+acceptAllRequestToJoinStageRequests"></a>

#### module.exports.acceptAllRequestToJoinStageRequests()

Accepts all join stage requests

**Kind**: instance method of
[<code>module.exports</code>](#exp_module_cocmdParticipants--module.exports)
