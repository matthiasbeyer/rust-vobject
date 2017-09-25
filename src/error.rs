
error_chain! {

    types {
        VObjectError, VObjectErrorKind, ResultExt, Result;
    }

    errors {
        ParserError(desc: String) {
            description("Parser error")
            display("{}", desc)
        }

        NotAVCard(content: String) {
            description("Input is not a valid VCard")
            display("Not a VCard: '{}'", content)
        }

        VersionNotFound {
            description("Version cannot be found")
            display("Version cannot be found")
        }

        NameNotFound {
            description("Name cannot be found")
            display("Name cannot be found")
        }

    }


}
