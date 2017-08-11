#![allow(unused_doc_comment)]

/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

error_chain! {
    foreign_links {
        IO(::std::io::Error);
        Parse(::url::ParseError);
        URI(::hyper::error::UriError);
        Hyper(::hyper::Error);
    }
    errors {
        NetworkError {
            description("invalid checksum")
            display("Invalid checksum")
        }
        DatabasePoolError {
            description("no database connection available")
            display("No database connection available")
        }
        InvalidRegistrationKey {
            description("invalid registration key")
            display("Invalid registration key")
        }
        RegistrationFailed {
            description("registration failed")
            display("Registration failed")
        }
        LoginFailed {
            description("login failed")
            display("Login failed")
        }
        SaveFailed {
            description("save failed")
            display("Save failed")
        }
        DeleteFailed {
            description("delete failed")
            display("Delete failed")
        }
        UploadFailed {
            description("upload failed")
            display("Upload failed")
        }
        FileNotFound {
            description("file not found")
            display("File not found")
        }
    }
}
