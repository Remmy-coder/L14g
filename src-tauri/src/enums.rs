use std::fmt;

use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::ToSql,
    sql_types::Text,
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub enum ClientRole {
    Admin,
    Maintainer,
    Developer,
}

impl fmt::Display for ClientRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientRole::Admin => write!(f, "Admin"),
            ClientRole::Maintainer => write!(f, "Maintainer"),
            ClientRole::Developer => write!(f, "Developer"),
        }
    }
}

impl TryFrom<&str> for ClientRole {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Admin" => Ok(ClientRole::Admin),
            "Maintainer" => Ok(ClientRole::Maintainer),
            "Developer" => Ok(ClientRole::Developer),
            _ => Err(format!("Unknown state: {}", value)),
        }
    }
}

impl FromSql<Text, Sqlite> for ClientRole {
    fn from_sql(
        bytes: <Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;

        Ok(t.as_str().try_into()?)
    }
}

impl ToSql<Text, Sqlite> for ClientRole {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}
