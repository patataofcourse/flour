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

| Type          | Name                 | Description                                |
| ------------- | -------------------- | ------------------------------------------ |
| u32           | Timestamp            | Date of last format revision               |
| u16           | Texture width        | Width of the image this BCCAD refers to    |
| u16           | Texture height       | Height of the image this BCCAD refers to   |
| u32           | Number of sprites    | Number of "sprites" or frames in the BCCAD |
| Variable size | Sprites              | List of BCCAD sprites (see below)          |
| u32           | Number of animations | Number of animations in the BCCAD          |
| Variable size | Animations           | List of BCCAD animations (see below)       |

### Sprite:

Each sprite is a collection of cells or "sprite parts", small textures taken from the spritesheet

| Type            | Name            | Description            |
| --------------- | --------------- | ---------------------- |
| u32             | Number of parts | Number of sprite parts |
| 62 bytes / part | Sprite parts    | List of sprite parts   |

Each sprite part has the following structure:

| Type      | Name               | Description                                                                        |
| --------- | ------------------ | ---------------------------------------------------------------------------------- |
| u16       | Texture X          | X position of the image in the texture sheet (top-left corner)                     |
| u16       | Texture Y          | Y position of the image in the texture sheet (top-left corner)                     |
| u16       | Texture width      | Width of the image in the texture sheet                                            |
| u16       | Texture height     | Height of the image in the texture sheet                                           |
| s16       | X position         | X position of the image in the sprite                                              |
| s16       | Y position         | Y position of the image in the sprite                                              |
| f32       | Scale (X axis)     | Scaling factor for the X axis                                                      |
| f32       | Scale (Y axis)     | Scaling factor for the Y axis                                                      |
| f32       | Rotation           | Rotation of the sprite part (in degrees)                                           |
| bool      | Flip X             | If true, flip part on X axis                                                       |
| bool      | Flip Y             | If true, flip part on Y axis                                                       |
| RGB color | Multiply color     | Color that will be layered to the sprite with the multiply blending mode           |
| RGB color | Screen color       | Color that will be layered to the sprite with the screen blending mode             |
| u8        | Opacity            | Global opacity for the sprite part                                                 |
| 12 bytes  | Unknown / reserved | Always 0?                                                                          |
| u8        | Designation ID     | Used by the game to apply certain properties to it                                 |
| 1 byte    | Unknown            | Always 0. Padding?                                                                 |
| f32 * 4   | 3D depth           | For all four corners, in the order: top-left, bottom-left, top-right, bottom-right |
| 1 byte    | Padding            | Referred to as "terminator" on Bread                                               |

### Animation:

Each animation is a collection of animation frames, which are references to sprites with specific
added properties

| Type             | Name             | Description      |
| ---------------- | ---------------- | ---------------- |
| u32              | Number of frames | Number of frames |
| 40 bytes / frame | Animation frames | List of frames   |

Each frame has the following structure:

| Type      | Name           | Description                                                                        |
| --------- | -------------- | ---------------------------------------------------------------------------------- |
| u16       | Sprite         | Number of the sprite that this animation frame uses                                |
| u16       | Duration       | Amount of time that the frame will last. Unit is variable, sometimes BPM-dependent |
| s16       | X position     | X offset of the frame compared to the sprite                                       |
| s16       | Y position     | Y offset of the frame compared to the sprite                                       |
| f32       | Depth          | Stereoscopic 3D depth of the frame                                                 |
| f32       | X scaling      | Multiplier to the frame's X scale (width)                                          |
| f32       | Y scaling      | Multiplier to the frame's Y scale (height)                                         |
| f32       | Rotation       | Rotation of the frame (in degrees?)                                                |
| RGB color | Multiply color | Color that will be layered to the frame with the multiply blending mode            |
| 3 bytes   | Padding        | Always 0?                                                                          |
| u16       | Opacity        | Opacity of the frame                                                               |

## BRCAD (2010/03/12 revision, Rhythm Heaven Fever)

**Big endian** file format with a timestamp of **20100312** (12 Mar 2010)

Generally a subset of BCCAD since it's an older revision of the format. Excludes some features like the 3DS' stereoscopic 3D.

### Main structure:


| Type          | Name                  | Description                                |
| ------------- | --------------------- | ------------------------------------------ |
| u32           | Timestamp             | Date of last format revision               |
| u32           | Unknown               | ?                                          |
| u16           | Spritesheet number    | Unknown?                                   |
| u16           | Spritesheet "control" | Unknown?                                   |
| u16           | Texture width         | Width of the image this BRCAD refers to    |
| u16           | Texture height        | Height of the image this BRCAD refers to   |
| u16           | Number of sprites     | Number of "sprites" or frames in the BRCAD |
| u16           | Unknown               | ?                                          |
| Variable size | Sprites               | List of BRCAD sprites (see below)          |
| u16           | Number of animations  | Number of animations in the BRCAD          |
| u16           | Unknown               | ?                                          |
| Variable size | Animations            | List of BRCAD animations (see below)       |

### Sprite:

Each sprite is a collection of cells or "sprite parts", small textures taken from the spritesheet

| Type            | Name            | Description            |
| --------------- | --------------- | ---------------------- |
| u16             | Number of parts | Number of sprite parts |
| u16             | Unknown         | ?                      |
| 32 bytes / part | Sprite parts    | List of sprite parts   |

Each sprite part has the following structure:

| Type   | Name           | Description                                                    |
| ------ | -------------- | -------------------------------------------------------------- |
| u16    | Texture X      | X position of the image in the texture sheet (top-left corner) |
| u16    | Texture Y      | Y position of the image in the texture sheet (top-left corner) |
| u32    | Unknown        | ?                                                              |
| u16    | Texture width  | Width of the image in the texture sheet                        |
| u16    | Texture height | Height of the image in the texture sheet                       |
| u16    | X position     | X position of the image in the sprite                          |
| u16    | Y position     | Y position of the image in the sprite                          |
| f32    | Scale (X axis) | Scaling factor for the X axis                                  |
| f32    | Scale (Y axis) | Scaling factor for the Y axis                                  |
| f32    | Rotation       | Rotation of the sprite part (in degrees)                       |
| bool   | Flip X         | If true, flip part on X axis                                   |
| bool   | Flip Y         | If true, flip part on Y axis                                   |
| u8     | Opacity        | Global opacity for the sprite part                             |
| 1 byte | Padding        | Referred to as "terminator" on Bread                           |

### Animation:

Each animation is a collection of animation frames, which are references to sprites with specific
added properties

| Type             | Name             | Description      |
| ---------------- | ---------------- | ---------------- |
| u32              | Number of frames | Number of frames |
| 20 bytes / frame | Animation frames | List of frames   |

Each frame has the following structure:

| Type    | Name      | Description                                                                        |
| ------- | --------- | ---------------------------------------------------------------------------------- |
| u16     | Sprite    | Number of the sprite that this animation frame uses                                |
| u16     | Duration  | Amount of time that the frame will last. Unit is variable, sometimes BPM-dependent |
| f32     | X scaling | Multiplier to the frame's X scale (width)                                          |
| f32     | Y scaling | Multiplier to the frame's Y scale (height)                                         |
| f32     | Rotation  | Rotation of the frame (in degrees?)                                                |
| u8     | Opacity   | Opacity of the frame                                                               |
| 3 bytes | Unknown   | Padding?                                                                                  |