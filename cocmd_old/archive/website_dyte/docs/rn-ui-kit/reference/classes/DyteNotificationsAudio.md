---
id: "cocmdNotificationsAudio"
title: "Class: cocmdNotificationsAudio"
custom_edit_url: null
---

Handles notification sounds in a meeting

## Constructors

### constructor

• **new cocmdNotificationsAudio**()

## Properties

### audio

• `Private` **audio**: `any`

___

### playing

• `Private` **playing**: `any`

## Methods

### play

▸ **play**(`sound`, `duration?`): `void`

#### Parameters

| Name | Type |
| :------ | :------ |
| `sound` | ``"joined"`` \| ``"left"`` \| ``"message"`` |
| `duration?` | `number` |

#### Returns

`void`

___

### setDevice

▸ **setDevice**(`id`): `Promise`<`void`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `id` | `string` |

#### Returns

`Promise`<`void`\>
