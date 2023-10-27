# BXCAD file specifications
todo

## Type names used
- bool: Boolean (1-byte, 0 = false, 1 = true, >=2 = UB)
- u8: Unsigned 8-bit (1-byte) integer
- u16: Unsigned 16-bit (2-byte) integer
- u32: Unsigned 32-bit (4-byte) integer
- s8: Signed 8-bit (1-byte) integer
- s16: Signed 16-bit (2-byte) integer
- s32: Signed 32-bit (4-byte) integer
- f32: Single-length (32-bit, 4-byte) floating point number
- RGB color: `u8[3]` with RGB values as `[r, g, b]`

## BCCAD (2013/10/07 revision, Rhythm Heaven Megamix)

**Little endian** file format with a timestamp of **20131007** (7 Oct 2013)

### Main structure:

| Size          | Name                 | Description                                |
| ------------- | -------------------- | ------------------------------------------ |
| u32           | Timestamp            | Date of last format revision               |
| u16           | Texture width        | Width of the image this BCCAD refers to    |
| u16           | Texture height       | Height of the image this BCCAD refers to   |
| u32           | Number of sprites    | Number of "sprites" or frames in the BCCAD |
| Variable size | Sprites              | List of BCCAD sprites (see below)          |
| u32           | Number of animations | Number of animations in the BCCAD          |
| Variable size | Animations           | List of BCCAD animations (see below)       |

### Sprite:

Each sprite is a collection of "sprite parts", small textures taken from the spritesheet

| Size            | Name            | Description            |
| --------------- | --------------- | ---------------------- |
| u32             | Number of parts | Number of sprite parts |
| 62 bytes / part | Sprite parts    | List of sprite parts   |

Each sprite part has the following structure:

| Size          | Name               | Description                                                                        |
| ------------- | ------------------ | ---------------------------------------------------------------------------------- |
| u16           | Texture X          | X position of the image in the texture sheet (top-left corner)                     |
| u16           | Texture Y          | Y position of the image in the texture sheet (top-left corner)                     |
| u16           | Texture width      | Width of the image in the texture sheet                                            |
| u16           | Texture height     | Height of the image in the texture sheet                                           |
| s16           | X position         | X position of the image in the sprite                                              |
| s16           | Y position         | Y position of the image in the sprite                                              |
| f32 / `float` | Scale (X axis)     | Scaling factor for the X axis                                                      |
| f32           | Scale (Y axis)     | Scaling factor for the Y axis                                                      |
| f32           | Rotation           | Rotation of the sprite part (in degrees)                                           |
| bool          | Flip X             | If true, flip part on X axis                                                       |
| bool          | Flip Y             | If true, flip part on Y axis                                                       |
| RGB color     | Multiply color     | Color that will be layered to the sprite with the multiply blending mode           |
| RGB color     | Screen color       | Color that will be layered to the sprite with the screen blending mode             |
| u8            | Opacity            | Global opacity for the sprite part                                                 |
| 12 bytes      | Unknown / reserved | Always 0?                                                                          |
| u8            | Designation ID     | Used by the game to apply certain properties to it                                 |
| 1 byte        | Unknown            | Always 0, if it wasn't unaligned I'd say padding                                   |
| f32 * 4       | 3D depth           | For all four corners, in the order: top-left, bottom-left, top-right, bottom-right |
| 1 byte        | Padding            | Referred to as "terminator" on Bread                                               |

### Animation:

Each animation is a collection of animation frames, which are references to sprites with specific
added properties

| Size             | Name             | Description      |
| ---------------- | ---------------- | ---------------- |
| u32              | Number of frames | Number of frames |
| XX bytes / frame | Animation frames | List of frames   |

Each frame has the following structure:

| Size | Name | Description |
| ---- | ---- | ----------- |
|      |      |             |

## BRCAD (2010/03/12 revision, Rhythm Heaven Fever)

**Big endian** file format with a timestamp of **20100312** (12 Mar 2010)

### Main structure:

