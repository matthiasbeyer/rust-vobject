
error_chain! {

    types {
        VObjectError, VObjectErrorKind, ResultExt, Result;
    }

    foreign_links {
        ChronoParseError(::chrono::format::ParseError) #[cfg(feature = "timeconversions")];
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

        NotAnIcalendar(content: String) {
            description("Input is not a valid ICalendar")
            display("Not an Icalendar: '{}'", content)
        }
    }


}
