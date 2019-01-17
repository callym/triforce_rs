#[allow(deprecated)]
use diesel::types::{HasSqlType, NotNull};
use diesel::pg::{Pg, PgTypeMetadata, PgMetadataLookup};

#[derive(Clone, Copy)] pub struct TsQuery;
#[derive(Clone, Copy)] pub struct TsVector;

impl HasSqlType<TsQuery> for Pg {
    fn metadata(_: &PgMetadataLookup) -> PgTypeMetadata {
        PgTypeMetadata {
            oid: 3615,
            array_oid: 3645,
        }
    }
}

impl HasSqlType<TsVector> for Pg {
    fn metadata(_: &PgMetadataLookup) -> PgTypeMetadata {
        PgTypeMetadata {
            oid: 3614,
            array_oid: 3643,
        }
    }
}

impl NotNull for TsVector {}
impl NotNull for TsQuery {}
