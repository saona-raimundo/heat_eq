use core::str::FromStr;

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

pub mod kind;
use kind::FnInputKind;

#[derive(Debug, Clone)]
pub struct FnInput {
    pub string: String,
    pub kind: FnInputKind,
}

impl Default for FnInput {
    fn default() -> Self {
        let string = "sin({x})".to_string();
        FnInput {
            string: string,
            kind: FnInputKind::default(),
        }
    }
}

impl FnInput {
    // pub fn kind(&self) -> &FnInputKind {
    //     &self.kind
    // }
    // pub fn set_kind(&mut self, kind: FnInputKind) -> &mut Self {
    //     self.kind = kind;
    //     self
    // }
    // pub fn set_string(&mut self, s: String) -> &mut Self {
    //     self.string = s;
    //     self
    // }
    /// Evaluates the function at a given value.
    pub fn eval(&self, value: f64) -> f64 {
        match &self.kind {
            FnInputKind::Analytical { expression } => expression.eval(&[value]).unwrap(),
            FnInputKind::Points { spline } => spline.clamped_sample(value).unwrap(),
        }
    }
}

impl Serialize for FnInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FnInput", 2)?;
        s.serialize_field("string", &self.string)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for FnInput {
    fn deserialize<D>(deserializer: D) -> Result<FnInput, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Clone, Deserialize)]
        pub struct __FnInput {
            pub string: String,
        }
        let __fn_input = __FnInput::deserialize(deserializer)?;
        let kind = FnInputKind::from_str(&__fn_input.string).unwrap();
        Ok(FnInput {
            string: __fn_input.string,
            kind,
        })
    }
}

impl FromStr for FnInput {
    type Err = kind::FormatError;
    fn from_str(s: &str) -> Result<Self, kind::FormatError> {
        let kind = FnInputKind::from_str(s)?;
        Ok(FnInput {
            string: s.to_string(),
            kind,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() -> anyhow::Result<()> {
        let fn_input = FnInput::default();
        let string: String = ron::ser::to_string(&fn_input)?;
        let other_fn_input: FnInput = ron::de::from_str(&string)?;
        assert_eq!(fn_input.string, other_fn_input.string);
        Ok(())
    }
}
