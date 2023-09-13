---
sidebar_position: 11
web_core_version: 1.6.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdStage"></a>

The cocmdStage module represents a class to mange the Stage of the meeting
Stage refers to a virtual area, where participants stream are visible to other participants.
When a participant is off stage, they are not producing media but only consuming media from participants who are on Stage


* [cocmdStage](#module_cocmdStage)
    * [.getAccessRequests()](#module_cocmdStage+getAccessRequests)
    * [.requestAccess()](#module_cocmdStage+requestAccess)
    * [.cancelRequestAccess()](#module_cocmdStage+cancelRequestAccess)
    * [.grantAccess()](#module_cocmdStage+grantAccess)
    * [.denyAccess()](#module_cocmdStage+denyAccess)
    * [.join()](#module_cocmdStage+join)
    * [.leave()](#module_cocmdStage+leave)
    * [.kick()](#module_cocmdStage+kick)

<a name="module_cocmdStage+getAccessRequests"></a>

### meeting.stage.getAccessRequests()
Method to fetch all Stage access requests from viewers

**Kind**: instance method of [<code>cocmdStage</code>](#module_cocmdStage)  
<a name="module_cocmdStage+requestAccess"></a>

### meeting.stage.requestAccess()
Method to send a request to privileged users to join the stage

**Kind**: instance method of [<code>cocmdStage</code>](#module_cocmdStage)  
<a name="module_cocmdStage+cancelRequestAccess"></a>

### meeting.stage.cancelRequestAccess()
Method to cancel a previous Stage join request

**Kind**: instance method of [<code>cocmdStage</code>](#module_cocmdStage)  
<a name="module_cocmdStage+grantAccess"></a>

### meeting.stage.grantAccess()
Method to grant access to Stage.
 This can be in response to a Stage Join request but it can be called on other users as well

`permissions.acceptPresentRequests` privilege required

**Kind**: instance method of [<code>cocmdStage</code>](#module_cocmdStage)  
<a name="module_cocmdStage+denyAccess"></a>

### meeting.stage.denyAccess()
Method to deny access to Stage.
This should be called in response to a Stage Join request

**Kind**: instance method of [<code>cocmdStage</code>](#module_cocmdStage)  
<a name="module_cocmdStage+join"></a>

### meeting.stage.join()
Method to join the stage
Users either need to have the permission in the preset or must be accepted by a priveleged
user to call this method

**Kind**: instance method of [<code>cocmdStage</code>](#module_cocmdStage)  
<a name="module_cocmdStage+leave"></a>

### meeting.stage.leave()
Method to leave the stage
Users must either be on the stage already or be accepted to join the stage
to call this method

**Kind**: instance method of [<code>cocmdStage</code>](#module_cocmdStage)  
<a name="module_cocmdStage+kick"></a>

### meeting.stage.kick()
Method to kick a user off the stage

`permissions.kickParticipant` privilege required

**Kind**: instance method of [<code>cocmdStage</code>](#module_cocmdStage)  
