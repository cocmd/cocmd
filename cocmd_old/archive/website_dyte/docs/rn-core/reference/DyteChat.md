---
sidebar_position: 8
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdChat"></a>

This is the chat module, which can be used to send and receive messages from the
meeting.

- [cocmdChat](#module_cocmdChat)
  - [.messages](#module_cocmdChat+messages)
  - [.pinned](#module_cocmdChat+pinned)
  - [.sendTextMessage(message)](#module_cocmdChat+sendTextMessage)
  - [.sendImageMessage(image)](#module_cocmdChat+sendImageMessage)
  - [.sendFileMessage(file)](#module_cocmdChat+sendFileMessage)
  - [.sendMessage(message)](#module_cocmdChat+sendMessage)
  - [.getMessagesByUser(userId)](#module_cocmdChat+getMessagesByUser)
  - [.getMessagesByType(type)](#module_cocmdChat+getMessagesByType)
  - [.pin(id)](#module_cocmdChat+pin)
  - [.unpin(id)](#module_cocmdChat+unpin)

<a name="module_cocmdChat+messages"></a>

### meeting.chat.messages

An array of chat messages.

**Kind**: instance property of [<code>cocmdChat</code>](#module_cocmdChat)  
<a name="module_cocmdChat+pinned"></a>

### meeting.chat.pinned

Returns an array of pinned messages.

**Kind**: instance property of [<code>cocmdChat</code>](#module_cocmdChat)  
<a name="module_cocmdChat+sendTextMessage"></a>

### meeting.chat.sendTextMessage(message)

Sends a chat text message to the room.

**Kind**: instance method of [<code>cocmdChat</code>](#module_cocmdChat)

| Param   | Description                                |
| ------- | ------------------------------------------ |
| message | The message that must be sent to the room. |

<a name="module_cocmdChat+sendImageMessage"></a>

### meeting.chat.sendImageMessage(image)

Sends an image message to the meeting.

**Kind**: instance method of [<code>cocmdChat</code>](#module_cocmdChat)

| Param | Description                   |
| ----- | ----------------------------- |
| image | The image that is to be sent. |

<a name="module_cocmdChat+sendFileMessage"></a>

### meeting.chat.sendFileMessage(file)

Sends a file to the meeting.

**Kind**: instance method of [<code>cocmdChat</code>](#module_cocmdChat)

| Param | Description    |
| ----- | -------------- |
| file  | A File object. |

<a name="module_cocmdChat+sendMessage"></a>

### meeting.chat.sendMessage(message)

Sends a message to the meeting. This method can be used to send text, image, or
file messages. The message type is determined by the key 'type' in `message`
object.

**Kind**: instance method of [<code>cocmdChat</code>](#module_cocmdChat)

| Param   | Description                                              |
| ------- | -------------------------------------------------------- |
| message | An object including the type and content of the message. |

<a name="module_cocmdChat+getMessagesByUser"></a>

### meeting.chat.getMessagesByUser(userId)

Returns an array of messages sent by a specific userId.

**Kind**: instance method of [<code>cocmdChat</code>](#module_cocmdChat)

| Param  | Description                                    |
| ------ | ---------------------------------------------- |
| userId | The user id of the user that sent the message. |

<a name="module_cocmdChat+getMessagesByType"></a>

### meeting.chat.getMessagesByType(type)

Returns an array of 'text', 'image' or 'file' messages.

**Kind**: instance method of [<code>cocmdChat</code>](#module_cocmdChat)

| Param | Description                 |
| ----- | --------------------------- |
| type  | 'text', 'image', or 'file'. |

<a name="module_cocmdChat+pin"></a>

### meeting.chat.pin(id)

Pins a chat message

**Kind**: instance method of [<code>cocmdChat</code>](#module_cocmdChat)

| Param | Description                    |
| ----- | ------------------------------ |
| id    | ID of the message to be pinned |

<a name="module_cocmdChat+unpin"></a>

### meeting.chat.unpin(id)

Unpins a chat message

**Kind**: instance method of [<code>cocmdChat</code>](#module_cocmdChat)

| Param | Description                      |
| ----- | -------------------------------- |
| id    | ID of the message to be unpinned |
