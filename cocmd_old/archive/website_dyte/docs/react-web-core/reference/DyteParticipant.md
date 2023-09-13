---
sidebar_position: 5
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdParticipant"></a>

This module represents a single participant in the meeting. The participant
object can be accessed from one of the participant lists present in the
`meeting.participants` object. For example,

```ts
const participant1 = meeting.participants.active.get(participantId);
const participant2 = meeting.participants.joined.get(participantId);
const participant3 = meeting.participants.active.toArray()[0];
const participant4 = meeting.participants.active
  .toArray()
  .filter((p) => p.name === 'John');
```

- [cocmdParticipant](#module_cocmdParticipant)
  - [.id](#module_cocmdParticipant+id)
  - [.userId](#module_cocmdParticipant+userId)
  - [.name](#module_cocmdParticipant+name)
  - [.picture](#module_cocmdParticipant+picture)
  - [.clientSpecificId](#module_cocmdParticipant+clientSpecificId)
  - [.device](#module_cocmdParticipant+device)
  - [.videoTrack](#module_cocmdParticipant+videoTrack)
  - [.audioTrack](#module_cocmdParticipant+audioTrack)
  - [.screenShareTracks](#module_cocmdParticipant+screenShareTracks)
  - [.videoEnabled](#module_cocmdParticipant+videoEnabled)
  - [.audioEnabled](#module_cocmdParticipant+audioEnabled)
  - [.screenShareEnabled](#module_cocmdParticipant+screenShareEnabled)
  - [.producers](#module_cocmdParticipant+producers)
  - [.supportsRemoteControl](#module_cocmdParticipant+supportsRemoteControl)
  - [.webinarStageStatus](#module_cocmdParticipant+webinarStageStatus)
  - [.presetName](#module_cocmdParticipant+presetName)
  - [.isPinned](#module_cocmdParticipant+isPinned)
  - [.pin()](#module_cocmdParticipant+pin)
  - [.unpin()](#module_cocmdParticipant+unpin)
  - [.setIsPinned()](#module_cocmdParticipant+setIsPinned)
  - [.disableAudio()](#module_cocmdParticipant+disableAudio)
  - [.kick()](#module_cocmdParticipant+kick)
  - [.disableVideo()](#module_cocmdParticipant+disableVideo)

<a name="module_cocmdParticipant+id"></a>

### participant.id

The peer ID of the participant. The participants are indexed by this ID in the
participant map.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+userId"></a>

### participant.userId

The user ID of the participant.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+name"></a>

### participant.name

The name of the participant.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+picture"></a>

### participant.picture

The picture of the participant.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+clientSpecificId"></a>

### participant.clientSpecificId

The clientSpecificId of the participant.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+device"></a>

### participant.device

The device configuration of the participant.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+videoTrack"></a>

### participant.videoTrack

The participant's video track.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+audioTrack"></a>

### participant.audioTrack

The participant's audio track.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+screenShareTracks"></a>

### participant.screenShareTracks

The participant's screenshare video and audio track.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+videoEnabled"></a>

### participant.videoEnabled

This is true if the participant's video is enabled.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+audioEnabled"></a>

### participant.audioEnabled

This is true if the participant's audio is enabled.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+screenShareEnabled"></a>

### participant.screenShareEnabled

This is true if the participant is screensharing.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+producers"></a>

### participant.producers

producers created by participant

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+supportsRemoteControl"></a>

### participant.supportsRemoteControl

This is true if the dyte participant supports remote control.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+webinarStageStatus"></a>

### participant.webinarStageStatus

Represents the participants current webinar status

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+presetName"></a>

### participant.presetName

The preset of the participant.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+isPinned"></a>

### participant.isPinned

Returns true if the participant is pinned.

**Kind**: instance property of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+pin"></a>

### participant.pin()

Returns `participant.id` if user has permission to pin participants.

**Kind**: instance method of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+unpin"></a>

### participant.unpin()

Returns `participant.id` if user has permission to unpin participants.

**Kind**: instance method of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+setIsPinned"></a>

### participant.setIsPinned()

**Kind**: instance method of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+disableAudio"></a>

### participant.disableAudio()

Disables audio for this participant. Requires the permission to disable
participant audio.

**Kind**: instance method of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+kick"></a>

### participant.kick()

Kicks this participant from the meeting. Requires the permission to kick a
participant.

**Kind**: instance method of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)  
<a name="module_cocmdParticipant+disableVideo"></a>

### participant.disableVideo()

Disables video for this participant. Requires the permission to disable video
for a participant.

**Kind**: instance method of
[<code>cocmdParticipant</code>](#module_cocmdParticipant)
