#![feature(async_closure)]
#![feature(macro_metavar_expr)]

mod cf;
mod db;
mod open;
mod tx_cf;

mod i18n;

column_family!(mtime, len, hash, trans);

/*
#[neon::main]
pub fn main(cx: ModuleContext) -> NeonResult<()> {
  js::main(cx)
}

impl User {
  pub fn full_name(&self) -> String {
    let s = self.first_name.clone() + &self.last_name;
    s
  }
}


fn user_full_name(mut cx: FunctionContext) -> JsResult<JsString> {
  let user = cx.argument::<JsBox<User>>(0)?;
  let full_name = user.full_name();
  Ok(cx.string(full_name))
}
*/

/*




*/

/*
   impl Db {
   pub fn file_changed
   }
#[derive(Debug)]
pub struct PkAddr<Addr: FromBytes<Addr>> {
pub pk: Box<[u8]>,
pub addr: Addr,
}

const KAD_LEN: usize = 32;
impl Db {
pub fn addr_range<Addr: FromBytes<Addr>>(
&self,
begin: u128,
end: u128,
) -> SmallVec<[PkAddr<Addr>; KAD_LEN]> {
let cf = &self.cf;
let _end = end.to_be_bytes();
let mut li = SmallVec::new();
for (pk, addr) in self.db.iterator_cf(
&cf.pk_addr,
IteratorMode::From(&begin.to_be_bytes(), Direction::Forward),
) {
let key: [u8; 16] = pk[..16].try_into().unwrap();
if key > _end {
break;
}
li.push(PkAddr {
pk,
addr: Addr::from_bytes(&addr),
});
if li.len() >= KAD_LEN {
break;
}
}
li
}

pub fn addr_pk_set(&self, addr: &[u8], pk: &[u8]) -> Result<()> {
tx_cf!(self);

if let Some(pre) = get!(pk_addr, pk) {
if addr == pre {
return Ok(());
}
delete!(addr_pk, pre);
}

if let Some(pre) = get!(addr_pk, addr) {
delete!(pk_addr, pre);
}

put!(addr_pk, addr, pk);
put!(pk_addr, pk, addr);
Ok(())
}
}

static mut ADDR_TIME_ID: Lazy<Id> = Lazy::new(Id::default);
impl Db {
pub fn addr_sk_set(&self, addr: &[u8], sk: &[u8]) -> Result<()> {
tx_cf!(self);
if let Some(id) = get!(addr_sk, addr) {
delete!(addr_sk, id);
}
loop {
let id = unsafe { ADDR_TIME_ID.get() };
if get!(alive_addr, id).is_none() {
put!(alive_addr, id, addr);
break;
}
}
put!(addr_sk, addr, sk);
Ok(())
  }

pub fn addr_sk_decrypt_encrypt(
  &self,
  addr: &[u8],
  iv: &[u8],
  msg: &[u8],
  func: impl Fn(&[u8]) -> Box<[u8]>,
) -> Result<Option<Box<[u8]>>> {
  tx_cf!(self);
  if let Some(sk) = get_pinned!(addr_sk, addr) {
    if let Some(msg) = xxblake3::decrypt(&sk, iv, msg) {
      return Ok(Some(xxblake3::encrypt(&sk, iv, &func(&msg))));
    }
  };
  Ok(None)
}

pub fn addr_sk_decrypt(&self, addr: &[u8], iv: &[u8], msg: &[u8]) -> Result<Option<Box<[u8]>>> {
  tx_cf!(self);
  let r = if let Some(sk) = get_pinned!(addr_sk, addr) {
    xxblake3::decrypt(&sk, iv, msg)
  } else {
    None
  };
  Ok(r)
}

pub fn addr_sk_encrypt(&self, addr: &[u8], iv: &[u8], msg: &[u8]) -> Result<Option<Box<[u8]>>> {
  tx_cf!(self);
  let r = get_pinned!(addr_sk, addr).map(|sk| xxblake3::encrypt(&sk, iv, msg));
  Ok(r)
}
}
*/
