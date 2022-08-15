//use napi::bindgen_prelude::*;
use napi_derive::napi;
use rocksdb::OptimisticTransactionDB;

use crate::{cf_all, open::open, Cf};

#[napi]
pub struct Db {
  pub(crate) db: OptimisticTransactionDB,
  pub(crate) cf: Cf,
}

#[napi]
impl Db {
  #[napi(constructor)]
  pub fn new(path: String) -> Self {
    let mut db = Db {
      db: open(path).unwrap(),
      cf: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
    };
    let ptr: *const OptimisticTransactionDB = &db.db;
    db.cf = cf_all(unsafe { &*ptr });
    db
  }
  /*
  pub fn get_or_create<Ref: AsRef<[u8]>>(
  &self,
  key: impl AsRef<[u8]>,
  create: impl Fn() -> Ref,
  ) -> DBPinnableSlice<'_> {
  let db = &self.db;
  let key = key.as_ref();
  loop {
  if let Ok(Some(val)) = err::ok!(db.get_pinned(key)) {
  return val;
  }
  err::log!(db.put(key, create()));
  }
  }
  */
}

//unsafe impl Send for Db {}
//unsafe impl Sync for Db {}
