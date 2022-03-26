// Copyright 2019-2022 ZomboDB, LLC and Technology Concepts & Design, Inc.
// <support@tcdi.com>. All rights reserved.  Use of this source code is governed
// by the MIT license that can be found in the LICENSE file.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum PgBuiltInOids {
    BOOLOID = crate::BOOLOID as isize,
    BYTEAOID = crate::BYTEAOID as isize,
    CHAROID = crate::CHAROID as isize,
    NAMEOID = crate::NAMEOID as isize,
    INT8OID = crate::INT8OID as isize,
    INT2OID = crate::INT2OID as isize,
    INT2VECTOROID = crate::INT2VECTOROID as isize,
    INT4OID = crate::INT4OID as isize,
    REGPROCOID = crate::REGPROCOID as isize,
    TEXTOID = crate::TEXTOID as isize,
    OIDOID = crate::OIDOID as isize,
    TIDOID = crate::TIDOID as isize,
    XIDOID = crate::XIDOID as isize,
    CIDOID = crate::CIDOID as isize,
    OIDVECTOROID = crate::OIDVECTOROID as isize,
    JSONOID = crate::JSONOID as isize,
    XMLOID = crate::XMLOID as isize,
    XMLARRAYOID = crate::XMLARRAYOID as isize,
    JSONARRAYOID = crate::JSONARRAYOID as isize,
    PGNODETREEOID = crate::PGNODETREEOID as isize,
    PGNDISTINCTOID = crate::PGNDISTINCTOID as isize,
    PGDEPENDENCIESOID = crate::PGDEPENDENCIESOID as isize,
    PGDDLCOMMANDOID = crate::PGDDLCOMMANDOID as isize,
    SMGROID = crate::SMGROID as isize,
    POINTOID = crate::POINTOID as isize,
    LSEGOID = crate::LSEGOID as isize,
    PATHOID = crate::PATHOID as isize,
    BOXOID = crate::BOXOID as isize,
    POLYGONOID = crate::POLYGONOID as isize,
    LINEOID = crate::LINEOID as isize,
    LINEARRAYOID = crate::LINEARRAYOID as isize,
    FLOAT4OID = crate::FLOAT4OID as isize,
    FLOAT8OID = crate::FLOAT8OID as isize,
    ABSTIMEOID = crate::ABSTIMEOID as isize,
    RELTIMEOID = crate::RELTIMEOID as isize,
    TINTERVALOID = crate::TINTERVALOID as isize,
    UNKNOWNOID = crate::UNKNOWNOID as isize,
    CIRCLEOID = crate::CIRCLEOID as isize,
    CIRCLEARRAYOID = crate::CIRCLEARRAYOID as isize,
    CASHOID = crate::CASHOID as isize,
    MONEYARRAYOID = crate::MONEYARRAYOID as isize,
    MACADDROID = crate::MACADDROID as isize,
    INETOID = crate::INETOID as isize,
    CIDROID = crate::CIDROID as isize,
    MACADDR8OID = crate::MACADDR8OID as isize,
    BOOLARRAYOID = crate::BOOLARRAYOID as isize,
    BYTEAARRAYOID = crate::BYTEAARRAYOID as isize,
    CHARARRAYOID = crate::CHARARRAYOID as isize,
    NAMEARRAYOID = crate::NAMEARRAYOID as isize,
    INT2ARRAYOID = crate::INT2ARRAYOID as isize,
    INT2VECTORARRAYOID = crate::INT2VECTORARRAYOID as isize,
    INT4ARRAYOID = crate::INT4ARRAYOID as isize,
    REGPROCARRAYOID = crate::REGPROCARRAYOID as isize,
    TEXTARRAYOID = crate::TEXTARRAYOID as isize,
    OIDARRAYOID = crate::OIDARRAYOID as isize,
    TIDARRAYOID = crate::TIDARRAYOID as isize,
    XIDARRAYOID = crate::XIDARRAYOID as isize,
    CIDARRAYOID = crate::CIDARRAYOID as isize,
    OIDVECTORARRAYOID = crate::OIDVECTORARRAYOID as isize,
    BPCHARARRAYOID = crate::BPCHARARRAYOID as isize,
    VARCHARARRAYOID = crate::VARCHARARRAYOID as isize,
    INT8ARRAYOID = crate::INT8ARRAYOID as isize,
    POINTARRAYOID = crate::POINTARRAYOID as isize,
    LSEGARRAYOID = crate::LSEGARRAYOID as isize,
    PATHARRAYOID = crate::PATHARRAYOID as isize,
    BOXARRAYOID = crate::BOXARRAYOID as isize,
    FLOAT4ARRAYOID = crate::FLOAT4ARRAYOID as isize,
    FLOAT8ARRAYOID = crate::FLOAT8ARRAYOID as isize,
    ABSTIMEARRAYOID = crate::ABSTIMEARRAYOID as isize,
    RELTIMEARRAYOID = crate::RELTIMEARRAYOID as isize,
    TINTERVALARRAYOID = crate::TINTERVALARRAYOID as isize,
    POLYGONARRAYOID = crate::POLYGONARRAYOID as isize,
    ACLITEMOID = crate::ACLITEMOID as isize,
    ACLITEMARRAYOID = crate::ACLITEMARRAYOID as isize,
    MACADDRARRAYOID = crate::MACADDRARRAYOID as isize,
    MACADDR8ARRAYOID = crate::MACADDR8ARRAYOID as isize,
    INETARRAYOID = crate::INETARRAYOID as isize,
    CIDRARRAYOID = crate::CIDRARRAYOID as isize,
    CSTRINGARRAYOID = crate::CSTRINGARRAYOID as isize,
    BPCHAROID = crate::BPCHAROID as isize,
    VARCHAROID = crate::VARCHAROID as isize,
    DATEOID = crate::DATEOID as isize,
    TIMEOID = crate::TIMEOID as isize,
    TIMESTAMPOID = crate::TIMESTAMPOID as isize,
    TIMESTAMPARRAYOID = crate::TIMESTAMPARRAYOID as isize,
    DATEARRAYOID = crate::DATEARRAYOID as isize,
    TIMEARRAYOID = crate::TIMEARRAYOID as isize,
    TIMESTAMPTZOID = crate::TIMESTAMPTZOID as isize,
    TIMESTAMPTZARRAYOID = crate::TIMESTAMPTZARRAYOID as isize,
    INTERVALOID = crate::INTERVALOID as isize,
    INTERVALARRAYOID = crate::INTERVALARRAYOID as isize,
    NUMERICARRAYOID = crate::NUMERICARRAYOID as isize,
    TIMETZOID = crate::TIMETZOID as isize,
    TIMETZARRAYOID = crate::TIMETZARRAYOID as isize,
    BITOID = crate::BITOID as isize,
    BITARRAYOID = crate::BITARRAYOID as isize,
    VARBITOID = crate::VARBITOID as isize,
    VARBITARRAYOID = crate::VARBITARRAYOID as isize,
    NUMERICOID = crate::NUMERICOID as isize,
    REFCURSOROID = crate::REFCURSOROID as isize,
    REFCURSORARRAYOID = crate::REFCURSORARRAYOID as isize,
    REGPROCEDUREOID = crate::REGPROCEDUREOID as isize,
    REGOPEROID = crate::REGOPEROID as isize,
    REGOPERATOROID = crate::REGOPERATOROID as isize,
    REGCLASSOID = crate::REGCLASSOID as isize,
    REGTYPEOID = crate::REGTYPEOID as isize,
    REGROLEOID = crate::REGROLEOID as isize,
    REGNAMESPACEOID = crate::REGNAMESPACEOID as isize,
    REGPROCEDUREARRAYOID = crate::REGPROCEDUREARRAYOID as isize,
    REGOPERARRAYOID = crate::REGOPERARRAYOID as isize,
    REGOPERATORARRAYOID = crate::REGOPERATORARRAYOID as isize,
    REGCLASSARRAYOID = crate::REGCLASSARRAYOID as isize,
    REGTYPEARRAYOID = crate::REGTYPEARRAYOID as isize,
    REGROLEARRAYOID = crate::REGROLEARRAYOID as isize,
    REGNAMESPACEARRAYOID = crate::REGNAMESPACEARRAYOID as isize,
    UUIDOID = crate::UUIDOID as isize,
    UUIDARRAYOID = crate::UUIDARRAYOID as isize,
    LSNOID = crate::LSNOID as isize,
    PG_LSNARRAYOID = crate::PG_LSNARRAYOID as isize,
    TSVECTOROID = crate::TSVECTOROID as isize,
    GTSVECTOROID = crate::GTSVECTOROID as isize,
    TSQUERYOID = crate::TSQUERYOID as isize,
    REGCONFIGOID = crate::REGCONFIGOID as isize,
    REGDICTIONARYOID = crate::REGDICTIONARYOID as isize,
    TSVECTORARRAYOID = crate::TSVECTORARRAYOID as isize,
    GTSVECTORARRAYOID = crate::GTSVECTORARRAYOID as isize,
    TSQUERYARRAYOID = crate::TSQUERYARRAYOID as isize,
    REGCONFIGARRAYOID = crate::REGCONFIGARRAYOID as isize,
    REGDICTIONARYARRAYOID = crate::REGDICTIONARYARRAYOID as isize,
    JSONBOID = crate::JSONBOID as isize,
    JSONBARRAYOID = crate::JSONBARRAYOID as isize,
    TXID_SNAPSHOTOID = crate::TXID_SNAPSHOTOID as isize,
    TXID_SNAPSHOTARRAYOID = crate::TXID_SNAPSHOTARRAYOID as isize,
    INT4RANGEOID = crate::INT4RANGEOID as isize,
    INT4RANGEARRAYOID = crate::INT4RANGEARRAYOID as isize,
    NUMRANGEOID = crate::NUMRANGEOID as isize,
    NUMRANGEARRAYOID = crate::NUMRANGEARRAYOID as isize,
    TSRANGEOID = crate::TSRANGEOID as isize,
    TSRANGEARRAYOID = crate::TSRANGEARRAYOID as isize,
    TSTZRANGEOID = crate::TSTZRANGEOID as isize,
    TSTZRANGEARRAYOID = crate::TSTZRANGEARRAYOID as isize,
    DATERANGEOID = crate::DATERANGEOID as isize,
    DATERANGEARRAYOID = crate::DATERANGEARRAYOID as isize,
    INT8RANGEOID = crate::INT8RANGEOID as isize,
    INT8RANGEARRAYOID = crate::INT8RANGEARRAYOID as isize,
    RECORDOID = crate::RECORDOID as isize,
    RECORDARRAYOID = crate::RECORDARRAYOID as isize,
    CSTRINGOID = crate::CSTRINGOID as isize,
    ANYOID = crate::ANYOID as isize,
    ANYARRAYOID = crate::ANYARRAYOID as isize,
    VOIDOID = crate::VOIDOID as isize,
    TRIGGEROID = crate::TRIGGEROID as isize,
    EVTTRIGGEROID = crate::EVTTRIGGEROID as isize,
    LANGUAGE_HANDLEROID = crate::LANGUAGE_HANDLEROID as isize,
    INTERNALOID = crate::INTERNALOID as isize,
    OPAQUEOID = crate::OPAQUEOID as isize,
    ANYELEMENTOID = crate::ANYELEMENTOID as isize,
    ANYNONARRAYOID = crate::ANYNONARRAYOID as isize,
    ANYENUMOID = crate::ANYENUMOID as isize,
    FDW_HANDLEROID = crate::FDW_HANDLEROID as isize,
    INDEX_AM_HANDLEROID = crate::INDEX_AM_HANDLEROID as isize,
    TSM_HANDLEROID = crate::TSM_HANDLEROID as isize,
    ANYRANGEOID = crate::ANYRANGEOID as isize,
    AT_REWRITE_ALTER_OID = crate::AT_REWRITE_ALTER_OID as isize,
}
impl PgBuiltInOids {
    pub fn from(oid: crate::Oid) -> Option<PgBuiltInOids> {
        match oid {
            crate::BOOLOID => Some(crate::PgBuiltInOids::BOOLOID),
            crate::BYTEAOID => Some(crate::PgBuiltInOids::BYTEAOID),
            crate::CHAROID => Some(crate::PgBuiltInOids::CHAROID),
            crate::NAMEOID => Some(crate::PgBuiltInOids::NAMEOID),
            crate::INT8OID => Some(crate::PgBuiltInOids::INT8OID),
            crate::INT2OID => Some(crate::PgBuiltInOids::INT2OID),
            crate::INT2VECTOROID => Some(crate::PgBuiltInOids::INT2VECTOROID),
            crate::INT4OID => Some(crate::PgBuiltInOids::INT4OID),
            crate::REGPROCOID => Some(crate::PgBuiltInOids::REGPROCOID),
            crate::TEXTOID => Some(crate::PgBuiltInOids::TEXTOID),
            crate::OIDOID => Some(crate::PgBuiltInOids::OIDOID),
            crate::TIDOID => Some(crate::PgBuiltInOids::TIDOID),
            crate::XIDOID => Some(crate::PgBuiltInOids::XIDOID),
            crate::CIDOID => Some(crate::PgBuiltInOids::CIDOID),
            crate::OIDVECTOROID => Some(crate::PgBuiltInOids::OIDVECTOROID),
            crate::JSONOID => Some(crate::PgBuiltInOids::JSONOID),
            crate::XMLOID => Some(crate::PgBuiltInOids::XMLOID),
            crate::XMLARRAYOID => Some(crate::PgBuiltInOids::XMLARRAYOID),
            crate::JSONARRAYOID => Some(crate::PgBuiltInOids::JSONARRAYOID),
            crate::PGNODETREEOID => Some(crate::PgBuiltInOids::PGNODETREEOID),
            crate::PGNDISTINCTOID => Some(crate::PgBuiltInOids::PGNDISTINCTOID),
            crate::PGDEPENDENCIESOID => Some(crate::PgBuiltInOids::PGDEPENDENCIESOID),
            crate::PGDDLCOMMANDOID => Some(crate::PgBuiltInOids::PGDDLCOMMANDOID),
            crate::SMGROID => Some(crate::PgBuiltInOids::SMGROID),
            crate::POINTOID => Some(crate::PgBuiltInOids::POINTOID),
            crate::LSEGOID => Some(crate::PgBuiltInOids::LSEGOID),
            crate::PATHOID => Some(crate::PgBuiltInOids::PATHOID),
            crate::BOXOID => Some(crate::PgBuiltInOids::BOXOID),
            crate::POLYGONOID => Some(crate::PgBuiltInOids::POLYGONOID),
            crate::LINEOID => Some(crate::PgBuiltInOids::LINEOID),
            crate::LINEARRAYOID => Some(crate::PgBuiltInOids::LINEARRAYOID),
            crate::FLOAT4OID => Some(crate::PgBuiltInOids::FLOAT4OID),
            crate::FLOAT8OID => Some(crate::PgBuiltInOids::FLOAT8OID),
            crate::ABSTIMEOID => Some(crate::PgBuiltInOids::ABSTIMEOID),
            crate::RELTIMEOID => Some(crate::PgBuiltInOids::RELTIMEOID),
            crate::TINTERVALOID => Some(crate::PgBuiltInOids::TINTERVALOID),
            crate::UNKNOWNOID => Some(crate::PgBuiltInOids::UNKNOWNOID),
            crate::CIRCLEOID => Some(crate::PgBuiltInOids::CIRCLEOID),
            crate::CIRCLEARRAYOID => Some(crate::PgBuiltInOids::CIRCLEARRAYOID),
            crate::CASHOID => Some(crate::PgBuiltInOids::CASHOID),
            crate::MONEYARRAYOID => Some(crate::PgBuiltInOids::MONEYARRAYOID),
            crate::MACADDROID => Some(crate::PgBuiltInOids::MACADDROID),
            crate::INETOID => Some(crate::PgBuiltInOids::INETOID),
            crate::CIDROID => Some(crate::PgBuiltInOids::CIDROID),
            crate::MACADDR8OID => Some(crate::PgBuiltInOids::MACADDR8OID),
            crate::BOOLARRAYOID => Some(crate::PgBuiltInOids::BOOLARRAYOID),
            crate::BYTEAARRAYOID => Some(crate::PgBuiltInOids::BYTEAARRAYOID),
            crate::CHARARRAYOID => Some(crate::PgBuiltInOids::CHARARRAYOID),
            crate::NAMEARRAYOID => Some(crate::PgBuiltInOids::NAMEARRAYOID),
            crate::INT2ARRAYOID => Some(crate::PgBuiltInOids::INT2ARRAYOID),
            crate::INT2VECTORARRAYOID => Some(crate::PgBuiltInOids::INT2VECTORARRAYOID),
            crate::INT4ARRAYOID => Some(crate::PgBuiltInOids::INT4ARRAYOID),
            crate::REGPROCARRAYOID => Some(crate::PgBuiltInOids::REGPROCARRAYOID),
            crate::TEXTARRAYOID => Some(crate::PgBuiltInOids::TEXTARRAYOID),
            crate::OIDARRAYOID => Some(crate::PgBuiltInOids::OIDARRAYOID),
            crate::TIDARRAYOID => Some(crate::PgBuiltInOids::TIDARRAYOID),
            crate::XIDARRAYOID => Some(crate::PgBuiltInOids::XIDARRAYOID),
            crate::CIDARRAYOID => Some(crate::PgBuiltInOids::CIDARRAYOID),
            crate::OIDVECTORARRAYOID => Some(crate::PgBuiltInOids::OIDVECTORARRAYOID),
            crate::BPCHARARRAYOID => Some(crate::PgBuiltInOids::BPCHARARRAYOID),
            crate::VARCHARARRAYOID => Some(crate::PgBuiltInOids::VARCHARARRAYOID),
            crate::INT8ARRAYOID => Some(crate::PgBuiltInOids::INT8ARRAYOID),
            crate::POINTARRAYOID => Some(crate::PgBuiltInOids::POINTARRAYOID),
            crate::LSEGARRAYOID => Some(crate::PgBuiltInOids::LSEGARRAYOID),
            crate::PATHARRAYOID => Some(crate::PgBuiltInOids::PATHARRAYOID),
            crate::BOXARRAYOID => Some(crate::PgBuiltInOids::BOXARRAYOID),
            crate::FLOAT4ARRAYOID => Some(crate::PgBuiltInOids::FLOAT4ARRAYOID),
            crate::FLOAT8ARRAYOID => Some(crate::PgBuiltInOids::FLOAT8ARRAYOID),
            crate::ABSTIMEARRAYOID => Some(crate::PgBuiltInOids::ABSTIMEARRAYOID),
            crate::RELTIMEARRAYOID => Some(crate::PgBuiltInOids::RELTIMEARRAYOID),
            crate::TINTERVALARRAYOID => Some(crate::PgBuiltInOids::TINTERVALARRAYOID),
            crate::POLYGONARRAYOID => Some(crate::PgBuiltInOids::POLYGONARRAYOID),
            crate::ACLITEMOID => Some(crate::PgBuiltInOids::ACLITEMOID),
            crate::ACLITEMARRAYOID => Some(crate::PgBuiltInOids::ACLITEMARRAYOID),
            crate::MACADDRARRAYOID => Some(crate::PgBuiltInOids::MACADDRARRAYOID),
            crate::MACADDR8ARRAYOID => Some(crate::PgBuiltInOids::MACADDR8ARRAYOID),
            crate::INETARRAYOID => Some(crate::PgBuiltInOids::INETARRAYOID),
            crate::CIDRARRAYOID => Some(crate::PgBuiltInOids::CIDRARRAYOID),
            crate::CSTRINGARRAYOID => Some(crate::PgBuiltInOids::CSTRINGARRAYOID),
            crate::BPCHAROID => Some(crate::PgBuiltInOids::BPCHAROID),
            crate::VARCHAROID => Some(crate::PgBuiltInOids::VARCHAROID),
            crate::DATEOID => Some(crate::PgBuiltInOids::DATEOID),
            crate::TIMEOID => Some(crate::PgBuiltInOids::TIMEOID),
            crate::TIMESTAMPOID => Some(crate::PgBuiltInOids::TIMESTAMPOID),
            crate::TIMESTAMPARRAYOID => Some(crate::PgBuiltInOids::TIMESTAMPARRAYOID),
            crate::DATEARRAYOID => Some(crate::PgBuiltInOids::DATEARRAYOID),
            crate::TIMEARRAYOID => Some(crate::PgBuiltInOids::TIMEARRAYOID),
            crate::TIMESTAMPTZOID => Some(crate::PgBuiltInOids::TIMESTAMPTZOID),
            crate::TIMESTAMPTZARRAYOID => Some(crate::PgBuiltInOids::TIMESTAMPTZARRAYOID),
            crate::INTERVALOID => Some(crate::PgBuiltInOids::INTERVALOID),
            crate::INTERVALARRAYOID => Some(crate::PgBuiltInOids::INTERVALARRAYOID),
            crate::NUMERICARRAYOID => Some(crate::PgBuiltInOids::NUMERICARRAYOID),
            crate::TIMETZOID => Some(crate::PgBuiltInOids::TIMETZOID),
            crate::TIMETZARRAYOID => Some(crate::PgBuiltInOids::TIMETZARRAYOID),
            crate::BITOID => Some(crate::PgBuiltInOids::BITOID),
            crate::BITARRAYOID => Some(crate::PgBuiltInOids::BITARRAYOID),
            crate::VARBITOID => Some(crate::PgBuiltInOids::VARBITOID),
            crate::VARBITARRAYOID => Some(crate::PgBuiltInOids::VARBITARRAYOID),
            crate::NUMERICOID => Some(crate::PgBuiltInOids::NUMERICOID),
            crate::REFCURSOROID => Some(crate::PgBuiltInOids::REFCURSOROID),
            crate::REFCURSORARRAYOID => Some(crate::PgBuiltInOids::REFCURSORARRAYOID),
            crate::REGPROCEDUREOID => Some(crate::PgBuiltInOids::REGPROCEDUREOID),
            crate::REGOPEROID => Some(crate::PgBuiltInOids::REGOPEROID),
            crate::REGOPERATOROID => Some(crate::PgBuiltInOids::REGOPERATOROID),
            crate::REGCLASSOID => Some(crate::PgBuiltInOids::REGCLASSOID),
            crate::REGTYPEOID => Some(crate::PgBuiltInOids::REGTYPEOID),
            crate::REGROLEOID => Some(crate::PgBuiltInOids::REGROLEOID),
            crate::REGNAMESPACEOID => Some(crate::PgBuiltInOids::REGNAMESPACEOID),
            crate::REGPROCEDUREARRAYOID => Some(crate::PgBuiltInOids::REGPROCEDUREARRAYOID),
            crate::REGOPERARRAYOID => Some(crate::PgBuiltInOids::REGOPERARRAYOID),
            crate::REGOPERATORARRAYOID => Some(crate::PgBuiltInOids::REGOPERATORARRAYOID),
            crate::REGCLASSARRAYOID => Some(crate::PgBuiltInOids::REGCLASSARRAYOID),
            crate::REGTYPEARRAYOID => Some(crate::PgBuiltInOids::REGTYPEARRAYOID),
            crate::REGROLEARRAYOID => Some(crate::PgBuiltInOids::REGROLEARRAYOID),
            crate::REGNAMESPACEARRAYOID => Some(crate::PgBuiltInOids::REGNAMESPACEARRAYOID),
            crate::UUIDOID => Some(crate::PgBuiltInOids::UUIDOID),
            crate::UUIDARRAYOID => Some(crate::PgBuiltInOids::UUIDARRAYOID),
            crate::LSNOID => Some(crate::PgBuiltInOids::LSNOID),
            crate::PG_LSNARRAYOID => Some(crate::PgBuiltInOids::PG_LSNARRAYOID),
            crate::TSVECTOROID => Some(crate::PgBuiltInOids::TSVECTOROID),
            crate::GTSVECTOROID => Some(crate::PgBuiltInOids::GTSVECTOROID),
            crate::TSQUERYOID => Some(crate::PgBuiltInOids::TSQUERYOID),
            crate::REGCONFIGOID => Some(crate::PgBuiltInOids::REGCONFIGOID),
            crate::REGDICTIONARYOID => Some(crate::PgBuiltInOids::REGDICTIONARYOID),
            crate::TSVECTORARRAYOID => Some(crate::PgBuiltInOids::TSVECTORARRAYOID),
            crate::GTSVECTORARRAYOID => Some(crate::PgBuiltInOids::GTSVECTORARRAYOID),
            crate::TSQUERYARRAYOID => Some(crate::PgBuiltInOids::TSQUERYARRAYOID),
            crate::REGCONFIGARRAYOID => Some(crate::PgBuiltInOids::REGCONFIGARRAYOID),
            crate::REGDICTIONARYARRAYOID => Some(crate::PgBuiltInOids::REGDICTIONARYARRAYOID),
            crate::JSONBOID => Some(crate::PgBuiltInOids::JSONBOID),
            crate::JSONBARRAYOID => Some(crate::PgBuiltInOids::JSONBARRAYOID),
            crate::TXID_SNAPSHOTOID => Some(crate::PgBuiltInOids::TXID_SNAPSHOTOID),
            crate::TXID_SNAPSHOTARRAYOID => Some(crate::PgBuiltInOids::TXID_SNAPSHOTARRAYOID),
            crate::INT4RANGEOID => Some(crate::PgBuiltInOids::INT4RANGEOID),
            crate::INT4RANGEARRAYOID => Some(crate::PgBuiltInOids::INT4RANGEARRAYOID),
            crate::NUMRANGEOID => Some(crate::PgBuiltInOids::NUMRANGEOID),
            crate::NUMRANGEARRAYOID => Some(crate::PgBuiltInOids::NUMRANGEARRAYOID),
            crate::TSRANGEOID => Some(crate::PgBuiltInOids::TSRANGEOID),
            crate::TSRANGEARRAYOID => Some(crate::PgBuiltInOids::TSRANGEARRAYOID),
            crate::TSTZRANGEOID => Some(crate::PgBuiltInOids::TSTZRANGEOID),
            crate::TSTZRANGEARRAYOID => Some(crate::PgBuiltInOids::TSTZRANGEARRAYOID),
            crate::DATERANGEOID => Some(crate::PgBuiltInOids::DATERANGEOID),
            crate::DATERANGEARRAYOID => Some(crate::PgBuiltInOids::DATERANGEARRAYOID),
            crate::INT8RANGEOID => Some(crate::PgBuiltInOids::INT8RANGEOID),
            crate::INT8RANGEARRAYOID => Some(crate::PgBuiltInOids::INT8RANGEARRAYOID),
            crate::RECORDOID => Some(crate::PgBuiltInOids::RECORDOID),
            crate::RECORDARRAYOID => Some(crate::PgBuiltInOids::RECORDARRAYOID),
            crate::CSTRINGOID => Some(crate::PgBuiltInOids::CSTRINGOID),
            crate::ANYOID => Some(crate::PgBuiltInOids::ANYOID),
            crate::ANYARRAYOID => Some(crate::PgBuiltInOids::ANYARRAYOID),
            crate::VOIDOID => Some(crate::PgBuiltInOids::VOIDOID),
            crate::TRIGGEROID => Some(crate::PgBuiltInOids::TRIGGEROID),
            crate::EVTTRIGGEROID => Some(crate::PgBuiltInOids::EVTTRIGGEROID),
            crate::LANGUAGE_HANDLEROID => Some(crate::PgBuiltInOids::LANGUAGE_HANDLEROID),
            crate::INTERNALOID => Some(crate::PgBuiltInOids::INTERNALOID),
            crate::OPAQUEOID => Some(crate::PgBuiltInOids::OPAQUEOID),
            crate::ANYELEMENTOID => Some(crate::PgBuiltInOids::ANYELEMENTOID),
            crate::ANYNONARRAYOID => Some(crate::PgBuiltInOids::ANYNONARRAYOID),
            crate::ANYENUMOID => Some(crate::PgBuiltInOids::ANYENUMOID),
            crate::FDW_HANDLEROID => Some(crate::PgBuiltInOids::FDW_HANDLEROID),
            crate::INDEX_AM_HANDLEROID => Some(crate::PgBuiltInOids::INDEX_AM_HANDLEROID),
            crate::TSM_HANDLEROID => Some(crate::PgBuiltInOids::TSM_HANDLEROID),
            crate::ANYRANGEOID => Some(crate::PgBuiltInOids::ANYRANGEOID),
            crate::AT_REWRITE_ALTER_OID => Some(crate::PgBuiltInOids::AT_REWRITE_ALTER_OID),
            _ => None,
        }
    }
}
