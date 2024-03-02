use crate::edges::Edges;
use crate::error;

impl TryFrom<bevy::prelude::Image> for Edges {
    type Error = error::Error;
    fn try_from(i: bevy::prelude::Image) -> Result<Edges, error::Error> {
        match i.try_into_dynamic() {
            Ok(di) => match Edges::try_from(di) {
                Ok(i) => Ok(i),
                Err(e) => Err(e),
            },
            Err(e) => Err(error::Error::BevyImageConversion(e.to_string())),
        }
    }
}

impl TryFrom<&bevy::prelude::Image> for Edges {
    type Error = error::Error;
    fn try_from(i: &bevy::prelude::Image) -> Result<Edges, error::Error> {
        match i.clone().try_into_dynamic() {
            Ok(di) => match Edges::try_from(di) {
                Ok(i) => Ok(i),
                Err(e) => Err(e),
            },
            Err(e) => Err(error::Error::BevyImageConversion(e.to_string())),
        }
    }
}
