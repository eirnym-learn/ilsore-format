use std::env;
use std::io;
use std::path;

mod error;
mod structs;
use error::Error;
use structs::HeadOptions;


type Result<T, E = error::Error> = std::result::Result<T, E>;


fn to_short_hex_oid(oid: &[u8]) -> String {
    return oid[1..8]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("");
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn git_subfolder() -> io::Result<Option<path::PathBuf>> {
    let path = env::current_dir()?;
    for sub_path in path.ancestors() {
        let folder = sub_path.join(".git");
        if folder.exists() {
            return Ok(Some(sub_path.to_path_buf()));
        }
    }
    return Ok(None);
}

fn main() -> Result<()> {
    let p = path::Path::new("refs/heads/main");
    let name = p.file_name().unwrap();
    print_type_of(&name);
    println!("head: {:?}", p.file_name().unwrap());
    let _ = process_current_dir()?;
    Ok(())
}





fn process_current_dir() -> Result<(), Error> {
	 let git_dir_buf = git_subfolder()?.ok_or_else(|| Error::Message("Not found .git folder".to_string()))?;



    print_type_of(&git_dir_buf);
    println!(
        "Folder {:?} is repo: {:?}",
        git_dir_buf,
        git_dir_buf.exists()
    );
    process_repo(&git_dir_buf);
    Ok(())
}

fn head_info(repo: git2::Repository) -> Option<HeadOptions> {
    let head = match repo.head() {
        Err(_) => return None,
        Ok(head) => head,
    };

    println!("Head name is {:?}", head.shorthand());

    let is_detached = repo.head_detached().ok().unwrap_or_default();
    let oid = head.target();

    Some(HeadOptions {
        head_name: head.shorthand().map(|oid|oid.to_string()),
        head_oid: oid.map(|oid|oid.to_string()),
        head_detached: is_detached,
    })
}

fn process_repo(path: &path::PathBuf) {
    let repo = match git2::Repository::open(path) {
        Err(_) => return,
        Ok(repo) => repo,
    };

    let head = match repo.head() {
        Err(e) => {
            println!("not head, {:?}", e);
            return;
        }
        Ok(head) => head,
    };
    println!("Head name is {:?}", head.shorthand());
    let oid = match head.target() {
        None => return,
        Some(oid) => oid,
    };
    let short_hex_oid = &oid.to_string()[0..7];
    println!("Head OID: {:?}", &short_hex_oid)
}
