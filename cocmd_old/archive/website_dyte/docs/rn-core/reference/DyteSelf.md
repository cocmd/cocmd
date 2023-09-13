---
sidebar_position: 2
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdSelf"></a>

The cocmdSelf module represents the current user, and allows to modify the state
of the user in the meeting. The audio and video streams of the user can be
retrieved from this module.

- [cocmdSelf](#module_cocmdSelf)
  - [.audioTrack](#module_cocmdSelf+audioTrack)
  - [.rawAudioTrack](#module_cocmdSelf+rawAudioTrack)
  - [.videoTrack](#module_cocmdSelf+videoTrack)
  - [.rawVideoTrack](#module_cocmdSelf+rawVideoTrack)
  - [.screenShareTracks](#module_cocmdSelf+screenShareTracks)
  - [.audioEnabled](#module_cocmdSelf+audioEnabled)
  - [.videoEnabled](#module_cocmdSelf+videoEnabled)
  - [.screenShareEnabled](#module_cocmdSelf+screenShareEnabled)
  - [.mediaPermissions](#module_cocmdSelf+mediaPermissions)
  - [.permissions](#module_cocmdSelf+permissions)
  - ~~[.suggestedTheme](#module_cocmdSelf+suggestedTheme)~~
  - [.config](#module_cocmdSelf+config)
  - [.roomJoined](#module_cocmdSelf+roomJoined)
  - [.isPinned](#module_cocmdSelf+isPinned)
  - [.addAudioMiddleware()](#module_cocmdSelf+addAudioMiddleware)
  - [.removeAudioMiddleware()](#module_cocmdSelf+removeAudioMiddleware)
  - [.addVideoMiddleware()](#module_cocmdSelf+addVideoMiddleware)
  - [.removeVideoMiddleware()](#module_cocmdSelf+removeVideoMiddleware)
  - [.setName(name)](#module_cocmdSelf+setName)
  - [.setupTracks(options)](#module_cocmdSelf+setupTracks)
  - [.enableAudio()](#module_cocmdSelf+enableAudio)
  - [.enableVideo()](#module_cocmdSelf+enableVideo)
  - [.enableScreenShare()](#module_cocmdSelf+enableScreenShare)
  - [.disableAudio()](#module_cocmdSelf+disableAudio)
  - [.disableVideo()](#module_cocmdSelf+disableVideo)
  - [.disableScreenShare()](#module_cocmdSelf+disableScreenShare)
  - [.getCurrentDevices()](#module_cocmdSelf+getCurrentDevices)
  - [.getAudioDevices()](#module_cocmdSelf+getAudioDevices)
  - [.getVideoDevices()](#module_cocmdSelf+getVideoDevices)
  - [.getSpeakerDevices()](#module_cocmdSelf+getSpeakerDevices)
  - [.getDeviceById(deviceId, kind)](#module_cocmdSelf+getDeviceById)
  - [.getAllDevices()](#module_cocmdSelf+getAllDevices)
  - [.setIsPinned()](#module_cocmdSelf+setIsPinned)
  - [.pin()](#module_cocmdSelf+pin)
  - [.unpin()](#module_cocmdSelf+unpin)
  - [.setDevice(device)](#module_cocmdSelf+setDevice)
  - [.requestToJoinStage()](#module_cocmdSelf+requestToJoinStage)
  - [.leaveStage()](#module_cocmdSelf+leaveStage)
  - [.joinStage()](#module_cocmdSelf+joinStage)
  - ~~[.disablePreview()](#module_cocmdSelf+disablePreview)~~

<a name="module_cocmdSelf+audioTrack"></a>

### meeting.self.audioTrack

Returns the `audioTrack`.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+rawAudioTrack"></a>

### meeting.self.rawAudioTrack

Returns the `rawAudioTrack` having no middleware executed on it.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+videoTrack"></a>

### meeting.self.videoTrack

Returns the `videoTrack`.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+rawVideoTrack"></a>

### meeting.self.rawVideoTrack

Returns the `videoTrack` having no middleware executed on it.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+screenShareTracks"></a>

### meeting.self.screenShareTracks

Returns the screen share tracks.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+audioEnabled"></a>

### meeting.self.audioEnabled

Returns true if audio is enabled.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+videoEnabled"></a>

### meeting.self.videoEnabled

Returns true if video is enabled.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+screenShareEnabled"></a>

### meeting.self.screenShareEnabled

Returns true if screen share is enabled.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+mediaPermissions"></a>

### meeting.self.mediaPermissions

Returns the current audio and video permissions given by the user. 'ACCEPTED' if
the user has given permission to use the media. 'DENIED' if the user has denied
permission to use the media. 'SYS_DENIED' if the user's system has denied
permission to use the media. 'UNAVAILABLE' if the media is not available (or
being used by a different application).

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+permissions"></a>

### meeting.self.permissions

Returns the current permission given to the user for the meeting.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+suggestedTheme"></a>

### ~~meeting.self.suggestedTheme~~

**_Deprecated_**

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+config"></a>

### meeting.self.config

Returns configuration for the meeting.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+roomJoined"></a>

### meeting.self.roomJoined

Returns true if the local participant has joined the meeting.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+isPinned"></a>

### meeting.self.isPinned

Returns true if the current user is pinned.

**Kind**: instance property of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+addAudioMiddleware"></a>

### meeting.self.addAudioMiddleware()

Adds the audio middleware to be executed on the raw audio stream. If there are
more than 1 audio middlewares, they will be executed in the sequence they were
added in. If you want the sequence to be altered, please remove all previous
middlewares and re-add.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+removeAudioMiddleware"></a>

### meeting.self.removeAudioMiddleware()

Removes the audio middleware, if it is there.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+addVideoMiddleware"></a>

### meeting.self.addVideoMiddleware()

Adds the video middleware to be executed on the raw video stream. If there are
more than 1 video middlewares, they will be executed in the sequence they were
added in. If you want the sequence to be altered, please remove all previous
middlewares and re-add.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+removeVideoMiddleware"></a>

### meeting.self.removeVideoMiddleware()

Removes the video middleware, if it is there.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+setName"></a>

### meeting.self.setName(name)

The name of the user can be set by calling this method. This will get reflected
to other participants ONLY if this method is called before the room is joined.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)

| Param | Description       |
| ----- | ----------------- |
| name  | Name of the user. |

<a name="module_cocmdSelf+setupTracks"></a>

### meeting.self.setupTracks(options)

Sets up the local media tracks.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)

| Param         | Description                           |
| ------------- | ------------------------------------- |
| options       | The audio and video options.          |
| options.video | If true, the video stream is fetched. |
| options.audio | If true, the audio stream is fetched. |

<a name="module_cocmdSelf+enableAudio"></a>

### meeting.self.enableAudio()

This method is used to unmute the local participant's audio.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+enableVideo"></a>

### meeting.self.enableVideo()

This method is used to start streaming the local participant's video to the
meeting.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+enableScreenShare"></a>

### meeting.self.enableScreenShare()

This method is used to start sharing the local participant's screen to the
meeting.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+disableAudio"></a>

### meeting.self.disableAudio()

This method is used to mute the local participant's audio.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+disableVideo"></a>

### meeting.self.disableVideo()

This participant is used to disable the local participant's video.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+disableScreenShare"></a>

### meeting.self.disableScreenShare()

This method is used to stop sharing the local participant's screen.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+getCurrentDevices"></a>

### meeting.self.getCurrentDevices()

Returns the media devices currently being used.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+getAudioDevices"></a>

### meeting.self.getAudioDevices()

Returns the local participant's audio devices.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+getVideoDevices"></a>

### meeting.self.getVideoDevices()

Returns the local participant's video devices.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+getSpeakerDevices"></a>

### meeting.self.getSpeakerDevices()

Returns the local participant's speaker devices.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+getDeviceById"></a>

### meeting.self.getDeviceById(deviceId, kind)

Returns the local participant's device, indexed by ID and kind.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)

| Param    | Description                                       |
| -------- | ------------------------------------------------- |
| deviceId | The ID of the device.                             |
| kind     | The kind of the device: audio, video, or speaker. |

<a name="module_cocmdSelf+getAllDevices"></a>

### meeting.self.getAllDevices()

Returns all media devices accessible by the local participant.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+setIsPinned"></a>

### meeting.self.setIsPinned()

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+pin"></a>

### meeting.self.pin()

Returns `self.id` if user has permission to pin participants.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+unpin"></a>

### meeting.self.unpin()

Returns `self.id` if user has permission to unpin participants.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+setDevice"></a>

### meeting.self.setDevice(device)

Change the current media device that is being used by the local participant.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)

| Param  | Description                                                                                      |
| ------ | ------------------------------------------------------------------------------------------------ |
| device | The device that is to be used. A device of the same `kind` will be replaced. the primary stream. |

<a name="module_cocmdSelf+requestToJoinStage"></a>

### meeting.self.requestToJoinStage()

Requests the moderator to join stage. ONLY FOR WEBINARS.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+leaveStage"></a>

### meeting.self.leaveStage()

Leave stage. ONLY FOR WEBINARS.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+joinStage"></a>

### meeting.self.joinStage()

Assumes you have been accepted to join stage.

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)  
<a name="module_cocmdSelf+disablePreview"></a>

### ~~meeting.self.disablePreview()~~

**_Deprecated_**

**Kind**: instance method of [<code>cocmdSelf</code>](#module_cocmdSelf)
