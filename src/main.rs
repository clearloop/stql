use stql::StQLDB;
use stql::StSled as DB;

fn main() {
    let s = DB::new("/usr/local/etc/stql");
    let _ = s.set(vec![0], vec![1]);
    let r = s.get(vec![0]);
    println!("{:?}", r);

    // let foo = r#"(foo "this is foo")";
    // let db = Db::start_default(db_path).unwrap();
    // db.set(&[0], vec![0]);
    // println!("{:?}", db.get(&[0]));
}
