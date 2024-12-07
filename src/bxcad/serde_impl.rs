/// Implementation for BRCAD AnimationStep pos_x and pos_y serialization in flour 2.1+
pub mod pos_xy {
    use crate::bxcad::brcad::AnimationStep;
    use serde::{
        de::{Error, SeqAccess, Unexpected, Visitor},
        ser::SerializeSeq,
        Deserializer, Serializer,
    };

    struct PosXYVisitor;

    const UNEXPECTED: &str = "a sequence of two 16-bit signed integers or a 32-bit integer";

    impl<'de> Visitor<'de> for PosXYVisitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "{}", UNEXPECTED)
        }

        // 1.0 - 2.0 behavior

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> {
            Ok(v as u32)
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            let Ok(v) = v.try_into() else {
                Err(E::invalid_type(Unexpected::Unsigned(v), &UNEXPECTED))?
            };
            self.visit_u32(v)
        }

        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            let Ok(v) = v.try_into() else {
                Err(E::invalid_type(Unexpected::Signed(v), &UNEXPECTED))?
            };
            self.visit_i32(v)
        }

        // 2.1+ behavior

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut out = 0u32;

            let Some(pos_x) = seq.next_element::<i16>()? else {
                Err(A::Error::invalid_length(0, &"2"))?
            };
            let Some(pos_y) = seq.next_element::<i16>()? else {
                Err(A::Error::invalid_length(1, &"2"))?
            };

            // this way we can catch some errors where there's too many values
            if let Some(size) = seq.size_hint() {
                if size != 2 {
                    Err(A::Error::invalid_length(size + 2, &"2"))?;
                }
            }

            *AnimationStep::get_pos_mut(&mut out, false) = pos_x;
            *AnimationStep::get_pos_mut(&mut out, true) = pos_y;
            Ok(out)
        }
    }

    pub fn serialize<S: Serializer>(value: &u32, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&AnimationStep::get_pos(value, false))?;
        seq.serialize_element(&AnimationStep::get_pos(value, true))?;
        seq.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
        deserializer.deserialize_any(PosXYVisitor)
    }
}

/// Implementation for BRCAD has_texture serialization in flour 2.1+
pub mod use_variation {
    use serde::{
        de::{Error, Unexpected, Visitor},
        Deserializer, Serializer,
    };

    struct UseVariationVisitor;

    const UNEXPECTED: &str = "a boolean or 32-bit integer";

    impl<'de> Visitor<'de> for UseVariationVisitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "{}", UNEXPECTED)
        }

        // 1.0 - 2.0 behavior

        fn visit_u32<E: Error>(self, v: u32) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_i32<E: Error>(self, v: i32) -> Result<Self::Value, E> {
            Ok(v as u32)
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            let Ok(v) = v.try_into() else {
                Err(E::invalid_type(Unexpected::Unsigned(v), &UNEXPECTED))?
            };
            self.visit_u32(v)
        }

        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            let Ok(v) = v.try_into() else {
                Err(E::invalid_type(Unexpected::Signed(v), &UNEXPECTED))?
            };
            self.visit_i32(v)
        }

        // 2.1+ behavior

        fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
            Ok(v as u32)
        }
    }

    pub fn serialize<S: Serializer>(value: &u32, serializer: S) -> Result<S::Ok, S::Error> {
        if cfg!(target_endian = "big") {
            serializer.serialize_bool(*value as u8 != 0)
        } else {
            serializer.serialize_bool((*value >> 24) as u8 != 0)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
        deserializer.deserialize_any(UseVariationVisitor)
    }
}

pub mod variation_num {
    use serde::{
        de::{Error, Visitor},
        Deserializer, Serializer,
    };

    pub struct VariationNumVisitor;

    const UNEXPECTED: &str = "an integer";

    impl<'de> Visitor<'de> for VariationNumVisitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "{}", UNEXPECTED)
        }

        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            self.visit_u64(v as u64)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            let v = v as u32;

            Ok(if v > u16::MAX as u32 { v >> 16 } else { v })
        }
    }

    pub fn serialize<S: Serializer>(value: &u32, serializer: S) -> Result<S::Ok, S::Error> {
        if cfg!(target_endian = "big") {
            serializer.serialize_u16(*value as u16)
        } else {
            serializer.serialize_u16((*value >> 16) as u16)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
        deserializer.deserialize_any(VariationNumVisitor)
    }
}
