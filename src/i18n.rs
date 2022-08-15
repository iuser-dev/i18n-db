use napi::bindgen_prelude::{Result, Uint8Array};
use napi_derive::napi;
use xxhash_rust::xxh3::xxh3_128;

use crate::db::Db;

pub fn hashkey(to: u8, bytes: impl AsRef<[u8]>) -> Vec<u8> {
  let mut r = Vec::with_capacity(17);
  r.push(to);
  let bytes = bytes.as_ref();
  let len = bytes.len() as u32;
  if len <= 20 {
    r.extend(bytes);
  } else {
    let hash = xxh3_128(&bytes);
    r.extend(hash.to_le_bytes());
    r.extend(len.to_le_bytes());
  }
  r
}

macro_rules! ok {
  ($code:block) => {
    Ok((|| Ok::<_, anyhow::Error>($code?))()?)
  };
}

#[napi]
impl Db {
  #[napi]
  pub fn trans_set(&self, to: u8, src: String, txt: String) -> Result<()> {
    ok!({
      let db = &self.db;
      let cf = &self.cf;
      let key = hashkey(to, &src);
      db.put_cf(&cf.trans, &key, &txt)
    })
  }

  #[napi]
  pub fn trans(&self, to: u8, src: String) -> Result<Option<Uint8Array>> {
    let db = &self.db;
    let cf = &self.cf;
    let key = hashkey(to, &src);
    Ok(db.get_cf(&cf.trans, &key).unwrap().map(Uint8Array::new))
  }
  /*
  #[napi]
  pub async fn file_is_change<T: Fn(String) -> Result<Promise<()>>>(
  &self,
  dir: String,
  path: String,
  callback: T,
  ) -> Result<()> {
  Ok(
  (async || {
  const OK: anyhow::Result<()> = Ok(());

  let dir = PathBuf::from(dir);
  let mut fp = dir.clone();
  fp.push(&path);

  let db = &self.db;
  let cf = &self.cf;

  if let Ok(meta) = fs::metadata(&fp) {
  if meta.is_file() {
  let len = &meta.len().to_le_bytes()[..];
  let mtime = meta.modified();
  if let Ok(time) = mtime {
  let mtime = &time
  .duration_since(SystemTime::UNIX_EPOCH)?
  .as_nanos()
  .to_le_bytes()[..];

  if let Some(pre) = db.get_cf(&cf.len, &path)? {
  if len == pre {
  if let Some(pre) = db.get_cf(&cf.mtime, &path)? {
  if mtime == pre {
  return OK;
  }
  }
  }
  }

  if let Ok(txt) = fs::read_to_string(&fp) {
  let hash = &hash128(&txt).to_le_bytes()[..];
  if let Some(pre) = db.get_cf(&cf.hash, &path)? {
  if hash == pre {
  return OK;
  }
  }

  callback(txt)?.await?;
  let tx = db.transaction();
  tx.put_cf(&cf.len, &path, len)?;
  tx.put_cf(&cf.hash, &path, hash)?;
  tx.put_cf(&cf.mtime, &path, mtime)?;
  tx.commit()?;
  }
  }
  }
  }

  OK
  })()
  .await?,
  )
  }
  */
}

/*
pub struct HashWriter<T: Hasher>(pub T);

impl<T: Hasher> Write for HashWriter<T> {
fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
self.0.write(buf);
Ok(buf.len())
}

fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
self.write(buf).map(|_| ())
}

fn flush(&mut self) -> io::Result<()> {
Ok(())
}
}
*/
