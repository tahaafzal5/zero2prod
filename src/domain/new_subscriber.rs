use crate::domain::{SubscriberEmail, SubscriberName};
use crate::routes::FormData;

pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;

        Ok(NewSubscriber { name, email })
    }
}
