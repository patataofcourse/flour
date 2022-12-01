# BXCAD file specifications

# BCCAD (2013/10/07 revision, Rhythm Heaven Megamix)

**Little endian** file format with a timestamp of **20131007** (7 Oct 2013)

#### Main structure:

| Size              | Name                  | Description                                   |
|-------------------|-----------------------|-----------------------------------------------|
| [u32] (4 bytes)   | Timestamp             | Date of last format revision                  |
| [u16] (2 bytes)   | Texture width         | Width of the image this BCCAD refers to       |
| [u16]             | Texture height        | Height of the image this BCCAD refers to      |
| [u32]             | Number of sprites     | Number of "sprites" or frames in the BCCAD    |
| Variable          | Sprites               | List of BCCAD sprites (see below)             |
| [u32]             | Number of animations  | Number of animations in the BCCAD             |
| Variable          | Animations            | List of BCCAD animations (see below)          |

#### Sprite:

Each sprite is a collection of "sprite parts", small textures taken from the spritesheet

| Size              | Name              | Description               |
|-------------------|-------------------|---------------------------|
| [u32]             | Number of parts   | Number of sprite parts    |
| XX bytes / part   | Sprite parts      | List of sprite parts      |

Each sprite part has the following structure:

| Size | Name | Description |
|-|-|-|
| | | |

#### Animation:

Each animation is a collection of animation frames, which are references to sprites with specific
added properties

| Size              | Name              | Description       |
|-------------------|-------------------|-------------------|
| [u32]             | Number of frames  | Number of frames  |
| XX bytes / frame  | Animation frames  | List of frames    |

Each frame has the following structure:

| Size | Name | Description |
|-|-|-|
| | | |

## BRCAD (2010/03/12 revision, Rhythm Heaven Fever)

**Big endian** file format with a timestamp of **20100312** (12 Mar 2010)

#### Main structure:

