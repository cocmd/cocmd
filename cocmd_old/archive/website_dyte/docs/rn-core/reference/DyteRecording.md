---
sidebar_position: 15
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdRecording"></a>

The cocmdRecording module represents the state of the current recording, and
allows to start/stop recordings and check if there's a recording in progress.

- [cocmdRecording](#module_cocmdRecording)
  - [.recordingId](#module_cocmdRecording+recordingId)
  - [.start()](#module_cocmdRecording+start)
  - [.stop()](#module_cocmdRecording+stop)
  - [.getRecordingId()](#module_cocmdRecording+getRecordingId)

<a name="module_cocmdRecording+recordingId"></a>

### meeting.recording.recordingId

NOTE(CallMeTarush): Not setting recording state here, waiting for recording peer
to join to set state automatically

**Kind**: instance property of
[<code>cocmdRecording</code>](#module_cocmdRecording)  
<a name="module_cocmdRecording+start"></a>

### meeting.recording.start()

Starts recording the meeting.

**Kind**: instance method of
[<code>cocmdRecording</code>](#module_cocmdRecording)  
<a name="module_cocmdRecording+stop"></a>

### meeting.recording.stop()

Stops recording the meeting assuming there is a recording in progress.

**Kind**: instance method of
[<code>cocmdRecording</code>](#module_cocmdRecording)  
<a name="module_cocmdRecording+getRecordingId"></a>

### meeting.recording.getRecordingId()

Refreshes current recording state.

**Kind**: instance method of [<code>cocmdRecording</code>](#module_cocmdRecording)
