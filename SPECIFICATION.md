# BXCAD file specifications

## Type names used
- bool: Boolean (1-byte, 0 = false, 1 = true, >=2 = UB)
- u8: Unsigned 8-bit (1-byte) integer
- u16: Unsigned 16-bit (2-byte) integer
- u32: Unsigned 32-bit (4-byte) integer
- s16: Signed 16-bit (2-byte) integer
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
| u16       | Region X           | X position of the part's region (top-left corner)                                  |
| u16       | Region Y           | Y position of the part's region (top-left corner)                                  |
| u16       | Region W           | Width of the part's region                                                         |
| u16       | Region H           | Height of the part's region                                                        |
| s16       | Position X         | The part's X position                                                              |
| s16       | Position Y         | The part's Y position                                                              |
| f32       | Scale X            | Scaling factor for the X axis                                                      |
| f32       | Scale Y            | Scaling factor for the Y axis                                                      |
| f32       | Angle              | Rotation angle in degrees                                                          |
| bool      | Flip X             | If true, flip part on X axis from the center                                       |
| bool      | Flip Y             | If true, flip part on Y axis from the center                                       |
| RGB color | Multiply color     | Color that will be layered on top of the sprite using the multiply blending mode   |
| RGB color | Screen color       | Color that will be layered on top of the sprite using the screen blending mode     |
| u8        | Opacity            | The part's opacity                                                                 |
| 12 bytes  | Unknown / reserved | Always 0?                                                                          |
| u8        | Designation ID     | Used by the game to apply certain properties to specific parts                     |
| 1 byte    | Unknown            | Always 0. Padding?                                                                 |
| f32 * 4   | 3D depth           | For all four corners, in the order: top-left, bottom-left, top-right, bottom-right |
| 1 byte    | Padding            | 8-bit padding. Referred to as "terminator" on Bread                                |

### Animation:

Each animation is a collection of animation keys, which are references to sprites with specific added properties

| Type           | Name           | Description    |
| -------------- | -------------- | -------------- |
| u16            | Number of keys | Number of keys |
| u16            | Unknown        | Padding?       |
| 40 bytes / key | Animation keys | List of keys   |

Each key has the following structure:

| Type      | Name           | Description                                                                      |
| --------- | -------------- | -------------------------------------------------------------------------------- |
| u16       | Sprite index   | Index of the sprite shown on this key                                            |
| u16       | Duration       | Amount of frames that this key will be shown for (FPS is not fixed)              |
| s16       | Position X     | Additive X offset for the sprite                                                 |
| s16       | Position Y     | Additive Y offset for the sprite                                                 |
| f32       | 3D depth       | Stereoscopic 3D depth of the sprite                                              |
| f32       | Scale X        | The sprite's X scaling factor                                                    |
| f32       | Scale Y        | The sprite's Y scaling factor                                                    |
| f32       | Angle          | Additive rotation angle for the sprite (in degrees)                              |
| RGB color | Multiply color | Color that will be layered on top of the sprite using the multiply blending mode |
| 3 bytes   | Padding        | Always 0?                                                                        |
| u16       | Opacity        | Opacity of the sprite                                                            |

## BRCAD (2010/03/12 revision, Rhythm Heaven Fever)

**Big endian** file format with a timestamp of **20100312** (12 Mar 2010)

Generally a subset of BCCAD since it's an older revision of the format. Excludes some features like the 3DS' stereoscopic 3D.

### Main structure:


| Type          | Name                  | Description                                                                                                                  |
| ------------- | --------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| u32           | Timestamp             | Date of last format revision                                                                                                 |
| bool          | Use variations        | True if associated texture sheet has variations. If true, textures must be paletted; if false, textures must not be paletted |
| 3 bytes       | (Padding)             | 24-bit padding                                                                                                               |
| u16           | Spritesheet index     | Index into cellanim.tpl for the associated texture atlas                                                                     |
| u16           | Spritesheet "control" | Unknown?                                                                                                                     |
| u16           | Texture width         | Width of the associated texture atlas                                                                                        |
| u16           | Texture height        | Height of the associated texture atlas                                                                                       |
| u16           | Number of sprites     | Number of sprites this BRCAD contains                                                                                        |
| u16           | (Padding)             | 16-bit padding                                                                                                               |
| Variable size | Sprites               | List of sprites (see below)                                                                                                  |
| u16           | Number of animations  | Number of animations this BRCAD contains                                                                                     |
| u16           | (Padding)             | 16-bit padding                                                                                                               |
| Variable size | Animations            | List of animations (see below)                                                                                               |

### Sprite:

Each sprite is a collection of cells or "sprite parts", small textures taken from the spritesheet

| Type            | Name            | Description            |
| --------------- | --------------- | ---------------------- |
| u16             | Number of parts | Number of sprite parts |
| u16             | (Padding)       | 16-bit padding         |
| 32 bytes / part | Sprite parts    | List of sprite parts   |

Each sprite part has the following structure:

| Type | Name             | Description                                                                                                                                              |
| ---- | ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| u16  | Region X         | X position of the part's region (top-left corner)                                                                                                        |
| u16  | Region Y         | Y position of the part's region (top-left corner)                                                                                                        |
| u16  | Region W         | Width of the part's region                                                                                                                               |
| u16  | Region H         | Height of the part's region                                                                                                                              |
| u16  | Variation number | Effectively selects which variation to use. If variations are enabled in this BRCAD, this is added to the BRCAD's texture atlas index for this part only |
| u16  | (Padding)        | 16-bit padding                                                                                                                                           |
| s16  | Position X       | The part's X position                                                                                                                                    |
| s16  | Position Y       | The part's Y position                                                                                                                                    |
| f32  | Scale X          | Scaling factor for the X axis                                                                                                                            |
| f32  | Scale Y          | Scaling factor for the Y axis                                                                                                                            |
| f32  | Angle            | Rotation angle in degrees                                                                                                                                |
| bool | Flip X           | If true, flip part on X axis from the center                                                                                                             |
| bool | Flip Y           | If true, flip part on Y axis from the center                                                                                                             |
| u8   | Opacity          | The part's opacity                                                                                                                                       |
| u8   | (Padding)        | 8-bit padding                                                                                                                                            |

### Animation:

Each animation is a collection of animation keys, which are references to sprites with specific added properties

| Type           | Name           | Description    |
| -------------- | -------------- | -------------- |
| u16            | Number of keys | Number of keys |
| u16            | (Padding)      | 16-bit padding |
| 24 bytes / key | Animation keys | List of keys   |

Each key has the following structure:

| Type    | Name         | Description                                                         |
| ------- | ------------ | ------------------------------------------------------------------- |
| u16     | Sprite index | Index of the sprite shown on this key                               |
| u16     | Duration     | Amount of frames that this key will be shown for (FPS is not fixed) |
| s16     | Position X   | Additive X offset for the sprite                                    |
| s16     | Position Y   | Additive Y offset for the sprite                                    |
| f32     | Scale X      | The sprite's X scaling factor                                       |
| f32     | Scale Y      | The sprite's Y scaling factor                                       |
| f32     | Angle        | Additive rotation angle for the sprite (in degrees)                 |
| u8      | Opacity      | Opacity of the sprite                                               |
| 3 bytes | (Padding)    | 24-bit padding                                                      |
