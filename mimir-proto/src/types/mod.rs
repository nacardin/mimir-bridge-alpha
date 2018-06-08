

mod identity;

pub use self::identity::Identity;


simple_unit!(
    Role, "basic client roles", 
    Oracle    => "oracle",
    Notary    => "notary",
    Requester => "requester",
    Router    => "router", 
    Verifier  => "verifier",
    Admin     => "admin",
);


