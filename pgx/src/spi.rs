/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Safe access to Postgres' *Server Programming Interface* (SPI).

use crate::{pg_sys, FromDatum, IntoDatum, Json, PgMemoryContexts, PgOid, TryFromDatumError};
use core::fmt::Formatter;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Index, IndexMut};

/// These match the Postgres `#define`d constants prefixed `SPI_OK_*` that you can find in `pg_sys`.
#[derive(Debug, PartialEq)]
#[repr(i32)]
#[non_exhaustive]
pub enum SpiOk {
    Connect = 1,
    Finish = 2,
    Fetch = 3,
    Utility = 4,
    Select = 5,
    SelInto = 6,
    Insert = 7,
    Delete = 8,
    Update = 9,
    Cursor = 10,
    InsertReturning = 11,
    DeleteReturning = 12,
    UpdateReturning = 13,
    Rewritten = 14,
    RelRegister = 15,
    RelUnregister = 16,
    TdRegister = 17,
    /// Added in Postgres 15
    Merge = 18,
}

/// These match the Postgres `#define`d constants prefixed `SPI_ERROR_*` that you can find in `pg_sys`.
/// It is hypothetically possible for a Postgres-defined status code to be `0`, AKA `NULL`, however,
/// this should not usually occur in Rust code paths. If it does happen, please report such bugs to the pgx repo.
#[derive(thiserror::Error, Debug, PartialEq)]
#[repr(i32)]
pub enum SpiError {
    Connect = -1,
    Copy = -2,
    OpUnknown = -3,
    Unconnected = -4,
    #[allow(dead_code)]
    Cursor = -5, /* not used anymore */
    Argument = -6,
    Param = -7,
    Transaction = -8,
    NoAttribute = -9,
    NoOutFunc = -10,
    TypUnknown = -11,
    RelDuplicate = -12,
    RelNotFound = -13,
}

impl std::fmt::Display for SpiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Debug)]
pub struct UnknownVariant;

impl TryFrom<libc::c_int> for SpiOk {
    // Yes, this gives us nested results.
    type Error = Result<SpiError, UnknownVariant>;

    fn try_from(code: libc::c_int) -> Result<SpiOk, Result<SpiError, UnknownVariant>> {
        // Cast to assure that we're obeying repr rules even on platforms where c_ints are not 4 bytes wide,
        // as we don't support any but we may wish to in the future.
        match code as i32 {
            err @ -13..=-1 => Err(Ok(
                // SAFETY: These values are described in SpiError, thus they are inbounds for transmute
                unsafe { mem::transmute::<i32, SpiError>(err) },
            )),
            ok @ 1..=18 => Ok(
                //SAFETY: These values are described in SpiOk, thus they are inbounds for transmute
                unsafe { mem::transmute::<i32, SpiOk>(ok) },
            ),
            _unknown => Err(Err(UnknownVariant)),
        }
    }
}

impl TryFrom<libc::c_int> for SpiError {
    // Yes, this gives us nested results.
    type Error = Result<SpiOk, UnknownVariant>;

    fn try_from(code: libc::c_int) -> Result<SpiError, Result<SpiOk, UnknownVariant>> {
        match SpiOk::try_from(code) {
            Ok(ok) => Err(Ok(ok)),
            Err(Ok(err)) => Ok(err),
            Err(Err(unknown)) => Err(Err(unknown)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error<E: Debug = ()> {
    #[error("SPI error: {0:?}")]
    SpiError(#[from] SpiError),
    #[error("Datum error: {0}")]
    DatumError(#[from] TryFromDatumError),
    #[error("SpiTupleTable positioned before start")]
    InvalidPosition,
    #[error("TupDesc is NULL")]
    TupDescIsNull,
    #[error(transparent)]
    Other(E),
}

pub struct Spi;

// TODO: should `'conn` be invariant?
pub struct SpiClient<'conn>(PhantomData<&'conn SpiConnection>);

/// a struct to manage our SPI connection lifetime
struct SpiConnection(PhantomData<*mut ()>);

impl SpiConnection {
    /// Connect to Postgres' SPI system
    fn connect() -> Self {
        // connect to SPI
        Spi::check_status(unsafe { pg_sys::SPI_connect() });
        SpiConnection(PhantomData)
    }
}

impl Drop for SpiConnection {
    /// when SpiConnection is dropped, we make sure to disconnect from SPI
    fn drop(&mut self) {
        // disconnect from SPI
        Spi::check_status(unsafe { pg_sys::SPI_finish() });
    }
}

impl SpiConnection {
    /// Return a client that with a lifetime scoped to this connection.
    fn client(&self) -> SpiClient<'_> {
        SpiClient(PhantomData)
    }
}

#[derive(Debug)]
pub struct SpiTupleTable {
    #[allow(dead_code)]
    status_code: SpiOk,
    table: *mut pg_sys::SPITupleTable,
    size: usize,
    tupdesc: Option<pg_sys::TupleDesc>,
    current: isize,
}

/// Represents a single `pg_sys::Datum` inside a `SpiHeapTupleData`
pub struct SpiHeapTupleDataEntry {
    datum: Option<pg_sys::Datum>,
    type_oid: pg_sys::Oid,
}

/// Represents the set of `pg_sys::Datum`s in a `pg_sys::HeapTuple`
pub struct SpiHeapTupleData {
    tupdesc: pg_sys::TupleDesc,
    entries: HashMap<usize, SpiHeapTupleDataEntry>,
}

impl Spi {
    pub fn get_one<A: FromDatum + IntoDatum + 'static>(query: &str) -> Result<Option<A>, Error> {
        Spi::connect(|client| client.select(query, Some(1), None).first().get_one())
    }

    pub fn get_two<A: FromDatum + IntoDatum + 'static, B: FromDatum + IntoDatum + 'static>(
        query: &str,
    ) -> Result<(Option<A>, Option<B>), Error> {
        Spi::connect(|client| client.select(query, Some(1), None).first().get_two::<A, B>())
    }

    pub fn get_three<
        A: FromDatum + IntoDatum + 'static,
        B: FromDatum + IntoDatum + 'static,
        C: FromDatum + IntoDatum + 'static,
    >(
        query: &str,
    ) -> Result<(Option<A>, Option<B>, Option<C>), Error> {
        Spi::connect(|client| client.select(query, Some(1), None).first().get_three::<A, B, C>())
    }

    pub fn get_one_with_args<A: FromDatum + IntoDatum + 'static>(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> Result<Option<A>, Error> {
        Spi::connect(|client| client.select(query, Some(1), Some(args)).first().get_one())
    }

    pub fn get_two_with_args<
        A: FromDatum + IntoDatum + 'static,
        B: FromDatum + IntoDatum + 'static,
    >(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> Result<(Option<A>, Option<B>), Error> {
        Spi::connect(|client| client.select(query, Some(1), Some(args)).first().get_two::<A, B>())
    }

    pub fn get_three_with_args<
        A: FromDatum + IntoDatum + 'static,
        B: FromDatum + IntoDatum + 'static,
        C: FromDatum + IntoDatum + 'static,
    >(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> Result<(Option<A>, Option<B>, Option<C>), Error> {
        Spi::connect(|client| {
            client.select(query, Some(1), Some(args)).first().get_three::<A, B, C>()
        })
    }

    /// just run an arbitrary SQL statement.
    ///
    /// ## Safety
    ///
    /// The statement runs in read/write mode
    pub fn run(query: &str) {
        Spi::run_with_args(query, None)
    }

    /// run an arbitrary SQL statement with args.
    ///
    /// ## Safety
    ///
    /// The statement runs in read/write mode
    pub fn run_with_args(query: &str, args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>) {
        Spi::execute(|client| {
            client.update(query, None, args);
        })
    }

    /// explain a query, returning its result in json form
    pub fn explain(query: &str) -> Json {
        Spi::explain_with_args(query, None)
    }

    /// explain a query with args, returning its result in json form
    pub fn explain_with_args(
        query: &str,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> Json {
        Spi::connect(|client| {
            let table =
                client.update(&format!("EXPLAIN (format json) {}", query), None, args).first();
            table.get_one::<Json>()
        })
        .unwrap()
        .unwrap()
    }

    /// execute SPI commands via the provided `SpiClient`
    pub fn execute<F: FnOnce(SpiClient) + std::panic::UnwindSafe>(f: F) {
        Spi::connect(|client| {
            f(client);
            Ok::<_, Error>(())
        })
        .unwrap();
    }

    /// execute SPI commands via the provided `SpiClient` and return a value from SPI which is
    /// automatically copied into the `CurrentMemoryContext` at the time of this function call
    ///
    /// Note that `SpiClient` is scoped to the connection lifetime and the following code will
    /// not compile:
    ///
    /// ```rust,compile_fail
    /// use pgx::*;
    /// Spi::connect(|client| Ok(Some(client)));
    /// ```
    pub fn connect<R, E: Into<Error<O>>, O: Debug, F: FnOnce(SpiClient<'_>) -> Result<R, E>>(
        f: F,
    ) -> Result<R, Error<O>>
    where
        Error<O>: From<E>,
    {
        // connect to SPI
        let connection = SpiConnection::connect();

        // run the provided closure within the memory context that SPI_connect()
        // just put us un.  We'll disconnect from SPI when the closure is finished.
        // If there's a panic or elog(ERROR), we don't care about also disconnecting from
        // SPI b/c Postgres will do that for us automatically
        Ok(f(connection.client())?)
    }

    pub fn check_status(status_code: i32) -> SpiOk {
        match SpiOk::try_from(status_code) {
            Ok(ok) => ok,
            Err(Err(UnknownVariant)) => panic!("unrecognized SPI status code: {status_code}"),
            Err(Ok(code)) => panic!("{code:?}"),
        }
    }
}

impl<'a> SpiClient<'a> {
    /// perform a SELECT statement
    pub fn select(
        &self,
        query: &str,
        limit: Option<i64>,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> SpiTupleTable {
        // Postgres docs say:
        //
        //    It is generally unwise to mix read-only and read-write commands within a single function
        //    using SPI; that could result in very confusing behavior, since the read-only queries
        //    would not see the results of any database updates done by the read-write queries.
        //
        // As such, we don't actually set read-only to true here

        // TODO:  can we detect if the command counter (or something?) has incremented and if yes
        //        then we set read_only=false, else we can set it to true?
        //        Is this even a good idea?
        self.execute(query, false, limit, args)
    }

    /// perform any query (including utility statements) that modify the database in some way
    pub fn update(
        &self,
        query: &str,
        limit: Option<i64>,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> SpiTupleTable {
        self.execute(query, false, limit, args)
    }

    fn execute(
        &self,
        query: &str,
        read_only: bool,
        limit: Option<i64>,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> SpiTupleTable {
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }

        let src = std::ffi::CString::new(query).expect("query contained a null byte");
        let status_code = match args {
            Some(args) => {
                let nargs = args.len();
                let mut argtypes = vec![];
                let mut datums = vec![];
                let mut nulls = vec![];

                for (argtype, datum) in args {
                    argtypes.push(argtype.value());

                    match datum {
                        Some(datum) => {
                            // ' ' here means that the datum is not null
                            datums.push(datum);
                            nulls.push(' ' as std::os::raw::c_char);
                        }

                        None => {
                            // 'n' here means that the datum is null
                            datums.push(pg_sys::Datum::from(0usize));
                            nulls.push('n' as std::os::raw::c_char);
                        }
                    }
                }

                unsafe {
                    pg_sys::SPI_execute_with_args(
                        src.as_ptr(),
                        nargs as i32,
                        argtypes.as_mut_ptr(),
                        datums.as_mut_ptr(),
                        nulls.as_ptr(),
                        read_only,
                        limit.unwrap_or(0),
                    )
                }
            }
            None => unsafe { pg_sys::SPI_execute(src.as_ptr(), read_only, limit.unwrap_or(0)) },
        };

        SpiTupleTable {
            status_code: Spi::check_status(status_code),
            table: unsafe { pg_sys::SPI_tuptable },
            size: unsafe { pg_sys::SPI_processed as usize },
            tupdesc: if unsafe { pg_sys::SPI_tuptable }.is_null() {
                None
            } else {
                Some(unsafe { (*pg_sys::SPI_tuptable).tupdesc })
            },
            current: -1,
        }
    }
}

impl SpiTupleTable {
    /// `SpiTupleTable`s are positioned before the start, for iteration purposes.
    ///
    /// This method moves the position to the first row.  If there are no rows, this
    /// method will silently return.
    pub fn first(mut self) -> Self {
        self.current = 0;
        self
    }

    /// How many rows were processed?
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_one<A: FromDatum + IntoDatum + 'static>(&self) -> Result<Option<A>, Error> {
        self.get_datum(1)
    }

    pub fn get_two<A: FromDatum + IntoDatum + 'static, B: FromDatum + IntoDatum + 'static>(
        &self,
    ) -> Result<(Option<A>, Option<B>), Error> {
        let a = self.get_datum::<A>(1)?;
        let b = self.get_datum::<B>(2)?;
        Ok((a, b))
    }

    pub fn get_three<
        A: FromDatum + IntoDatum + 'static,
        B: FromDatum + IntoDatum + 'static,
        C: FromDatum + IntoDatum + 'static,
    >(
        &self,
    ) -> Result<(Option<A>, Option<B>, Option<C>), Error> {
        let a = self.get_datum::<A>(1)?;
        let b = self.get_datum::<B>(2)?;
        let c = self.get_datum::<C>(3)?;
        Ok((a, b, c))
    }

    pub fn get_heap_tuple(&self) -> Option<SpiHeapTupleData> {
        if self.current < 0 {
            panic!("SpiTupleTable positioned before start")
        }
        if self.current as u64 >= unsafe { pg_sys::SPI_processed } {
            None
        } else {
            match self.tupdesc {
                Some(tupdesc) => unsafe {
                    let heap_tuple = std::slice::from_raw_parts((*self.table).vals, self.size)
                        [self.current as usize];

                    // SAFETY:  we know heap_tuple is valid because we just made it
                    Some(SpiHeapTupleData::new(tupdesc, heap_tuple))
                },
                None => panic!("TupDesc is NULL"),
            }
        }
    }

    pub fn get_datum<T: FromDatum + IntoDatum + 'static>(
        &self,
        ordinal: i32,
    ) -> Result<Option<T>, Error> {
        if self.current < 0 {
            return Err(Error::InvalidPosition);
        }
        if self.current as u64 >= unsafe { pg_sys::SPI_processed } {
            Ok(None)
        } else {
            match self.tupdesc {
                Some(tupdesc) => unsafe {
                    let natts = (*tupdesc).natts;

                    if ordinal < 1 || ordinal > natts {
                        Ok(None)
                    } else {
                        let heap_tuple = std::slice::from_raw_parts((*self.table).vals, self.size)
                            [self.current as usize];
                        let mut is_null = false;
                        let datum =
                            pg_sys::SPI_getbinval(heap_tuple, tupdesc, ordinal, &mut is_null);

                        Ok(T::try_from_datum_in_memory_context(
                            PgMemoryContexts::CurrentMemoryContext
                                .parent()
                                .expect("parent memory context is absent"),
                            datum,
                            is_null,
                            pg_sys::SPI_gettypeid(tupdesc, ordinal),
                        )?)
                    }
                },
                None => Err(Error::TupDescIsNull),
            }
        }
    }

    /// Returns the number of columns
    pub fn columns(&self) -> usize {
        match self.tupdesc {
            Some(tupdesc) => unsafe { (*tupdesc).natts as usize },
            None => 0,
        }
    }

    /// Returns column type OID
    ///
    /// The ordinal position is 1-based
    pub fn column_type_oid(&self, ordinal: usize) -> Option<PgOid> {
        match self.tupdesc {
            Some(tupdesc) => unsafe {
                let nattrs = (*tupdesc).natts;
                if ordinal < 1 || ordinal > (nattrs as usize) {
                    None
                } else {
                    let oid = pg_sys::SPI_gettypeid(tupdesc, ordinal as i32);
                    Some(PgOid::from(oid))
                }
            },
            None => None,
        }
    }

    /// Returns column name
    ///
    /// The ordinal position is 1-based
    pub fn column_name(&self, ordinal: usize) -> Option<String> {
        match self.tupdesc {
            Some(tupdesc) => unsafe {
                let nattrs = (*tupdesc).natts;
                if ordinal < 1 || ordinal > (nattrs as usize) {
                    None
                } else {
                    let name = pg_sys::SPI_fname(tupdesc, ordinal as i32);
                    let str = CStr::from_ptr(name).to_string_lossy().into_owned();
                    pg_sys::pfree(name as *mut _);
                    Some(str)
                }
            },
            None => None,
        }
    }
}

impl SpiHeapTupleData {
    /// Create a new `SpiHeapTupleData` from its constituent parts
    pub unsafe fn new(tupdesc: pg_sys::TupleDesc, htup: *mut pg_sys::HeapTupleData) -> Self {
        let mut data = SpiHeapTupleData { tupdesc, entries: HashMap::default() };

        for i in 1..=tupdesc.as_ref().unwrap().natts {
            let mut is_null = false;
            let datum = pg_sys::SPI_getbinval(htup, tupdesc, i, &mut is_null);

            data.entries.entry(i as usize).or_insert_with(|| SpiHeapTupleDataEntry {
                datum: if is_null { None } else { Some(datum) },
                type_oid: pg_sys::SPI_gettypeid(tupdesc, i),
            });
        }

        data
    }

    /// Get a typed Datum value from this HeapTuple by its ordinal position.  
    ///
    /// The ordinal position is 1-based
    #[deprecated(since = "0.1.6", note = "Please use the `by_ordinal` function instead")]
    pub fn get_datum<T: FromDatum>(&self, ordinal: usize) -> Option<T> {
        match self.entries.get(&ordinal) {
            Some(datum) => datum.value(),
            None => None,
        }
    }

    /// Get a typed Datum value from this HeapTuple by its ordinal position.  
    ///
    /// The ordinal position is 1-based.
    ///
    /// If the specified ordinal is out of bounds a `Err(SpiError::NoAttribute)` is returned
    pub fn by_ordinal(
        &self,
        ordinal: usize,
    ) -> std::result::Result<&SpiHeapTupleDataEntry, SpiError> {
        match self.entries.get(&ordinal) {
            Some(datum) => Ok(datum),
            None => Err(SpiError::NoAttribute),
        }
    }

    /// Get a typed Datum value from this HeapTuple by its field name.  
    ///
    /// If the specified name does not exist a `Err(SpiError::NoAttribute)` is returned
    pub fn by_name(&self, name: &str) -> std::result::Result<&SpiHeapTupleDataEntry, SpiError> {
        use crate::pg_sys::AsPgCStr;
        unsafe {
            let fnumber = pg_sys::SPI_fnumber(self.tupdesc, name.as_pg_cstr());
            if fnumber == pg_sys::SPI_ERROR_NOATTRIBUTE {
                Err(SpiError::NoAttribute)
            } else {
                self.by_ordinal(fnumber as usize)
            }
        }
    }

    /// Get a mutable typed Datum value from this HeapTuple by its ordinal position.  
    ///
    /// The ordinal position is 1-based.
    ///
    /// If the specified ordinal is out of bounds a `Err(SpiError::NoAttribute)` is returned
    pub fn by_ordinal_mut(
        &mut self,
        ordinal: usize,
    ) -> std::result::Result<&mut SpiHeapTupleDataEntry, SpiError> {
        match self.entries.get_mut(&ordinal) {
            Some(datum) => Ok(datum),
            None => Err(SpiError::NoAttribute),
        }
    }

    /// Get a mutable typed Datum value from this HeapTuple by its field name.  
    ///
    /// If the specified name does not exist a `Err(SpiError::NoAttribute)` is returned
    pub fn by_name_mut(
        &mut self,
        name: &str,
    ) -> std::result::Result<&mut SpiHeapTupleDataEntry, SpiError> {
        use crate::pg_sys::AsPgCStr;
        unsafe {
            let fnumber = pg_sys::SPI_fnumber(self.tupdesc, name.as_pg_cstr());
            if fnumber == pg_sys::SPI_ERROR_NOATTRIBUTE {
                Err(SpiError::NoAttribute)
            } else {
                self.by_ordinal_mut(fnumber as usize)
            }
        }
    }

    /// Set a datum value for the specified ordinal position
    ///
    /// If the specified ordinal is out of bounds a `Err(SpiError::NoAttribute)` is returned
    pub fn set_by_ordinal<T: IntoDatum + FromDatum>(
        &mut self,
        ordinal: usize,
        datum: T,
    ) -> std::result::Result<(), SpiError> {
        unsafe {
            if ordinal < 1 || ordinal > self.tupdesc.as_ref().unwrap().natts as usize {
                Err(SpiError::NoAttribute)
            } else {
                self.entries.insert(
                    ordinal,
                    SpiHeapTupleDataEntry { datum: datum.into_datum(), type_oid: T::type_oid() },
                );
                Ok(())
            }
        }
    }

    /// Set a datum value for the specified field name
    ///
    /// If the specified name does not exist a `Err(SpiError::NoAttribute)` is returned
    pub fn set_by_name<T: IntoDatum + FromDatum>(
        &mut self,
        name: &str,
        datum: T,
    ) -> std::result::Result<(), SpiError> {
        use crate::pg_sys::AsPgCStr;
        unsafe {
            let fnumber = pg_sys::SPI_fnumber(self.tupdesc, name.as_pg_cstr());
            if fnumber == pg_sys::SPI_ERROR_NOATTRIBUTE {
                Err(SpiError::NoAttribute)
            } else {
                self.set_by_ordinal(fnumber as usize, datum)
            }
        }
    }
}

impl<Datum: IntoDatum + FromDatum> From<Datum> for SpiHeapTupleDataEntry {
    fn from(datum: Datum) -> Self {
        SpiHeapTupleDataEntry { datum: datum.into_datum(), type_oid: Datum::type_oid() }
    }
}

impl SpiHeapTupleDataEntry {
    pub fn value<T: FromDatum>(&self) -> Option<T> {
        match self.datum.as_ref() {
            Some(datum) => unsafe { T::from_polymorphic_datum(*datum, false, self.type_oid) },
            None => None,
        }
    }

    pub fn oid(&self) -> pg_sys::Oid {
        self.type_oid
    }
}

/// Provide ordinal indexing into a `SpiHeapTupleData`.
///
/// If the index is out of bounds, it will panic
impl Index<usize> for SpiHeapTupleData {
    type Output = SpiHeapTupleDataEntry;

    fn index(&self, index: usize) -> &Self::Output {
        self.by_ordinal(index).expect("invalid ordinal value")
    }
}

/// Provide named indexing into a `SpiHeapTupleData`.  
///
/// If the field name doesn't exist, it will panic
impl Index<&str> for SpiHeapTupleData {
    type Output = SpiHeapTupleDataEntry;

    fn index(&self, index: &str) -> &Self::Output {
        self.by_name(index).expect("invalid field name")
    }
}

/// Provide mutable ordinal indexing into a `SpiHeapTupleData`.  
///
/// If the index is out of bounds, it will panic
impl IndexMut<usize> for SpiHeapTupleData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.by_ordinal_mut(index).expect("invalid ordinal value")
    }
}

/// Provide mutable named indexing into a `SpiHeapTupleData`.  
///
/// If the field name doesn't exist, it will panic
impl IndexMut<&str> for SpiHeapTupleData {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.by_name_mut(index).expect("invalid field name")
    }
}

impl Iterator for SpiTupleTable {
    type Item = SpiHeapTupleData;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.size as isize {
            self.current = -1;
            None
        } else {
            self.current += 1;
            assert!(self.current >= 0);
            self.get_heap_tuple()
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.size))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.size
    }

    // Removed this function as it comes with an iterator
    //fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
}
