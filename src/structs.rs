#[derive(Debug)]
pub struct StatusOptions {
    include_untracked: bool,
    include_ignored: bool,
    include_submodules: bool,
    include_unreadable: bool,
}

#[derive(Debug)]
pub struct HeadOptions {
    head_name: Option<String>,
    head_oid: Option<String>, // Short oid
    head_detached: bool,
}

#[derive(Debug)]
pub struct OutputStatus {
    untracked: bool,
    ignored: bool,
    unreadable: bool,
}

#[derive(Debug)]
pub struct OutputOptions {
    is_repo: bool,
    head_options: Option<HeadOptions>,
    output_status: OutputStatus,
}

impl HeadOptions {
    pub fn new2(head_name: Option<String>, head_oid: Option<String>, head_detached: bool) -> Self {
        HeadOptions {
            head_name: match head_name {
                None => None,
                Some(head_name) => Some(head_name.to_string()),
            },
            head_oid: match head_oid {
                None => None,
                Some(head_oid) => Some(head_oid.to_string()),
            },
            head_detached,
        }
    }
    pub fn head_name(&self) -> Option<&str> {
        return self.head_name.as_deref();
    }

    pub fn head_detached(&self) -> bool {
        self.head_detached
    }
}
