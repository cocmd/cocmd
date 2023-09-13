---
sidebar_position: 9
web_core_version: 1.2.0
---

<!-- Auto Generated Below -->

<a name="module_cocmdPolls"></a>

The cocmdPolls module consists of the polls that have been created in the
meeting.

- [cocmdPolls](#module_cocmdPolls)
  - [.items](#module_cocmdPolls+items)
  - [.create(question, options, anonymous, hideVotes)](#module_cocmdPolls+create)
  - [.vote(pollId, index)](#module_cocmdPolls+vote)

<a name="module_cocmdPolls+items"></a>

### meeting.polls.items

An array of poll items.

**Kind**: instance property of [<code>cocmdPolls</code>](#module_cocmdPolls)  
<a name="module_cocmdPolls+create"></a>

### meeting.polls.create(question, options, anonymous, hideVotes)

Creates a poll in the meeting.

**Kind**: instance method of [<code>cocmdPolls</code>](#module_cocmdPolls)

| Param     | Default            | Description                                |
| --------- | ------------------ | ------------------------------------------ |
| question  |                    | The question that is to be voted for.      |
| options   |                    | The options of the poll.                   |
| anonymous | <code>false</code> | If true, the poll votes are anonymous.     |
| hideVotes | <code>false</code> | If true, the votes on the poll are hidden. |

<a name="module_cocmdPolls+vote"></a>

### meeting.polls.vote(pollId, index)

Casts a vote on an existing poll.

**Kind**: instance method of [<code>cocmdPolls</code>](#module_cocmdPolls)

| Param  | Description                                |
| ------ | ------------------------------------------ |
| pollId | The ID of the poll that is to be voted on. |
| index  | The index of the option.                   |
