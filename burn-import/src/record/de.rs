use std::collections::HashMap;

use super::adapter::BurnModuleAdapter;
use super::data::NestedValue;

use serde::{
    de::{self, value::Error, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor},
    forward_to_deserialize_any,
};

const RECORD_ITEM_SUFFIX: &str = "RecordItem";

/// A deserializer for the nested value data structure.
pub struct Deserializer<A: BurnModuleAdapter> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    value: Option<NestedValue>,
    phantom: std::marker::PhantomData<A>,
}

impl<A: BurnModuleAdapter> Deserializer<A> {
    pub fn new(value: NestedValue) -> Self {
        Self {
            value: Some(value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, A: BurnModuleAdapter> serde::Deserializer<'de> for Deserializer<A> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Pass the module the module through the adapter
        let value = if let Some(name) = name.strip_suffix(RECORD_ITEM_SUFFIX) {
            A::adapt(name, self.value.unwrap())
        } else {
            self.value.unwrap()
        };

        match value {
            NestedValue::Map(map) => {
                // Add missing fields into the map with default value
                let mut map = map;
                for field in fields.iter().map(|s| s.to_string()) {
                    map.entry(field).or_insert(NestedValue::Default);
                }

                visitor.visit_map(HashMapAccess::<A>::new(map))
            }

            _ => Err(de::Error::custom(format!(
                "Expected struct but got {:?}",
                value
            ))),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.value.unwrap().get_string().unwrap().to_string())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.value.unwrap().get_i16().unwrap().to_owned())
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.value.unwrap().get_i32().unwrap().to_owned())
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.value.unwrap().get_i64().unwrap().to_owned())
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.value.unwrap().get_u16().unwrap().to_owned())
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.value.unwrap().get_u64().unwrap().to_owned())
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.value.unwrap().get_f32().unwrap().to_owned())
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.value.unwrap().get_f64().unwrap().to_owned())
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.value.unwrap().get_string().unwrap())
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(value) = self.value {
            visitor.visit_some(Deserializer::<A>::new(value))
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(Deserializer::<A>::new(self.value.unwrap()))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(NestedValue::Vec(vec)) = self.value {
            visitor.visit_seq(VecSeqAccess::<A>::new(vec))
        } else {
            Err(de::Error::custom(format!(
                "Expected Vec but got {:?}",
                self.value
            )))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

/// A sequence access for a vector in the nested value data structure.
struct VecSeqAccess<A: BurnModuleAdapter> {
    iter: std::vec::IntoIter<NestedValue>,
    phantom: std::marker::PhantomData<A>,
}

impl<A: BurnModuleAdapter> VecSeqAccess<A> {
    fn new(vec: Vec<NestedValue>) -> Self {
        VecSeqAccess {
            iter: vec.into_iter(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, A> SeqAccess<'de> for VecSeqAccess<A>
where
    NestedValueWrapper<A>: IntoDeserializer<'de>,
    A: BurnModuleAdapter,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(v) => seed
                .deserialize(NestedValueWrapper::<A>::new(v).into_deserializer())
                .map(Some),
            None => Ok(None),
        }
    }
}

/// A map access for a map in the nested value data structure.
struct HashMapAccess<A: BurnModuleAdapter> {
    iter: std::collections::hash_map::IntoIter<String, NestedValue>,
    next_value: Option<NestedValue>,
    phantom: std::marker::PhantomData<A>,
}

impl<A: BurnModuleAdapter> HashMapAccess<A> {
    fn new(map: HashMap<String, NestedValue>) -> Self {
        HashMapAccess {
            iter: map.into_iter(),
            next_value: None,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, A> MapAccess<'de> for HashMapAccess<A>
where
    String: IntoDeserializer<'de>,
    NestedValueWrapper<A>: IntoDeserializer<'de>,
    A: BurnModuleAdapter,
{
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((k, v)) => {
                // Keep the value for the next call to next_value_seed.
                self.next_value = Some(v);
                // Deserialize the key.
                seed.deserialize(k.into_deserializer()).map(Some)
            }
            None => Ok(None),
            // None => seed.deserialize(k.into_deserializer()).map(Some),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.next_value.take() {
            Some(NestedValue::Default) => seed.deserialize(DefaultDeserializer),
            Some(v) => seed.deserialize(NestedValueWrapper::new(v).into_deserializer()),
            None => seed.deserialize(DefaultDeserializer),
        }
    }
}

/// A wrapper for the nested value data structure with a burn module adapter.
struct NestedValueWrapper<A: BurnModuleAdapter> {
    value: NestedValue,
    phantom: std::marker::PhantomData<A>,
}

impl<A: BurnModuleAdapter> NestedValueWrapper<A> {
    fn new(value: NestedValue) -> Self {
        Self {
            value,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<A: BurnModuleAdapter> IntoDeserializer<'_, Error> for NestedValueWrapper<A> {
    type Deserializer = Deserializer<A>;

    fn into_deserializer(self) -> Self::Deserializer {
        Deserializer::<A>::new(self.value)
    }
}

/// A default deserializer that always returns the default value.
struct DefaultDeserializer;

impl<'de> serde::Deserializer<'de> for DefaultDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(Default::default())
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(Default::default())
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(Default::default())
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(Default::default())
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(Default::default())
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(Default::default())
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(Default::default())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(Default::default())
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(Default::default())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(Default::default())
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(Default::default())
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(Default::default())
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(Default::default())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_none()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(DefaultSeqAccess::new())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(Default::default())
    }

    forward_to_deserialize_any! {
        u128 bytes byte_buf unit unit_struct newtype_struct tuple
        tuple_struct map enum identifier ignored_any struct
    }
}

/// A default sequence access that always returns None (empty sequence).
pub struct DefaultSeqAccess;

impl Default for DefaultSeqAccess {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultSeqAccess {
    pub fn new() -> Self {
        DefaultSeqAccess
    }
}

impl<'de> SeqAccess<'de> for DefaultSeqAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        // Since this is a default implementation, we'll just return None.
        Ok(None)
    }

    fn size_hint(&self) -> Option<usize> {
        // Since this is a default implementation, we'll just return None.
        None
    }
}
